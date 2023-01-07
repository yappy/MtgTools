use std::{fs::File, io::BufReader};

use crate::{api, common};
use anyhow::Result;

pub fn entry() -> Result<()> {
    let cards: Vec<api::Card> = {
        let file = File::open(common::PATH_CARDS)?;
        let file = BufReader::new(file);

        serde_json::from_reader(file)?
    };

    println!("Loaded: {} cards", cards.len());

    Ok(())
}
