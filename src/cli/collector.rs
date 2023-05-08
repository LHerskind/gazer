use crate::collector::account_collector::AccountCollector;
use crate::collector::transaction_collector::TransactionCollector;
use crate::types::gazer::GazerDB;
use crate::types::{Account, Transaction};
use clap::Parser;
use dashmap::{DashMap, DashSet};
use ethers::providers::{Http, Middleware, Provider, ProviderError, StreamExt};
use ethers::types::{Address, NameOrAddress, TxHash};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::panic;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::time::Duration;
use tokio::spawn;

#[derive(Debug, Parser)]
pub enum CollectorCommands {
    #[clap(
        name = "user",
        about = "Adds a user and his outgoing transactions to the database"
    )]
    User {
        #[clap(
        help = "Names or addresses of the users to pull",
        value_parser = Address::from_str
        )]
        name_or_addresses: Vec<Address>,
    },
    #[clap(name = "tx", about = "Adds a transaction to the database")]
    Tx {
        #[clap(help = "Transaction hash")]
        tx_hash: String,
    },
    #[clap(
        name = "populate-txs",
        about = "Populates the database with transactions"
    )]
    PopulateTxs {},
    #[clap(
        name = "populate-accounts",
        about = "Populates the database with code size data"
    )]
    PopulateAccounts {
        #[clap(short, name = "force", help = "force update of accounts")]
        force: bool,
    },
    #[clap(name = "update", about = "Updates the database with new transactions since last fetched")]
    Update {},
}

impl CollectorCommands {
    pub async fn run(self, gazer: &GazerDB, rpc_url: String) {
        let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url).unwrap();
        match self {
            CollectorCommands::User { name_or_addresses } => {
                // Something here don't make sense. Why is it matching it as a name and not an address?
                // To just progress, lets only take in an address.

                add_accounts_by_involved(gazer, provider, name_or_addresses)
                    .await
                    .expect("Error adding accounts");
            }
            CollectorCommands::Update {} => {
                // TODO: Would probably be nice with some kind of indicator on how many of the users we have updated
                match gazer.all_fetched_accounts() {
                    Ok(accounts) => {
                        add_accounts_by_involved(gazer, provider, accounts)
                            .await
                            .expect("Error adding accounts");
                    }
                    Err(_) => {
                        println!("No accounts to update");
                    }
                }
            }
            CollectorCommands::Tx { tx_hash } => {
                let tx_hash = tx_hash.parse::<TxHash>().unwrap();
                if fetch_and_add_tx(gazer, provider, vec![tx_hash], None).await {
                    println!("Added transaction");
                } else {
                    println!("Transaction already in database");
                };
            }
            CollectorCommands::PopulateTxs {} => populate_txs(gazer, provider).await,
            CollectorCommands::PopulateAccounts { force } => {
                populate_accounts(gazer, provider, force).await
            }
        }
    }
}

pub async fn fetch_and_update_accounts(
    gazer: &GazerDB,
    provider: Provider<Http>,
    addresses: Vec<Address>,
    pb: Option<&ProgressBar>,
) {
    let res = DashMap::new();
    let mut v = Vec::new();
    for address in addresses {
        v.push(provider.get_account_info(address));
    }

    tokio_stream::iter(v)
        .for_each_concurrent(25, |info| async {
            if let Ok(info) = info.await {
                res.insert(info.0, (info.1, info.2));
                if let Some(pb) = pb {
                    pb.inc(1);
                }
            }
        })
        .await;

    for (addr, (nonce, code_size)) in res {
        let mut account = match gazer.get_account(addr) {
            Ok(Some(account)) => account,
            Ok(None) | Err(_) => Account::new(addr),
        };

        account.set_code_size(code_size);
        account.set_nonce(nonce);

        gazer.put_account(account).expect("TODO: panic message");
    }
    match gazer.commit() {
        Ok(_) => (),
        Err(_) => panic!("Error committing to database"),
    }
}

pub async fn fetch_and_add_tx(
    gazer: &GazerDB,
    provider: Provider<Http>,
    tx_hashes: Vec<TxHash>,
    pb: Option<&ProgressBar>,
) -> bool {
    let to_fetch = DashSet::new();
    for tx_hash in tx_hashes {
        match gazer.has_tx(tx_hash) {
            Ok(true) => true,
            Ok(false) => to_fetch.insert(tx_hash),
            Err(_) => panic!("Error checking if transaction exists in database"),
        };
    }

    let res = DashMap::new();
    let mut v = Vec::new();
    for tx_hash in to_fetch {
        v.push(provider.populate_transaction_from_hash(tx_hash));
    }

    tokio_stream::iter(v)
        .for_each_concurrent(25, |tx| async {
            if let Ok(tx) = tx.await {
                res.insert(tx.tx_hash, tx);
                if let Some(pb) = pb {
                    pb.inc(1);
                }
            }
        })
        .await;

    for (_, tx) in res {
        gazer.put_tx(vec![tx]).expect("TODO: panic message");
    }

    match gazer.commit() {
        Ok(_) => (),
        Err(_) => panic!("Error committing to database"),
    }

    true
}

