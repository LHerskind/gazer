use crate::types::gazer::GazerDB;
use clap::Parser;
use ethers::types::Address;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum RemoveCommands {
    #[clap(name = "user", about = "Remove a user from the database")]
    User {
        #[clap(
        help = "Names or addresses of the users to remove",
        value_parser = Address::from_str
        )]
        name_or_addresses: Vec<Address>,
    },
    #[clap(name = "all-txs", about = "Deletes all transactions")]
    AllTxs {},
}

impl RemoveCommands {
    pub fn run(self, gazer: &GazerDB) {
        match self {
            RemoveCommands::User { name_or_addresses } => {
                for name_or_address in name_or_addresses {
                    match gazer.delete_account(name_or_address) {
                        Ok(_) => println!("Removed {:?}", name_or_address),
                        Err(_) => println!("{:?} not found", name_or_address),
                    }
                }
                match gazer.commit() {
                    Ok(_) => (),
                    Err(_) => println!("Failed to commit to db"),
                }
            }
            RemoveCommands::AllTxs {} => {
                println!("Clearing TX database");
                /*let tx_db = db.namespace("tx");
                let transactions = tx_db.keys().unwrap();
                for tx in transactions {
                    let key = tx.split("@").collect::<Vec<&str>>()[1];
                    tx_db.delete(key).unwrap();
                }
                db.commit().unwrap();
                println!("Cleared transactions");*/
            }
        }
    }
}
