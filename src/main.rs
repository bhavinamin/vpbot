mod cli;
mod market_event;
mod replay;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Replay(args) => {
            let events = replay::load_events(&args)?;
            println!("Loaded {} events for {}", events.len(), args.symbol);
            println!("First events:");

            for event in events.iter().take(5) {
                println!(
                    "    {} {} price={} size={}",
                    event.timestamp, event.symbol, event.price, event.size
                );
            }
        }
    }

    Ok(())
}