pub async fn add_accounts_by_involved<T: Into<NameOrAddress> + Send + Sync>(
    gazer: &GazerDB,
    provider: Provider<Http>,
    addresses: Vec<T>,
) -> Result<(), ProviderError> {
    let mut addrs: Vec<Address> = Vec::new();
    let block_number = provider.get_block_number().await?;
    let (sender, receiver) = channel::<TxHash>();

    let spinner_thread = spawn(async move {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] {prefix} {pos} transactions fetched. {msg}",
            )
            .unwrap(),
        );
        spinner.set_prefix("Fetching transactions...");
        spinner.set_message("");
        spinner.enable_steady_tick(Duration::from_millis(100));

        while let Ok(tx_hash) = receiver.recv() {
            spinner.inc(1);
            if tx_hash.is_zero() {
                spinner.set_message("Committed data to database");
            } else {
                spinner.set_message(format!("Latest tx: {:?}", tx_hash));
            }
        }

        spinner.finish_with_message("Finished fetching transactions");
    });

    for addr in addresses {
        let addr = match addr.into() {
            NameOrAddress::Name(ens_name) => match provider.resolve_name(&ens_name).await {
                Ok(addr) => addr,
                Err(err) => panic!("Error resolving name: {}", err),
            },
            NameOrAddress::Address(addr) => addr,
        };
        addrs.push(addr);
    }

    while !addrs.is_empty() {
        let to_consume = match addrs.len() {
            0..=25 => addrs.len(),
            _ => 25,
        };
        let temp = addrs.drain(..to_consume).collect::<Vec<Address>>();

        let mut v = Vec::new();

        let block_numbers = match gazer.get_account_last_update(temp.clone()) {
            Ok(block_numbers) => block_numbers,
            Err(_) => panic!("Error getting account last update"),
        };

        for i in 0..temp.len() {
            let addr = temp[i];
            let block_number = block_numbers[i];
            v.push(provider.get_involved_txs(addr, block_number, sender.clone()));
        }

        let tot_txs: DashMap<TxHash, Transaction> = DashMap::new();
        tokio_stream::iter(v)
            .for_each_concurrent(10, |txs| async {
                match txs.await {
                    Ok(txs) => {
                        txs.into_iter().for_each(|tx| {
                            tot_txs.insert(tx.tx_hash, tx);
                        });
                    }
                    Err(err) => panic!("Error getting involved txs: {}", err),
                }
            })
            .await;

        let tot_txs = tot_txs
            .into_iter()
            .map(|(_, tx)| tx)
            .collect::<Vec<Transaction>>();

        gazer.put_tx(tot_txs).expect("TODO: panic message");

        gazer
            .put_account_last_update(temp, block_number)
            .expect("TODO: panic message");

        match gazer.commit() {
            Ok(_) => sender.send(TxHash::zero()).expect("TODO: panic message"),
            Err(err) => panic!("Error committing to database: {}", err),
        }
    }
    drop(sender);

    spinner_thread.await.expect("TODO: panic message");

    match gazer.commit() {
        Ok(_) => {
            println!("Committed to database");
            Ok(())
        }
        Err(err) => panic!("Error committing to database: {}", err),
    }
}

pub async fn populate_txs(gazer: &GazerDB, provider: Provider<Http>) {
    let mut tx_hashes = DashSet::new();
    let accounts = match gazer.get_account_keys() {
        Ok(accounts) => accounts,
        Err(_) => panic!("Error getting account keys from database"),
    };

    for account in accounts {
        let account: Account = match gazer.get_account(account) {
            Ok(Some(account)) => account,
            Ok(None) | Err(_) => panic!("Error getting account from database"),
        };
        tx_hashes.extend(account.tx_hashes.into_iter());
    }

    let mut tx_hashes = tx_hashes
        .into_iter()
        .filter(|tx_hash| match gazer.get_tx(*tx_hash) {
            Ok(Some(_)) => false,
            Ok(None) => true,
            Err(_) => panic!("Error getting transaction from database"),
        })
        .collect::<Vec<TxHash>>();

    let pb = ProgressBar::new(tx_hashes.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} (eta: {eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    while !tx_hashes.is_empty() {
        let to_consume = match tx_hashes.len() {
            0..=250 => tx_hashes.len(),
            _ => 250,
        };
        let hashes = tx_hashes.drain(..to_consume).collect::<Vec<TxHash>>();
        fetch_and_add_tx(gazer, provider.clone(), hashes, Some(&pb)).await;
    }

    match gazer.commit() {
        Ok(_) => (),
        Err(_) => panic!("Error committing to database"),
    }

    pb.finish();
}

pub async fn populate_accounts(gazer: &GazerDB, provider: Provider<Http>, force: bool) {
    let accounts = match gazer.get_account_keys() {
        Ok(accounts) => accounts,
        Err(_) => panic!("Error getting account keys from database"),
    };

    let accounts_to_lookup = DashSet::new();
    for address in accounts {
        match gazer.get_account(address) {
            Ok(Some(account)) => {
                if force || account.code_size.is_none() {
                    accounts_to_lookup.insert(address);
                }
            }
            Ok(None) | Err(_) => {
                accounts_to_lookup.insert(address);
            }
        };
    }

    let pb = ProgressBar::new(accounts_to_lookup.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} (eta: {eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );
    pb.set_message("Collecting accounts");

    let mut accounts_to_lookup = accounts_to_lookup
        .into_iter()
        .collect::<Vec<Address>>();

    while !accounts_to_lookup.is_empty() {
        let to_consume = match accounts_to_lookup.len() {
            0..=1000 => accounts_to_lookup.len(),
            _ => 1000,
        };
        let addresses = accounts_to_lookup
            .drain(..to_consume)
            .collect::<Vec<Address>>();
        fetch_and_update_accounts(gazer, provider.clone(), addresses, Some(&pb)).await;
    }

    match gazer.commit() {
        Ok(_) => (),
        Err(_) => panic!("Error committing to database"),
    }

    pb.finish();
}
