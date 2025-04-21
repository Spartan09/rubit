use std::collections::HashMap;

use crate::{errors::ApplicationError, tx::Tx};

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

    pub fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, ApplicationError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            account
                .checked_add(amount)
                .and_then(|r| {
                    *account = r;
                    Some(r)
                })
                .ok_or(ApplicationError::AccountOverFunded(
                    signer.to_string(),
                    amount,
                ))
                .map(|_| Tx::Deposit {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            self.accounts.insert(signer.to_string(), amount);
            Ok(Tx::Deposit {
                account: signer.to_string(),
                amount,
            })
        }
    }

    pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, ApplicationError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_sub(amount)
                .and_then(|r| {
                    *account = r;
                    Some(r)
                })
                .ok_or(ApplicationError::AccountUnderFunded(
                    signer.to_string(),
                    amount,
                ))
                // Using map() here is an easy way to only manipulate the non-error result
                .map(|_| Tx::Withdraw {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            Err(ApplicationError::AccountNotFound(signer.to_string()))
        }
    }

    pub fn send(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<(Tx, Tx), ApplicationError> {
        match self.accounts.get(sender) {
            Some(amt) if self.accounts.contains_key(recipient) && *amt >= amount => {
                let tx_withdraw = self.withdraw(sender, amount)?;
                self.deposit(recipient, amount)
                    .map_err(|e| {
                        self.deposit(sender, amount).unwrap();
                        e
                    })
                    .map(|tx_deposit| (tx_withdraw, tx_deposit))
            }
            Some(amt) if self.accounts.contains_key(recipient) && *amt < amount => Err(
                ApplicationError::AccountUnderFunded(sender.to_owned(), amount),
            ),
            Some(_) => Err(ApplicationError::AccountNotFound(recipient.to_owned())),
            None => Err(ApplicationError::AccountNotFound(sender.to_string())),
        }
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
            Err(ApplicationError::AccountUnderFunded(
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
            Err(ApplicationError::AccountOverFunded(
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

    #[test]
    fn test_accounts_send_works() {
        let mut accounts = Accounts::new();
        let amt = 100;
        accounts.deposit("a-key", amt).expect("Couldn't deposit");

        // creating the receiverzs
        accounts.deposit("b-key", 0).expect("Couldn't deposit");

        let (tx1, tx2) = accounts.send("a-key", "b-key", amt).expect("Send failed");
        assert_eq!(
            tx1,
            Tx::Withdraw {
                account: "a-key".to_string(),
                amount: amt
            }
        );
        assert_eq!(
            tx2,
            Tx::Deposit {
                account: "b-key".to_string(),
                amount: amt
            }
        );

        let actual = accounts.withdraw("b-key", amt);
        assert_eq!(
            actual,
            Ok(Tx::Withdraw {
                account: "b-key".to_string(),
                amount: amt
            })
        );
    }

    #[test]
    fn test_accounts_send_underfunded_fails_and_rolls_back() {
        let mut accounts = Accounts::new();
        let amt = 100;
        accounts.deposit("a-key", amt).expect("Couldn't deposit");

        // creating the receiver
        accounts.deposit("b-key", 0).expect("Couldn't deposit");

        let actual = accounts.send("a-key", "b-key", amt + 1);
        assert!(actual.is_err());
        let expected: HashMap<String, u64> =
            vec![("a-key".to_string(), amt), ("b-key".to_string(), 0)]
                .into_iter()
                .collect();
        assert_eq!(accounts.accounts, expected);
    }

    #[test]
    fn test_accounts_send_overfunded_fails_and_rolls_back() {
        let mut accounts = Accounts::new();
        let amt = 100;
        accounts.deposit("a-key", amt).expect("Couldn't deposit");

        // creating the receiver
        accounts
            .deposit("b-key", u64::MAX)
            .expect("Couldn't deposit");

        let actual = accounts.send("a-key", "b-key", 1);
        assert!(actual.is_err());
        let expected: HashMap<String, u64> =
            vec![("a-key".to_string(), amt), ("b-key".to_string(), u64::MAX)]
                .into_iter()
                .collect();
        assert_eq!(accounts.accounts, expected);
    }
}
