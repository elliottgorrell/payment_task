use thiserror::Error;

pub type Result<T> = std::result::Result<T, AccountProcesserError>;

#[derive(Error, Debug)]
pub enum AccountProcesserError {
    #[error("A transaction tried to do an invalid operation {0}")]
    InvalidTransaction(String),
}
