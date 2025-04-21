#[derive(Debug, PartialEq, Eq)]
pub enum AccountingError {
    AccountNotFound(String),
    AccountUnderFunded(String, u64),
    AccountOverFunded(String, u64),
}
