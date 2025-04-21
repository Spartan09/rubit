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

fn read_from_stdin(label: &str) -> String {
    let mut buffer = String::new();
    println!("{}", label);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_owned()
}

fn main() {
    println!("Hello, accounting world!");

    let mut ledger = Accounts::new();
    loop {
        let input = read_from_stdin(
            "Choose operation [deposit, withdraw, send, print, quit], confirm with return",
        );
        match input.as_str() {
            "deposit" => {
                let account = read_from_stdin("Account");
                let raw_amount = read_from_stdin("Amount");
                if let Ok(amount) = raw_amount.parse::<u64>() {
                    match ledger.deposit(&account, amount) {
                        Ok(_) => println!("Deposited {} to {}", amount, account),
                        Err(e) => println!("An error occurred during deposit: {:?}", e),
                    }
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "withdraw" => {
                let account = read_from_stdin("Account");
                let raw_amount = read_from_stdin("Amount");
                if let Ok(amount) = raw_amount.parse::<u64>() {
                    match ledger.withdraw(&account, amount) {
                        Ok(_) => println!("Withdrawn {} from {}", amount, account),
                        Err(e) => println!("An error occurred during withdrawal: {:?}", e),
                    }
                } else {
                    eprintln!("Not a number: '{:?}'", raw_amount);
                }
            }
            "send" => {
                let sender = read_from_stdin("Sender Account:");
                let recipient = read_from_stdin("Recipient Account:");
                let raw_amount = read_from_stdin("Amount:");
                if let Ok(amount) = raw_amount.parse::<u64>() {
                    match ledger.send(&sender, &recipient, amount) {
                        Ok((tx1, tx2)) => {
                            println!("Sent {} from {} to {}", amount, sender, recipient);
                            println!("Tx1: {:?}", tx1);
                            println!("Tx2: {:?}", tx2);
                        }
                        Err(e) => println!("An error occurred during send: {:?}", e),
                    }
                }
            }
            "print" => println!("The ledger {:?}", ledger),
            "quit" => {
                println!("Quitting...");
                break;
            }
            _ => println!("Usage: [deposit, withdraw, send, print, quit]"),
        }
    }
    // println!("Hello, accounting world!");
    //
    // // We are using simple &str instances as keys
    // // for more sophisticated keys (e.g. hashes)
    // // the data type could remain the same
    // let bob = "bob";
    // let alice = "alice";
    // let charlie = "charlie";
    // let initial_amount = 100;
    //
    // // Creates the basic ledger and a tx log container
    // let mut ledger = Accounts::new();
    // let mut tx_log = vec![];
    //
    // // Deposit an amount to each account
    // for signer in &[bob, alice, charlie] {
    //     let status = ledger.deposit(*signer, initial_amount);
    //     println!("Depositing {} for {}: {:?}", signer, initial_amount, status);
    //     // Add the resulting transaction to a list of transactions
    //     // .unwrap() will crash the program if the status is an error.
    //     tx_log.push(status.unwrap());
    // }
    //
    // // Send currency from one account (bob) to the other (alice)
    // let send_amount = 10_u64;
    // let status = ledger.send(bob, alice, send_amount);
    // println!(
    //     "Sent {} from {} to {}: {:?}",
    //     send_amount, bob, alice, status
    // );
    //
    // // Add both transactions to the transaction log
    // let (tx1, tx2) = status.unwrap();
    // tx_log.push(tx1);
    // tx_log.push(tx2);
    //
    // // Withdraw everything from the accounts
    // let tx = ledger.withdraw(charlie, initial_amount).unwrap();
    // tx_log.push(tx);
    // let tx = ledger
    //     .withdraw(alice, initial_amount + send_amount)
    //     .unwrap();
    // tx_log.push(tx);
    //
    // // Here we are withdrawing too much and there won't be a transaction
    // println!(
    //     "Withdrawing {} from {}: {:?}",
    //     initial_amount,
    //     bob,
    //     ledger.withdraw(bob, initial_amount)
    // );
    // // Withdrawing the expected amount results in a transaction
    // let tx = ledger.withdraw(bob, initial_amount - send_amount).unwrap();
    // tx_log.push(tx);
    //
    // // {:?} prints the Debug implementation, {:#?} pretty-prints it
    // println!("Ledger empty: {:?}", ledger);
    // println!("The TX log: {:#?}", tx_log);
}
