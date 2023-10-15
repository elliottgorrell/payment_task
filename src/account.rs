use std::collections::HashMap;

use crate::types::{AccountProcesserError, Result};

use log::warn;
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
    /// Currently open disputes on this account the key is the tx_id and the val the amount of dispute. This tx_id  can be looked up in global ledger
    #[serde(skip_serializing)]
    open_disputes: HashMap<u32, f32>,
}

impl Account {
    pub fn new(client_id: u16) -> Self {
        Account {
            client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
            open_disputes: HashMap::new(),
        }
    }

    fn check_lock(&self) -> Result<()> {
        if self.locked {
            return Err(AccountProcesserError::AccountLocked(self.client_id));
        }
        Ok(())
    }

    pub fn deposit(&mut self, amount: f32) -> Result<()> {
        self.check_lock()?;
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
        self.check_lock()?;
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

    pub fn dispute(&mut self, amount: f32, tx_id: u32) -> Result<()> {
        self.check_lock()?;
        // This could potentially put the account into a negative balance
        self.available -= amount;
        self.held += amount;
        self.open_disputes.insert(tx_id, amount);
        Ok(())
    }

    pub fn resolve(&mut self, tx_id: u32) -> Result<()> {
        self.check_lock()?;
        let Some(amount) = self.open_disputes.remove(&tx_id) else {
            warn!("A dispute which is not in progress was resolved and being ignored: {tx_id}");
            return Ok(());
        };
        self.available += amount;
        self.held -= amount;
        Ok(())
    }

    pub fn chargeback(&mut self, tx_id: u32) -> Result<()> {
        self.check_lock()?;
        let Some(amount) = self.open_disputes.remove(&tx_id) else {
            warn!("A dispute which is not in progress was charged back and being ignored: {tx_id}");
            return Ok(());
        };
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
        Ok(())
    }
}
