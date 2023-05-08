use crate::types::gazer::GazerDB;
use clap::Parser;
use csv::Writer;
use ethers::types::{Address, U256};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExportAccount {
    pub address: Address,
    pub code_size: Option<usize>,
    pub nonce: U256,
}

#[derive(Debug, Parser)]
pub enum ExportCommands {
    #[clap(
        name = "accounts",
        about = "Exports all accounts from the database to a csv file"
    )]
    Accounts {
        #[clap(
        long,
        default_value_t = String::from("./export/accounts.csv"),
        help = "The path to export the csv to"
        )]
        path: String,
    },
    #[clap(
        name = "txs",
        about = "Exports all transfers from the database to a csv file"
    )]
    Txs {
        #[clap(
        long,
        default_value_t = String::from("./export/transfers.csv"),
        help = "The path to export the csv to"
    )]
        path: String,
    },
}

impl ExportCommands {
    pub fn run(self, gazer: &GazerDB) {
        match self {
            ExportCommands::Accounts { path } => export_accounts(gazer, path),
            ExportCommands::Txs { path } => export_value_transfers(gazer, path),
        }
    }
}

pub fn export_accounts(gazer: &GazerDB, path: String) {
    let path = Path::new(&path);
    match path.parent() {
        None => (),
        Some(parent) => match fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(_) => todo!(),
        },
    }

    let accounts = match gazer.get_account_keys() {
        Ok(accounts) => accounts,
        Err(_) => todo!(),
    };

    let mut wtr = Writer::from_path(path).unwrap();
    let pb = ProgressBar::new(accounts.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] accounts exported {pos}/{len}")
        .unwrap()
        .progress_chars("#>-"));
    pb.set_message("Exporting accounts");

    for account in accounts {
        if let Ok(Some(account)) = gazer.get_account(account) {
            wtr.serialize(ExportAccount {
                address: account.address,
                code_size: account.code_size,
                nonce: account.nonce,
            })
            .unwrap();
            pb.inc(1);
        }
    }
    pb.finish();
}

pub fn export_value_transfers(gazer: &GazerDB, path: String) {
    let path = Path::new(&path);
    match path.parent() {
        None => (),
        Some(parent) => match fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(_) => todo!(),
        },
    }

    let tx_hashes = match gazer.get_tx_keys() {
        Ok(txs) => txs,
        Err(_) => todo!(),
    };

    let mut wtr = Writer::from_path(path).unwrap();
    let pb = ProgressBar::new(tx_hashes.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] txs exported {pos}/{len}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb.set_message("Exporting value transfers");

    for tx_hash in tx_hashes {
        if let Ok(Some(tx)) = gazer.get_tx(tx_hash) {
            tx.transfers.iter().for_each(|transfer| {
                wtr.serialize(transfer).unwrap();
            });
            pb.inc(1);
        }
    }
    pb.finish();
}
