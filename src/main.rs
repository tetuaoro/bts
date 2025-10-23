mod utils;

use utils::*;

fn main() {
    let candles = faker_candle(7);
    println!("{candles:#?}");
}
