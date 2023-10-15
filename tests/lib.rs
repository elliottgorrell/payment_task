use payment_task::transactions::{Transaction, TransactionType};
use payment_task::PaymentEngine;

#[test]
fn deposits_and_withdrawls_work() {
    let mut payment_engine = PaymentEngine::new();
    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 1,
            amount: Some(1.0),
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 2,
            transaction_id: 2,
            amount: Some(2.0),
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 3,
            transaction_id: 3,
            amount: Some(3.0),
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            client_id: 3,
            transaction_id: 4,
            amount: Some(0.0001),
        },
    ];

    for tx in transactions {
        let _ = payment_engine.process_transaction(tx);
    }
    let account_1 = payment_engine.get_accounts().get(&1).unwrap();
    let account_2 = payment_engine.get_accounts().get(&2).unwrap();
    let account_3 = payment_engine.get_accounts().get(&3).unwrap();
    assert_eq!(account_1.get_available(), 1.0);
    assert_eq!(account_2.get_available(), 2.0);
    assert_eq!(account_3.get_available(), 2.9999);
}

#[test]
fn cant_withdraw_more_than_available() {
    let mut payment_engine = PaymentEngine::new();
    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 1,
            amount: Some(1.0),
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            client_id: 1,
            transaction_id: 2,
            amount: Some(2.0),
        },
    ];

    let _ = payment_engine.process_transaction(transactions[0].clone());
    let result_2 = payment_engine.process_transaction(transactions[1].clone());

    let account = payment_engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.get_available(), 1.0);
    assert!(result_2.is_err());
}

#[test]
fn cant_withdraw_while_dispute_in_progress() {
    let mut payment_engine = PaymentEngine::new();
    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 1,
            amount: Some(5.0),
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 2,
            amount: Some(1.0),
        },
        Transaction {
            transaction_type: TransactionType::Dispute,
            client_id: 1,
            transaction_id: 1,
            amount: None,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            client_id: 1,
            transaction_id: 3,
            amount: Some(3.0),
        },
    ];

    for tx in transactions {
        let _ = payment_engine.process_transaction(tx);
    }

    let account = payment_engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.get_available(), 1.0);
    assert_eq!(account.get_held(), 5.0);
    assert_eq!(account.get_total(), 6.0);
}

#[test]
fn once_chargeback_all_future_transactions_are_ignored() {
    let mut payment_engine = PaymentEngine::new();
    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 1,
            amount: Some(1.0),
        },
        Transaction {
            transaction_type: TransactionType::Dispute,
            client_id: 1,
            transaction_id: 1,
            amount: None,
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 2,
            amount: Some(2.5),
        },
        Transaction {
            transaction_type: TransactionType::Chargeback,
            client_id: 1,
            transaction_id: 1,
            amount: None,
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 3,
            amount: Some(5.0),
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 4,
            amount: Some(3.0),
        },
    ];

    for tx in transactions {
        let _ = payment_engine.process_transaction(tx);
    }
    let account = payment_engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.get_available(), 2.5);
}
