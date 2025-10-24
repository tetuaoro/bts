use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

pub struct Backtest {
    data: VecDeque<Candle>,
    current_index: usize,
}

impl Backtest {
    pub fn new(data: Vec<Candle>) -> Self {
        Self {
            data: VecDeque::from(data),
            current_index: 0,
        }
    }

    pub fn current_candle(&self) -> Option<&Candle> {
        self.data.get(self.current_index)
    }

    pub fn next(&mut self) -> Option<&Candle> {
        if self.current_index < self.data.len() - 1 {
            self.current_index += 1;
            self.current_candle()
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    pub fn iter(&mut self) -> BacktestIterator<'_> {
        BacktestIterator { backtest: self }
    }
}

pub struct BacktestIterator<'a> {
    backtest: &'a mut Backtest,
}

impl<'a> Iterator for BacktestIterator<'a> {
    type Item = Candle;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(candle) = self.backtest.next() {
            Some(candle.clone())
        } else {
            None
        }
    }
}
