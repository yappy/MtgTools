use crate::{api, common};
use anyhow::{anyhow, bail, Result};
use reqwest::blocking::{Client, Response};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[allow(dead_code)]
const BULK_TYPE_ORACLE: &str = "oracle_cards";
#[allow(dead_code)]
const BULK_TYPE_ARTWORK: &str = "unique_artwork";
#[allow(dead_code)]
const BULK_TYPE_DEFAULT: &str = "default_cards";
#[allow(dead_code)]
const BULK_TYPE_ALL: &str = "all_cards";
#[allow(dead_code)]
const BULK_TYPE_RULINGS: &str = "rulings";

/// `User-Agent: <product> / <product-version> <comment>`
///
/// Example:
/// Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0
/// Mozilla/5.0 (Macintosh; Intel Mac OS X x.y; rv:42.0) Gecko/20100101 Firefox/42.0
const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_HOMEPAGE"),
    ")"
);

fn http_get(url: &str) -> Result<Response> {
    let client = Client::new();
    let resp = client
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::ACCEPT, "*/*")
        .send()?;
    Ok(resp)
}

fn bulk_get(bulk_type: &str) -> Result<String> {
    let url = format!("https://api.scryfall.com/bulk-data/{bulk_type}");
    let resp = http_get(&url)?;

    Ok(resp.text()?)
}

fn download_bulk(url: &str, size: u64, dist: &str) -> Result<()> {
    let outfile = File::create(dist)?;
    let mut outfile = BufWriter::new(outfile);
    let mut resp = http_get(url)?;
    let read_size = resp.copy_to(&mut outfile)?;
    assert_eq!(read_size, size);

    Ok(())
}

fn sets_get() -> Result<Vec<api::Set>> {
    let mut data = Vec::new();

    let mut url = "https://api.scryfall.com/sets".to_string();
    loop {
        let resp = http_get(&url)?;
        let st = resp.error_for_status_ref().map(|_| ());
        if let Err(err) = st {
            eprintln!("{}", resp.text()?);
            bail!(err);
        }
        let list = resp.json::<api::List<api::Set>>()?;

        data.extend(list.data);

        if list.has_more {
            url = list.next_page.ok_or_else(|| anyhow!("No next_page"))?;
        } else {
            break;
        }
    }

    Ok(data)
}

pub fn entry() -> Result<()> {
    println!("USER_AGENT: {USER_AGENT}");

    let sets = sets_get()?;
    println!("{} sets fetched", sets.len());
    {
        let outfile = File::create(common::PATH_SETS)?;
        let outfile = BufWriter::new(outfile);
        serde_json::to_writer(outfile, &sets)?;
    }

    let bulk = bulk_get(BULK_TYPE_DEFAULT)?;
    println!("bulk info fetched");
    {
        let outfile = File::create(common::PATH_CARDS_INFO)?;
        let mut outfile = BufWriter::new(outfile);
        outfile.write_all(bulk.as_bytes())?;
    }
    let bulk: api::Bulk = serde_json::from_str(&bulk)?;

    println!("download bulk ({} MiB)", bulk.size / 1024 / 1024);
    download_bulk(&bulk.download_uri, bulk.size, common::PATH_CARDS)?;

    Ok(())
}
