pub mod account;
pub mod transactions;
pub mod error;

use account::Account;
use log::info;
use std::collections::HashMap;
use transactions::{Transaction, TransactionType};
use error::{AccountProcesserError, Result};

pub struct PaymentEngine {
    accounts: HashMap<u16, Account>,
    transaction_ledger: HashMap<u32, Transaction>,
}

impl Default for PaymentEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PaymentEngine {
    pub fn new() -> Self {
        PaymentEngine {
            accounts: HashMap::new(),
            transaction_ledger: HashMap::new(),
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<()> {
        // We lazily create accounts currently as we have no idea of knowing what accounts exist before seeing them in the transaction file
        // In the future we could have a separate process that creates accounts first either my doing an extra full parse of the csv or other input type
        let account = self
            .accounts
            .entry(transaction.client_id)
            .or_insert_with(|| Account::new(transaction.client_id));

        match transaction.transaction_type {
            TransactionType::Deposit => {
                self.transaction_ledger
                    .insert(transaction.transaction_id, transaction.clone());
                let amount =
                    transaction
                        .amount
                        .ok_or(AccountProcesserError::InvalidTransaction(
                            "Deposit transaction must have an amount".to_string(),
                        ))?;
                account.deposit(amount)?;
            }
            TransactionType::Withdrawal => {
                self.transaction_ledger
                    .insert(transaction.transaction_id, transaction.clone());
                let amount =
                    transaction
                        .amount
                        .ok_or(AccountProcesserError::InvalidTransaction(
                            "Withdrawal transaction must have an amount".to_string(),
                        ))?;
                account.withdrawl(amount)?;
            }
            TransactionType::Dispute => {
                let Some(disputed_transaction) =
                    self.transaction_ledger.get(&transaction.transaction_id)
                else {
                    info!(
                        "A dispute was lodged for transaction {} which doesn't exist. ignoring",
                        &transaction.transaction_id
                    );
                    return Ok(());
                };
                if transaction.client_id != disputed_transaction.client_id {
                    return Err(AccountProcesserError::InvalidTransaction(
                  "Dispute transaction must be for the same client as the disputed transaction"
                      .to_string(),
              ));
                }
                let amount = disputed_transaction.amount.ok_or(
                    AccountProcesserError::InvalidTransaction(
                        "Dispute transaction must have an amount".to_string(),
                    ),
                )?;
                account.dispute(amount, transaction.transaction_id)?;
            }
            TransactionType::Resolve => {
                // This is non-falliable (unless account is locked) as the task spec deems we just ignore all issues such as the dispute
                // not being open or the transaction not existing
                account.resolve(transaction.transaction_id)?;
            }
            TransactionType::Chargeback => {
                // This is non-falliable (unless account is locked)  as the task spec deems we just ignore all issues such as the dispute
                // not being open or the transaction not existing
                account.chargeback(transaction.transaction_id)?;
            }
        };

        Ok(())
    }

    pub fn get_accounts(&self) -> &HashMap<u16, Account> {
        &self.accounts
    }
}
