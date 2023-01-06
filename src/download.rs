use anyhow::Result;
use serde::{Deserialize, Serialize};

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

/// <https://scryfall.com/docs/api/bulk-data>
#[derive(Serialize, Deserialize, Debug)]
struct Bulk {
    /// A unique ID for this bulk item.
    id: String,
    /// The Scryfall API URI for this file.
    uri: String,
    /// A computer-readable string for the kind of bulk item.
    #[serde(alias = "type")]
    type_: String,
    /// A human-readable name for this file.
    name: String,
    /// A human-readable description for this file.
    description: String,
    /// The URI that hosts this bulk file for fetching.
    download_uri: String,
    /// The time when this file was last updated.
    updated_at: String,
    /// The size of this file in integer bytes.
    size: u64,
    /// The MIME type of this file.
    content_type: String,
    /// The Content-Encoding encoding that will be used to transmit this file
    /// when you download it.
    content_encoding: String,
}

fn bulk_get(bulk_type: &str) -> Result<Bulk> {
    let url = format!("https://api.scryfall.com/bulk-data/{bulk_type}");
    let bulk = reqwest::blocking::get(url)?.json::<Bulk>()?;

    Ok(bulk)
}

pub fn entry() -> Result<()> {
    let bulk = bulk_get(BULK_TYPE_ORACLE)?;
    println!("{bulk:?}");

    Ok(())
}
