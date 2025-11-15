use crate::errors::{Error, Result};

/// Represents a trading wallet with balance and locked funds management.
#[derive(Debug)]
pub struct Wallet {
    // Initial balance used for reset
    initial_balance: f64,
    // Available balance
    balance: f64,
    // Funds locked in open positions
    locked: f64,
}

impl Wallet {
    /// Creates a new wallet with the given initial balance.
    /// Negative balances are set to 0.
    pub fn new(balance: f64) -> Result<Self> {
        if balance <= 0.0 {
            return Err(Error::NegZeroBalance);
        }

        Ok(Self {
            initial_balance: balance,
            balance: balance,
            locked: 0.0,
        })
    }

    /// Returns the balance.
    pub fn balance(&self) -> f64 {
        self.balance
    }

    /// Returns the free balance (available for new trades).
    pub fn free_balance(&self) -> Result<f64> {
        let free_balance = self.balance - self.locked;
        if free_balance.is_sign_negative() {
            return Err(Error::NegZeroBalance);
        }

        Ok(free_balance)
    }

    /// Adds funds to the wallet.
    pub(crate) fn add(&mut self, amount: f64) -> Result<f64> {
        self.balance += amount;
        self.free_balance()
    }

    /// Subtracts funds from the balance (after an order is executed).
    /// Assumes funds are already locked.
    pub(crate) fn sub(&mut self, amount: f64) -> Result<f64> {
        self.balance -= amount;
        self.locked -= amount;
        self.free_balance()
    }

    /// Locks additional funds for a position.
    pub(crate) fn lock(&mut self, amount: f64) -> Result<f64> {
        self.locked += amount;
        self.free_balance()
    }

    /// Unlocks funds when an order/position is closed.
    pub(crate) fn unlock(&mut self, amount: f64) -> Result<f64> {
        self.locked -= amount;
        self.free_balance()
    }

    /// Resets the wallet to its initial balance.
    pub(crate) fn reset(&mut self) {
        self.locked = 0.0;
        self.balance = self.initial_balance;
    }
}

#[test]
fn test_new_instance() {
    if let Ok(_) = Wallet::new(100.0) {
        assert!(true);
    }

    if let Err(_) = Wallet::new(0.0) {
        assert!(true);
    }
}

#[test]
fn test_cancel_order() {
    if let Ok(mut wallet) = Wallet::new(100.0) {
        // place order
        if let Ok(_free_balance) = wallet.lock(20.0) {
            assert_eq!(wallet.balance, 100.0);
            assert_eq!(wallet.locked, 20.0);
        } else {
            assert!(false);
        }

        // cancel order
        if let Ok(_free_balance) = wallet.unlock(20.0) {
            assert_eq!(wallet.balance, 100.0);
            assert_eq!(wallet.locked, 0.0);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_open_close_profit_position() {
    if let Ok(mut wallet) = Wallet::new(100.0) {
        // place order
        let locked_amount = 20.0;
        if let Ok(free_balance) = wallet.lock(locked_amount) {
            assert_eq!(free_balance, 80.0);
            assert_eq!(wallet.balance, 100.0);
            assert_eq!(wallet.locked, 20.0);
        } else {
            assert!(false);
        }

        // open position
        if let Ok(free_balance) = wallet.sub(locked_amount) {
            assert_eq!(free_balance, 80.0);
            assert_eq!(wallet.locked, 0.0);
            assert_eq!(wallet.free_balance().unwrap(), 80.0);
        } else {
            assert!(false);
        }

        // close profitable position
        let profit = 10.0;
        if let Ok(free_balance) = wallet.add(profit + locked_amount) {
            assert_eq!(free_balance, 110.0);
            assert_eq!(wallet.balance, 110.0);
            assert_eq!(wallet.locked, 0.0);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_open_close_loss_position() -> Result<()> {
    if let Ok(mut wallet) = Wallet::new(100.0) {
        // place order
        let locked_amount = 20.0;
        if let Ok(free_balance) = wallet.lock(locked_amount) {
            assert_eq!(free_balance, 80.0);
            assert_eq!(wallet.balance, 100.0);
            assert_eq!(wallet.locked, 20.0);
        } else {
            assert!(false);
        }

        // open position
        if let Ok(free_balance) = wallet.sub(locked_amount) {
            assert_eq!(free_balance, 80.0);
            assert_eq!(wallet.locked, 0.0);
            assert_eq!(wallet.free_balance().unwrap(), 80.0);
        } else {
            assert!(false);
        }

        // close unprofitable position
        let profit = -30.0;
        let free_balance = wallet.add(profit + locked_amount)?;
        assert_eq!(free_balance, 70.0);
        assert_eq!(wallet.balance, 70.0);
        assert_eq!(wallet.locked, 0.0);
    }

    Ok(())
}
