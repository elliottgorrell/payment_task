use crate::types::{AccountProcesserError, Result};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Account {
    client_id: u16,
    /// The total funds that are available for trading, staking, withdrawal, etc. This should be equal to the total - held amounts
    available: f32,
    /// The total funds that are held for dispute. This should be equal to total - available amounts
    held: f32,
    /// The total funds that are available or held. This should be equal to available + held
    total: f32,
    /// Whether the account is locked. An account is locked if a charge back occurs
    locked: bool,
}

impl Account {
    pub fn new(client_id: u16) -> Self {
        Account {
            client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn deposit(&mut self, amount: f32) -> Result<()> {
        if amount < 0.0 {
            return Err(AccountProcesserError::InvalidTransaction(
                "Cannot deposit a negative amount".to_string(),
            ));
        }
        self.available += amount;
        self.total += amount;
        Ok(())
    }

    pub fn withdrawl(&mut self, amount: f32) -> Result<()> {
        if amount < 0.0 {
            return Err(AccountProcesserError::InvalidTransaction(
                "Cannot withdraw a negative amount".to_string(),
            ));
        }
        if amount > self.available {
            return Err(AccountProcesserError::InvalidTransaction(
                "Cannot withdraw more than available".to_string(),
            ));
        }
        self.available -= amount;
        self.total -= amount;
        Ok(())
    }
}
