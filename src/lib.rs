pub mod engine;
pub mod errors;
pub mod utils;

pub mod prelude {
    pub use super::*;
    pub use crate::engine::*;
    pub use crate::errors::*;
    pub use crate::utils::*;
}

use std::ops::{Add, Div, Mul, Sub};

pub trait PercentCalculus<Rhs = Self> {
    fn addpercent(self, rhs: Rhs) -> Self;
    fn subpercent(self, rhs: Rhs) -> Self;
    fn how_many(self, percent: Self) -> Self;
}

impl PercentCalculus for f64 {
    fn addpercent(self, percent: Self) -> Self {
        self.add(self.mul(percent.div(100.0)))
    }

    fn subpercent(self, percent: Self) -> Self {
        self.sub(self.mul(percent.div(100.0)))
    }

    fn how_many(self, percent: Self) -> Self {
        percent.mul(self.div(100.0))
    }
}

pub trait TruncCalculus {
    fn trunc_at(self, at: u32) -> Self;
}

impl TruncCalculus for f64 {
    fn trunc_at(self, at: u32) -> Self {
        let factor = 10.0_f64.powi(at as i32);
        (self * factor).trunc() / factor
    }
}
