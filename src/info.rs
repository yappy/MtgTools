use std::{fs::File, io::BufReader};

use crate::{api, common};
use anyhow::Result;

pub fn entry() -> Result<()> {
    let info: api::Bulk = {
        let f = File::open(common::PATH_CARDS_INFO)?;
        let f = BufReader::new(f);

        serde_json::from_reader(f)?
    };

    println!("Update: {}", info.updated_at);
    println!();
    println!("{}", info.uri);

    Ok(())
}
