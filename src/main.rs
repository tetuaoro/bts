mod engine;
mod plot;
mod utils;

use crate::engine::*;
use crate::utils::*;

use anyhow::*;
use ta::*;

fn main() -> Result<()> {
    let items = get_data_from_file("data/btc.json".into())?;

    // let candles = items
    //     .iter()
    //     .map(|d| Candle {
    //         open: d.open(),
    //         high: d.high(),
    //         low: d.low(),
    //         close: d.close(),
    //         volume: d.volume(),
    //     })
    //     .collect::<Vec<_>>();

    // let mut backtest = Backtest::new(candles);

    // backtest.iter().for_each(|d| println!("{d:?}"));

    Ok(())
}
