mod api;
mod common;
mod deck;
mod download;

use anyhow::Result;
use getopts::Options;
use std::env;

/// Parse args and dispatch.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help");
    opts.optflag("d", "download", "Download");
    opts.optflag("a", "arena-deck", "Build decks for Arena");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(fail) => {
            eprintln!("{fail}");
            std::process::exit(1);
        }
    };

    let print_help = || {
        let brief = format!("Usage: {program} [options]");
        print!("{}", opts.usage(&brief));
    };

    if matches.opt_present("h") {
        print_help();
        std::process::exit(0);
    }

    if matches.opt_present("d") {
        download::entry()?;
    } else if matches.opt_present("a") {
        deck::entry()?;
    } else {
        print_help();
        std::process::exit(1);
    }

    Ok(())
}
