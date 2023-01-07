use std::{fs::File, io::BufReader};

use crate::{api, common};
use anyhow::Result;

fn load_sets() -> Result<Vec<api::Set>> {
    let file = File::open(common::PATH_SETS)?;
    let file = BufReader::new(file);

    Ok(serde_json::from_reader(file)?)
}

fn load_all_cards() -> Result<Vec<api::Card>> {
    let file = File::open(common::PATH_CARDS)?;
    let file = BufReader::new(file);

    Ok(serde_json::from_reader(file)?)
}

pub fn entry() -> Result<()> {
    let sets = load_sets()?;
    println!("Loaded: {} sets", sets.len());
    let cards = load_all_cards()?;
    println!("Loaded: {} cards", cards.len());

    Ok(())
}
