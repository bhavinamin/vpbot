use anyhow::Result;

use crate::cli::ReplayArgs;
use crate::market_event::MarketEvent;

//ReplayArgs is borrowed
pub fn load_events(args: &ReplayArgs) -> Result<Vec<MarketEvent>> {
    let mut reader = csv::Reader::from_path(&args.file)?;
    let mut events = Vec::new();

    for row in reader.deserialize() {
        let event: MarketEvent = row?;

        if event.symbol == args.symbol {
            events.push(event);
        }
    }

    Ok(events) //return post-filtered events 
}
