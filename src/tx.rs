#[derive(Debug, PartialEq, Eq)]
pub enum Tx {
    Deposit { account: String, amount: u64 },
    Withdraw { account: String, amount: u64 },
}
