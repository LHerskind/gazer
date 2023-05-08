mod cli;
mod collector;
mod types;

use microkv::MicroKV;
use std::path::PathBuf;

use crate::cli::{collector::CollectorCommands, exporter::ExportCommands, remover::RemoveCommands};
use crate::types::gazer::GazerDB;
use clap::{Parser, Subcommand};
use ethers::types::{Address, TxHash, U64};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author = "Lasse Herskind <lherskind@proton.me>")]
#[command(about = "Gazing Transactions", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "stats", about = "Gives basic stats of the database")]
    Stats {},
    #[clap(name = "get-tx", about = "Print a transaction from the database")]
    GetTx {
        #[clap(help = "Transaction hash", value_parser = TxHash::from_str) ]
        tx_hash: TxHash,
    },
    #[clap(
        name = "get-account",
        about = "Print transfers for the address from the database"
    )]
    GetAccount {
        #[clap(help = "Address", value_parser = Address::from_str) ]
        address: Address,
    },
    #[clap(name = "export", about = "Exports data from the database to csv files")]
    Export {
        #[clap(subcommand)]
        command: ExportCommands,
    },
    #[clap(
        name = "collect",
        about = "Fetch data from the chain and put it into the database"
    )]
    Collect {
        #[clap(
        long,
        default_value_t = String::from("http://localhost:8545"),
        help = "The RPC url to an ethereum node"
        )]
        rpc_url: String,
        #[clap(subcommand)]
        command: CollectorCommands,
    },
    #[clap(name = "remove", about = "Remove data from the database")]
    Remove {
        #[clap(subcommand)]
        command: RemoveCommands,
    },
}

#[tokio::main]
async fn main() {
    let db = MicroKV::open_with_base_path("gazer", PathBuf::from("./data/"))
        .expect("Failed to open database");
    let gazer = GazerDB::connect(db);

    let cli = Cli::parse();

    match cli.command {
        Commands::Stats {} => {
            let unique_tx_count = match gazer.get_tx_keys() {
                Ok(txs) => txs.len(),
                Err(_) => {
                    println!("No transactions found");
                    return;
                }
            };
            let accounts = match gazer.get_account_keys() {
                Ok(accounts) => accounts,
                Err(_) => {
                    println!("No accounts found");
                    return;
                }
            };
            let account_count = accounts.len();

            let mut wallet_accounts = 0;
            let mut tx_count = 0;

            for account in accounts {
                let account = gazer.get_account(account);
                if let Ok(Some(account)) = account {
                    tx_count += account.tx_hashes.len();
                    if account.code_size.is_some() && account.is_wallet() {
                        wallet_accounts += 1;
                    }
                }
            }

            let format_with_separator = |n: usize| {
                let mut chars: Vec<char> = format!("{}", n).chars().rev().collect();
                for i in (3..chars.len()).step_by(4) {
                    chars.insert(i, ',');
                }
                chars.iter().rev().collect::<String>()
            };

            println!(
                "Accounts: {} ({} wallets)",
                format_with_separator(account_count),
                format_with_separator(wallet_accounts)
            );
            println!("Unique Transactions: {}", format_with_separator(unique_tx_count));
            println!("Summed transactions for users: {}", format_with_separator(tx_count));
        }
        Commands::GetTx { tx_hash } => match gazer.get_tx(tx_hash) {
            Ok(Some(tx)) => {
                println!("{:#?}", tx);
            }
            _ => println!("Transaction not found"),
        },
        Commands::GetAccount { address } => {
            let account = match gazer.get_account(address) {
                Ok(Some(account)) => account,
                _ => {
                    println!("Account not found");
                    return;
                }
            };
            let last_update = match gazer.get_account_last_update(vec![address]) {
                Ok(l) => match l[0] {
                    Some(l) => l,
                    None => U64::from(0),
                },
                _ => U64::from(0),
            };
            println!(
                "Account: {:?} has a last_update of {:?}",
                address, last_update
            );

            let mut value_txs = Vec::new();

            account.tx_hashes.into_iter().for_each(|tx_hash| {
                let tx = gazer.get_tx(tx_hash).unwrap().unwrap();

                tx.transfers.into_iter().for_each(|transfer| {
                    if transfer.from == address || transfer.to == address {
                        value_txs.push(transfer);
                    }
                });
            });
            println!("{:#?}", value_txs);
        }
        Commands::Collect { rpc_url, command } => command.run(&gazer, rpc_url).await,
        Commands::Export { command } => command.run(&gazer),
        Commands::Remove { command } => command.run(&gazer),
    }
}
