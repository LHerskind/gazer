pub(crate) mod gazer;

pub(crate) const GNOSIS_SAFE_CODE_SIZE: usize = 171;
pub(crate) const ARGENT_CODE_SIZE: usize = 319;

use crate::collector::bindings::ierc20::TransferFilter;
use crate::collector::otterprovider::OtsInternals;
use dashmap::DashSet;
use ethers::types::{Address, TxHash, H256, U256, U64};
use ethers::{abi::RawLog, prelude::EthEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    pub tx_hash: TxHash,
    pub block_number: U64,
    pub from: Address,
    pub to: Option<Address>,
    pub value: U256,
    pub transfers: Vec<ValueTransfer>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum AssetType {
    #[default]
    ETH,
    ERC20(Address),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ValueTransfer {
    pub asset: AssetType,
    pub from: Address,
    pub to: Address,
    pub value: U256,
}

impl
    From<(
        OtsInternals,
        ethers::types::Transaction,
        ethers::types::TransactionReceipt,
    )> for Transaction
{
    fn from(
        (internal, tx, receipt): (
            OtsInternals,
            ethers::types::Transaction,
            ethers::types::TransactionReceipt,
        ),
    ) -> Self {
        let mut tx = Transaction {
            tx_hash: tx.hash,
            block_number: receipt.block_number.unwrap(),
            from: receipt.from,
            to: receipt.to,
            value: tx.value,
            transfers: Vec::new(),
        };

        if !tx.value.is_zero() {
            tx.transfers.push(ValueTransfer {
                asset: AssetType::ETH,
                from: tx.from,
                to: match tx.to {
                    Some(to) => to,
                    None => Address::zero(),
                },
                value: tx.value,
            });
        }

        let transfer_topic = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            .parse::<H256>()
            .unwrap();

        for log in receipt.logs {
            if log.topics[0] != transfer_topic {
                continue;
            }

            let event = TransferFilter::decode_log(&RawLog {
                topics: log.topics.clone(),
                data: log.data.to_vec(),
            });
            match event {
                Ok(transfer) => {
                    tx.transfers.push(ValueTransfer {
                        asset: AssetType::ERC20(log.address),
                        from: transfer.from,
                        to: transfer.to,
                        value: transfer.value,
                    });
                }
                Err(_) => (),
            }
        }

        for op in internal.ops {
            tx.transfers.push(ValueTransfer {
                asset: AssetType::ETH,
                from: op.from,
                to: op.to,
                value: op.value,
            });
        }

        tx
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Account {
    pub address: Address,
    pub code_size: Option<usize>,
    pub tx_hashes: DashSet<TxHash>,
    pub nonce: U256,
}

impl Account {
    pub(crate) fn new(address: Address) -> Self {
        Self {
            address,
            code_size: None,
            tx_hashes: DashSet::default(),
            nonce: U256::default(),
        }
    }

    pub(crate) fn set_code_size(&mut self, code_size: usize) {
        self.code_size = Some(code_size);
    }

    pub(crate) fn set_nonce(&mut self, nonce: U256) {
        self.nonce = nonce;
    }

    pub(crate) fn is_eoa(&self) -> bool {
        self.code_size.is_none() || self.code_size.unwrap() == 0
    }

    pub(crate) fn is_contract_wallet(&self) -> bool {
        // return false if no code_size or if code size is not a known wallet implementation
        self.code_size.is_some()
            && match self.code_size.unwrap() {
                GNOSIS_SAFE_CODE_SIZE | ARGENT_CODE_SIZE => true,
                _ => false,
            }
    }

    pub(crate) fn is_wallet(&self) -> bool {
        self.is_eoa() || self.is_contract_wallet()
    }
}
