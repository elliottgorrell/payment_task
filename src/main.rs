use payment_task::account::Account;
use payment_task::transactions::Transaction;
use payment_task::PaymentEngine;

use csv::{Reader, ReaderBuilder, Trim};
use log::{debug, error};
use std::{collections::HashMap, env, fs::File};

fn process_transactions(
    mut reader: Reader<File>,
    payment_engine: &mut PaymentEngine,
) -> Result<(), csv::Error> {
    for result in reader.deserialize() {
        let transaction: Transaction = result?;
        debug!("processing transaction: {:?}", transaction);
        let result = payment_engine.process_transaction(transaction);
        if result.is_err() {
            error!("Error processing transaction: {:?}", result);
        }
    }
    Ok(())
}

/**
 * Creates a CSV reader from the file path provided as the first argument to the program
 * If anything goes wrong we panic as nothing can be done to resolve this
 * The reader will be buffered and trim any whitespace from csv fields and headers
 */
fn create_csv_reader() -> Reader<File> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Must provide the file path of transaction file to be processed. Example: cargo run -- transactions.csv");
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(file_path)
        .expect("Unable to read CSV file provided");
    reader
}

fn output_results(accounts: &HashMap<u16, Account>) {
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    for account in accounts.values() {
        writer
            .serialize(account)
            .expect("Unable to write account to stdout");
    }
    writer.flush().expect("Unable to flush stdout");
}

fn main() {
    let mut payment_engine = PaymentEngine::new();

    let reader = create_csv_reader();
    process_transactions(reader, &mut payment_engine)
        .expect("Something went wrong while processing transactions");

    output_results(payment_engine.get_accounts());
}
