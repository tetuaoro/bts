use rand::Rng;
use ta::DataItem;

pub(crate) fn faker_candle(n: usize) -> Vec<DataItem> {
    let mut rng = rand::rng();
    let mut price = 100.0;
    let mut volume = 15120.0;
    let mut candles = Vec::with_capacity(n);
    let ratio = 5.0;
    for _ in 0..n {
        price += rng.random_range(-5.0..5.0);
        volume += rng.random_range(-2000.0..2000.0);

        let open = price + rng.random_range(price - 1.0..price + 1.0);
        let close = price + rng.random_range(price - 1.0..price + 1.0);
        #[allow(unused_assignments)]
        let mut high = 0.0;
        #[allow(unused_assignments)]
        let mut low = 0.0;

        if open < close {
            high = rng.random_range(close..close + ratio);
            low = rng.random_range(open - ratio..open);
        } else {
            high = rng.random_range(open..open + ratio);
            low = rng.random_range(close - ratio..close);
        }

        let item = DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()
            .unwrap();
        candles.push(item);
    }
    candles
}
