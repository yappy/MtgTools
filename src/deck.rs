use std::{
    cmp::Ordering,
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use crate::{api, common};
use anyhow::Result;

type Sets = Vec<api::Set>;
type Cards = Vec<api::Card>;

fn load_sets() -> Result<Sets> {
    let file = File::open(common::PATH_SETS)?;
    let file = BufReader::new(file);

    Ok(serde_json::from_reader(file)?)
}

fn load_all_cards() -> Result<Cards> {
    let file = File::open(common::PATH_CARDS)?;
    let file = BufReader::new(file);
    let mut cards: Vec<api::Card> = serde_json::from_reader(file)?;

    cards.sort_by(|a, b| {
        let o1 = a.name.cmp(&b.name);
        if o1 == Ordering::Equal {
            let mut cna = a.collector_number.clone();
            let mut cnb = b.collector_number.clone();
            while cna.len() < 5 {
                cna.insert_str(0, " ");
            }
            while cnb.len() < 5 {
                cnb.insert_str(0, " ");
            }
            cna.cmp(&cnb)
        } else {
            o1
        }
    });

    Ok(cards)
}

/// Return 1st-face name if multiple faced card
fn convert_name(orig: &str) -> &str {
    let idx = orig.find(" // ");
    if let Some(idx) = idx {
        &orig[0..idx]
    }else {
        orig
    }
}

fn create_one(cards: &Cards, set: &api::Set, rarity: &str, rname: &str) -> Result<()> {
    let cards = cards
        .iter()
        .filter(|card| card.set_id == set.id && card.rarity == rarity)
        .filter(|card| !(card.type_line.is_some() && card.type_line.as_ref().unwrap().starts_with("Basic Land")));

    {
        let path = format!(
            "{}/{}_{}_{}.txt",
            common::PATH_DECK_DIR,
            set.released_at.as_deref().unwrap_or(""),
            set.code.to_ascii_uppercase(),
            rname
        );
        let outfile = File::create(path)?;
        let mut outfile = BufWriter::new(outfile);

        let mut count = 0;
        writeln!(&mut outfile, "Deck")?;
        for card in cards {
            if count + 4 > 250 {
                writeln!(&mut outfile, "================================================================================")?;
                writeln!(&mut outfile, "Deck")?;
                count = 0;
            }
            writeln!(
                &mut outfile,
                "4 {} ({}) {}",
                convert_name(&card.name),
                set.code.to_ascii_uppercase(),
                card.collector_number
            )?;
            count += 4;
        }
    }

    Ok(())
}

fn create_all(sets: &Sets, cards: &Cards) -> Result<()> {
    // XLN 2017-09-29 or later
    const RELEASE_FILTER: &str = "2017-09-29";

    let sets = sets
        .iter()
        // type = expansion or digital masters only
        .filter(|set| (set.set_type == "expansion") || (set.set_type == "masters" && set.digital))
        // Ixalan or later only
        .filter(|set| {
            set.released_at
                .as_ref()
                .map_or(false, |rel| rel.as_str() >= RELEASE_FILTER)
        });

    let rarity_list = [
        ("common", "1C"),
        ("uncommon", "2U"),
        ("rare", "3R"),
        ("mythic", "4M"),
    ];

    for set in sets {
        for (rarity, rname) in rarity_list {
            create_one(cards, set, rarity, rname)?;
        }
    }

    Ok(())
}

pub fn entry() -> Result<()> {
    let sets = load_sets()?;
    println!("Loaded: {} sets", sets.len());
    let cards = load_all_cards()?;
    println!("Loaded: {} cards", cards.len());

    create_all(&sets, &cards)?;

    Ok(())
}
