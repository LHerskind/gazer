use crate::types::{Account, Transaction};
use dashmap::DashSet;
use ethers::types::{Address, TxHash, U64};
use microkv::errors::KVError;
use microkv::namespace::NamespaceMicrokv;
use microkv::MicroKV;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GazerDB {
    pub db: MicroKV,
}

impl GazerDB {
    // Account holds the account information
    // Tx holds the transaction information
    // Misc holds information related to when certain things were last updated
    // For one, when pulling for a specific user, we can add the users address to the misc database
    // along with the last block we git something on. By doing so, we might be able to skip a lot of previous work
    // However, this only makes sense when pulling for full user. And not adding transactions individually.
    pub fn connect(db: MicroKV) -> Self {
        Self { db }
    }

    // Slow. Don't do all the time, waste of resources
    pub fn commit(&self) -> Result<(), KVError> {
        self.db.commit()
    }

    pub fn account_db(&self) -> NamespaceMicrokv {
        self.db.namespace("account")
    }

    pub fn get_account_keys(&self) -> Result<Vec<Address>, KVError> {
        match self.account_db().keys() {
            Ok(keys) => Ok(keys
                .iter()
                .map(|key| {
                    key.split('@').collect::<Vec<_>>()[1]
                        .to_string()
                        .parse::<Address>()
                        .unwrap()
                })
                .collect()),
            Err(e) => Err(e),
        }
    }

    pub fn put_account(&self, account: Account) -> microkv::errors::Result<()> {
        let key = format!("{:?}", account.address);
        self.account_db().put(key, &account)
    }

    pub fn get_account(&self, address: Address) -> Result<Option<Account>, KVError> {
        let key = format!("{:?}", address);
        self.account_db().get(key)
    }

    pub fn delete_account(&self, address: Address) -> Result<(), KVError> {
        let key = format!("{:?}", address);
        self.account_db().delete(key)
    }

    pub fn tx_db(&self) -> NamespaceMicrokv {
        self.db.namespace("tx")
    }

    pub fn get_tx_keys(&self) -> Result<Vec<TxHash>, KVError> {
        match self.tx_db().keys() {
            Ok(keys) => Ok(keys
                .iter()
                .map(|key| {
                    key.split('@').collect::<Vec<_>>()[1]
                        .to_string()
                        .parse::<TxHash>()
                        .unwrap()
                })
                .collect()),
            Err(e) => Err(e),
        }
    }

    pub fn put_tx(&self, txs: Vec<Transaction>) -> Result<(), KVError> {
        let account_db = self.account_db();
        let tx_db = self.tx_db();

        for tx in txs {
            let tx_key = format!("{:?}", tx.tx_hash);
            match tx_db.put(tx_key, &tx) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            };

            let participants = DashSet::new();
            participants.insert(tx.from);
            if tx.to.is_some() {
                participants.insert(tx.to.unwrap());
            }
            tx.transfers.iter().for_each(|transfer| {
                participants.insert(transfer.from);
                participants.insert(transfer.to);
            });

            // Fetch the par
            participants.into_iter().for_each(|address| {
                let address_key = format!("{:?}", address.clone());
                let account = match account_db.get(address_key.clone()) {
                    Ok(Some(account)) => account,
                    Ok(None) | Err(_) => Account::new(address),
                };
                account.tx_hashes.insert(tx.tx_hash);
                account_db.put(address_key, &account).unwrap();
            });
        }

        Ok(())
    }

    pub fn get_tx(&self, tx_hash: TxHash) -> Result<Option<Transaction>, KVError> {
        let key = format!("{:?}", tx_hash);
        self.tx_db().get(key)
    }

    pub fn has_tx(&self, tx_hash: TxHash) -> Result<bool, KVError> {
        let key = format!("{:?}", tx_hash);
        self.tx_db().exists(key)
    }

    pub fn misc_db(&self) -> NamespaceMicrokv {
        self.db.namespace("misc")
    }

    pub fn all_fetched_accounts(&self) -> Result<Vec<Address>, KVError> {
        let misc_db = self.misc_db();
        let keys = misc_db.keys()?;
        Ok(keys
            .iter()
            .map(|key| {
                key.split('@').collect::<Vec<_>>()[1]
                    .to_string()
                    .parse::<Address>()
                    .unwrap()
            })
            .collect())
    }

    pub fn put_account_last_update(
        &self,
        addresses: Vec<Address>,
        block_number: U64,
    ) -> Result<(), KVError> {
        let misc_db = self.misc_db();
        for address in addresses {
            let key = format!("{:?}", address);
            misc_db.put(key, &block_number)?;
        }
        Ok(())
    }

    pub fn get_account_last_update(
        &self,
        addresses: Vec<Address>,
    ) -> Result<Vec<Option<U64>>, KVError> {
        let misc_db = self.misc_db();
        let mut block_numbers = Vec::new();
        for address in addresses {
            let key = format!("{:?}", address);
            let block_number = misc_db.get(key)?;
            block_numbers.push(block_number);
        }

        Ok(block_numbers)
    }
}
