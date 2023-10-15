use std::collections::HashMap;

use crate::account::Account;
use crate::types::{AccountProcesserError, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub transaction_id: u32,
    pub amount: Option<f32>,
}

pub fn process_transaction(
    accounts: &mut HashMap<u16, Account>,
    transactions_ledger: &mut HashMap<u32, Transaction>,
    transaction: Transaction,
) -> Result<()> {
    transactions_ledger.insert(transaction.transaction_id, transaction.clone());
    // We lazily create accounts currently as we have no idea of knowing what accounts exist before seeing them in the transaction file
    // In the future we could have a separate process that creates accounts first either my doing an extra full parse of the csv or other input type
    let account = accounts
        .entry(transaction.client_id)
        .or_insert_with(|| Account::new(transaction.client_id));

    match transaction.transaction_type {
        TransactionType::Deposit => {
            let amount = transaction
                .amount
                .ok_or(AccountProcesserError::InvalidTransaction(
                    "Deposit transaction must have an amount".to_string(),
                ))?;
            account.deposit(amount)?;
        }
        TransactionType::Withdrawal => {
            let amount = transaction
                .amount
                .ok_or(AccountProcesserError::InvalidTransaction(
                    "Withdrawal transaction must have an amount".to_string(),
                ))?;
            account.withdrawl(amount)?;
        }
        _ => unimplemented!(),
    };

    Ok(())
}
