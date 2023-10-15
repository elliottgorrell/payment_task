use csv::{Error, Reader, ReaderBuilder, Trim};
use serde::Deserialize;
use std::{env, fs::File};

#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(rename = "type")]
    transaction_type: String,
    #[serde(rename = "client")]
    client_id: u16,
    #[serde(rename = "tx")]
    transaction_id: u32,
    amount: Option<f32>,
}

fn process_transactions(mut reader: Reader<File>) -> Result<(), Error> {
    println!("processing");
    for result in reader.deserialize() {
        let transaction: Transaction = result?;
        println!("{:?}", transaction);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).expect("Must provide the file path of transaction file to be processed. Example: cargo run -- transactions.csv");

    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(file_path)
        .expect("Unable to read CSV file provided");
    process_transactions(reader).expect("Something went wrong while processing transactions");
    println!("{}", file_path);
}
