mod candle;
mod position;

#[cfg(test)]
mod bt;

pub use candle::*;
pub use position::*;

use crate::{
    PercentCalculus, TruncCalculus,
    errors::{Error, Result},
};

#[derive(Debug)]
pub struct Backtest {
    data: Vec<Candle>,
    positions: Vec<Position>,
    // used to reset balance
    _balance: f64,
    balance: f64,
    index: usize,
    position_history: Vec<PositionEvent>,
}

impl Backtest {
    pub fn new(data: Vec<Candle>, initial_balance: f64) -> Self {
        Self {
            data,
            index: 0,
            positions: Vec::new(),
            balance: initial_balance,
            _balance: initial_balance,
            position_history: Vec::new(),
        }
    }

    pub fn current_balance(&self) -> f64 {
        self.balance
    }

    pub fn total_balance(&self, current_price: f64) -> f64 {
        let positions_value: f64 = self
            .positions
            .iter()
            .map(|p| p.estimate_profit(current_price))
            .sum();
        self.balance + positions_value
    }

    pub fn positions(&self) -> Vec<Position> {
        self.positions.clone()
    }

    pub fn events(&self) -> &Vec<PositionEvent> {
        &self.position_history
    }

    pub fn open_position(&mut self, position: Position) -> Result<()> {
        let (side, price, quantity) =
            (position.side(), position.entry_price(), position.quantity());
        let cost = price * quantity;

        if self.balance < cost {
            return Err(Error::LessBalance(cost));
        }

        match side {
            PositionSide::Long => self.balance -= cost,
            PositionSide::Short => self.balance -= cost,
        }

        let mut position = position.to_owned();
        position.set_id(self.index as u32);

        let event = (position.id(), self.index, side.to_owned(), price);
        self.positions.push(position);
        self.position_history.push(event.into());

        Ok(())
    }

    pub fn close_position(&mut self, position_id: u32, exit_price: f64) -> Result<f64> {
        if let Some(idx) = self.positions.iter().position(|p| p.id() == position_id) {
            let position = self.positions.remove(idx);
            let value = match position.side() {
                PositionSide::Long => {
                    let value = exit_price * position.quantity();
                    self.balance += value;
                    value
                }
                PositionSide::Short => {
                    self.balance += position.cost();
                    let profit = (position.entry_price() - exit_price) * position.quantity();
                    self.balance += profit;
                    profit
                }
            };

            if let Some(event) = self
                .position_history
                .iter_mut()
                .find(|p| p.id() == position_id)
            {
                event.close(self.index, exit_price);
            }

            return Ok(value);
        }

        Err(Error::EmptyPosition)
    }

    pub fn close_all_positions(&mut self, exit_price: f64) -> Result<f64> {
        let positions = self.positions.clone();
        let value = positions
            .iter()
            .map(|position| {
                self.close_position(position.id(), exit_price)
                    .unwrap_or_default()
            })
            .sum();
        self.positions.clear();

        Ok(value)
    }

    pub fn execute_exit_rules(&self, exit_price: f64) -> Result<()> {
        let positions = self.positions.clone();
        positions.iter().for_each(|p| {
            let entry_price = p.entry_price();
            let _is_done = match p.exit_rule() {
                PositionExitRule::Limit(_type) | PositionExitRule::StopLoss(_type) => match _type {
                    PriceType::Usd(rule_price) => rule_price == &exit_price,
                    PriceType::Percent(rule_percent) => {
                        let rule_price = if p.side() == &PositionSide::Long {
                            entry_price.addpercent(*rule_percent)
                        } else {
                            entry_price.subpercent(*rule_percent)
                        };
                        rule_price.trunc_at(3) == exit_price.trunc_at(3)
                    }
                },
                PositionExitRule::TakeProfit(_) => todo!(),
                PositionExitRule::TrailingStop(_) => todo!(),
                PositionExitRule::TakeProfitAndStopLoss(_) => todo!(),
            };
        });

        Ok(())
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.positions = Vec::new();
        self.balance = self._balance;
        self.position_history = Vec::new();
    }
}

impl Iterator for Backtest {
    type Item = Candle;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.data.get(self.index).cloned();
        self.index += 1;
        item
    }
}
