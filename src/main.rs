mod account;
mod transactions;
mod types;

use account::Account;
use csv::{Error, Reader, ReaderBuilder, Trim};
use std::{collections::HashMap, env, fs::File};
use transactions::Transaction;

fn process_transactions(
    mut reader: Reader<File>,
    accounts: &mut HashMap<u16, Account>,
) -> Result<(), Error> {
    for result in reader.deserialize() {
        let transaction: Transaction = result?;
        println!("{:?}", transaction);
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
fn main() {
    let mut accounts: HashMap<u16, Account> = HashMap::new();
    let reader = create_csv_reader();
    process_transactions(reader, &mut accounts)
        .expect("Something went wrong while processing transactions");
}
