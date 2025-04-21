use std::collections::HashMap;

use crate::{errors::AccountingError, tx::Tx};

#[derive(Debug)]
pub struct Accounts {
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

#[cfg(test)]
mod tests {
    use std::u64;

    use super::*;

    #[test]
    fn test_accounts_withdraw_underfunded() {
        let mut accounts = Accounts::new();
        accounts.deposit("a-key", 0).unwrap();
        let actual = accounts.withdraw("a-key", 100);
        assert_eq!(
            actual,
            Err(AccountingError::AccountUnderFunded(
                "a-key".to_string(),
                100
            ))
        );
    }

    #[test]
    fn test_accounts_deposited_overfunded() {
        let mut accounts = Accounts::new();
        accounts
            .deposit("a-key", 1)
            .expect("Initial deposit failed");
        let actual = accounts.deposit("a-key", u64::MAX);
        assert_eq!(
            actual,
            Err(AccountingError::AccountOverFunded(
                "a-key".to_string(),
                u64::MAX
            ))
        );
    }

    #[test]
    fn test_accounts_deposit_works() {
        let mut accounts = Accounts::new();
        let amt = 100;
        let actual = accounts.deposit("a-key", amt);
        assert_eq!(
            actual,
            Ok(Tx::Deposit {
                account: "a-key".to_string(),
                amount: amt
            })
        );
    }

    #[test]
    fn test_accounts_withdraw_works() {
        let mut accounts = Accounts::new();
        let amt = 100;
        accounts.deposit("a-key", amt).expect("Couldn't deposit");
        let actual = accounts.withdraw("a-key", amt);
        assert_eq!(
            actual,
            Ok(Tx::Withdraw {
                account: "a-key".to_string(),
                amount: amt
            })
        );
    }
    
    
}
