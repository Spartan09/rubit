use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum AccountingError {
    AccountNotFound(String),
    AccountUnderFunded(String, u64),
    AccountOverFunded(String, u64),
}

#[derive(Debug)]
pub enum Tx {
    Deposit { account: String, amount: u64 },
    Withdraw { account: String, amount: u64 },
}

#[derive(Debug)]
struct Accounts {
    accounts: HashMap<String, u64>,
}

impl Accounts {
    pub fn new() -> Self {
        Accounts {
            accounts: Default::default(),
        }
    }

    pub fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
        let account = self
            .accounts
            .entry(signer.to_string())
            .or_insert(u64::default());
        let new_balance = account
            .checked_add(amount)
            .ok_or_else(|| AccountingError::AccountOverFunded(signer.to_string(), amount))?;
        *account = new_balance;
        Ok(Tx::Deposit {
            account: signer.to_string(),
            amount,
        })
    }

    pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
        let account = self
            .accounts
            .get_mut(signer)
            .ok_or(AccountingError::AccountNotFound(signer.to_owned()))?;
        let new_balance = account
            .checked_sub(amount)
            .ok_or_else(|| AccountingError::AccountUnderFunded(signer.to_string(), amount))?;
        *account = new_balance;
        Ok(Tx::Withdraw {
            account: signer.to_owned(),
            amount,
        })
    }

    pub fn send(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<(Tx, Tx), AccountingError> {
        let withdraw_tx = self.withdraw(sender, amount)?;
        let deposit_tx = self.deposit(recipient, amount)?;
        Ok((withdraw_tx, deposit_tx))
    }
}

fn main() {}
