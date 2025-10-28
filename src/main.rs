mod engine;
mod utils;

use crate::engine::*;
use crate::utils::*;

use anyhow::*;
use ta::Next;
use ta::indicators::SimpleMovingAverage;

fn main() -> Result<()> {
    let items = get_data_from_file("data/btc.json".into())?;
    let candles = items
        .iter()
        .map(|d| Candle::from((d.open(), d.high(), d.low(), d.close(), d.volume(), d.bid())))
        .collect::<Vec<_>>();

    let initial_balance = 1_000.0;
    let mut bt = Backtest::new(candles.clone(), initial_balance);

    let mut sma = SimpleMovingAverage::new(5)?;

    while let Some(candle) = bt.next() {
        let close = candle.close();
        let output = sma.next(close);
        let long_limit = output - (output * 5.0 / 100.0);
        let high = candle.high();
        let low = candle.low();

        if low < long_limit {
            let quantity = (99.0 * bt.balance() / 100.0) / long_limit;
            _ = bt.open_position((PositionSide::Long, long_limit, quantity).into());
        }

        if output < high && output > low {
            _ = bt.close_position(output);
        }
    }

    let f = candles.first().unwrap();
    let l = candles.last().unwrap();
    let buy_and_hold = 100.0 * (initial_balance * l.close() / f.close()) / initial_balance;
    let new_balance = bt.balance();
    let performance = 100.0 * new_balance / initial_balance;
    let performance = if performance < 100.0 {
        -(100.0 - performance)
    } else {
        performance - 100.0
    };
    let count_position = bt.position_history().len();
    println!(
        "new balance {new_balance} USD\ntrades {count_position}\nperformance {performance:.3}%\nbuy and hold {buy_and_hold:.3}%"
    );

    Ok(())
}
