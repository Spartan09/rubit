mod accounts;
mod errors;
mod tx;

use accounts::Accounts;
use std::io;

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
}
