use crate::api::{self};
use anyhow::{anyhow, Result};
use std::{fs::File, io::Write, path::Path};

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

fn bulk_get(bulk_type: &str) -> Result<String> {
    let url = format!("https://api.scryfall.com/bulk-data/{bulk_type}");
    let resp = reqwest::blocking::get(url)?;

    Ok(resp.text()?)
}

fn download_bulk(url: &str, size: u64, dist: &Path) -> Result<()> {
    let mut outfile = File::create(dist)?;
    let mut resp = reqwest::blocking::get(url)?;
    let read_size = resp.copy_to(&mut outfile)?;
    assert_eq!(read_size, size);

    Ok(())
}

fn sets_get() -> Result<Vec<api::Set>> {
    let mut data = Vec::new();

    let mut url = "https://api.scryfall.com/sets".to_string();
    loop {
        let resp = reqwest::blocking::get(url)?;
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
    let sets = sets_get()?;
    println!("{} sets fetched", sets.len());
    {
        let dist = "./download/sets.json";
        let outfile = File::create(dist)?;
        serde_json::to_writer(outfile, &sets)?;
    }

    let bulk = bulk_get(BULK_TYPE_ORACLE)?;
    println!("bulk info fetched");
    {
        let dist = "./download/cards_info.json";
        let mut outfile = File::create(dist)?;
        outfile.write(bulk.as_bytes())?;
    }
    let bulk: api::Bulk = serde_json::from_str(&bulk)?;

    println!("download bulk ({} MiB)", bulk.size / 1024 / 1024);
    let dist = "./download/cards.json";
    download_bulk(&bulk.download_uri, bulk.size, &Path::new(dist))?;

    Ok(())
}
