//! Strategy parameter optimization.
//!
//! This module provides tools to optimize trading strategies by testing different parameter combinations.
//! The `Optimizer` struct handles the execution of backtests for each combination, while the
//! `ParameterCombination` trait defines how to generate parameter sets.

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use crate::engine::{Backtest, Candle};
use crate::errors::{Error, Result};

use rayon::prelude::*;

/// Trait defining how to generate parameter combinations for optimization.
///
/// Implement this trait for your parameter types to define how combinations should be generated.
/// The associated type `P` represents a single parameter combination (e.g., a tuple of values).
pub trait ParameterCombination {
    /// Type representing a single parameter combination (e.g., `(usize, f64)`).
    type T: Clone + Sync;

    /// Generates all possible parameter combinations to test.
    ///
    /// # Returns
    /// A vector containing all parameter combinations.
    fn generate() -> Vec<Self::T>;
}

/// Optimizer for testing trading strategies with different parameter combinations.
///
/// This struct handles the execution of backtests for each parameter combination,
/// collecting results for analysis.
pub struct Optimizer<PS: ParameterCombination> {
    data: Vec<Candle>,
    initial_balance: f64,
    _marker: PhantomData<PS>,
    market_fees: Option<(f64, f64)>,
}

impl<PS: ParameterCombination> Optimizer<PS> {
    /// Creates a new `Optimizer` with the given data and initial balance.
    ///
    /// # Arguments
    /// * `data` - Historical candle data for backtesting.
    /// * `initial_balance` - Starting balance for the backtest.
    /// * `market_fees` - Optional tuple of (maker fee, taker fee).
    ///
    /// # Returns
    /// A new `Optimizer` instance.
    pub fn new(data: Vec<Candle>, initial_balance: f64, market_fees: Option<(f64, f64)>) -> Self {
        Self {
            data,
            market_fees,
            initial_balance,
            _marker: PhantomData,
        }
    }

    /// Optimizes a trading strategy by testing all parameter combinations.
    ///
    /// # Arguments
    /// * `transformers` - Function that converts a parameter combination into strategy-specific parameters.
    /// * `strategy` - Trading strategy function to test.
    ///
    /// # Returns
    /// A vector of tuples containing each parameter combination and its resulting balance.
    ///
    /// # Errors
    /// Returns an error if backtest execution fails.
    pub fn with<T, TR, S>(&self, transformers: TR, strategy: S) -> Result<Vec<(PS::T, f64)>>
    where
        T: Clone,
        PS: Sync,
        PS::T: Send,
        TR: Fn(&PS::T) -> Result<T> + Sync,
        S: FnMut(&mut Backtest, &mut T, &Candle) -> Result<()> + Send,
    {
        let num_cpus = num_cpus::get();
        let strategy = Arc::new(Mutex::new(strategy));
        let combinations = PS::generate();
        let chunk_size = ((combinations.len() + num_cpus - 1) / num_cpus).max(1);

        let chunk_results = combinations
            .par_chunks(chunk_size)
            .map::<_, Result<_>>(|par_combinations| {
                let mut backtest = Backtest::new(self.data.clone(), self.initial_balance, self.market_fees)?;
                let mut local_results = Vec::with_capacity(par_combinations.len());

                let strategy_arc = Arc::clone(&strategy);
                let mut strategy_guard = strategy_arc.lock().map_err(|e| Error::MutexPoisoned(e.to_string()))?;

                for param_set in par_combinations {
                    let mut transformer = transformers(param_set)?;
                    backtest.run(|bt, candle| strategy_guard(bt, &mut transformer, candle))?;
                    local_results.push((param_set.clone(), backtest.total_balance()));
                    backtest.reset();
                }

                Ok(local_results)
            })
            .collect::<Vec<_>>();

        let mut results = Vec::with_capacity(combinations.len());
        for chunk_result in chunk_results {
            let chunk = chunk_result?;
            results.extend(chunk);
        }

        Ok(results)
    }
}
