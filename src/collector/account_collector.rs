use crate::collector::otterprovider::{OtsInternals, OtsSearchTransactionsResponse, OtterProvider};
use crate::types::Transaction;
use async_trait::async_trait;
use dashmap::DashMap;
use ethers::abi::Address;
use ethers::providers::StreamExt;
use ethers::{
    providers::{Http as HttpProvider, Middleware, Provider, ProviderError},
    types::{NameOrAddress, TxHash, U256, U64},
};
use std::sync::mpsc::Sender;
use tokio::join;

#[async_trait]
pub(crate) trait AccountCollector {
    async fn get_involved_txs<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
        block_number: Option<U64>,
        pb: Sender<TxHash>,
    ) -> Result<Vec<Transaction>, ProviderError>;

    async fn get_originating_tx_hashes<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
        start_nonce: U256,
    ) -> Result<Vec<TxHash>, ProviderError>;

    async fn get_account_info<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
    ) -> Result<(Address, U256, usize), ProviderError>;
}

#[async_trait]
impl AccountCollector for Provider<HttpProvider> {
    async fn get_involved_txs<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
        block_number: Option<U64>,
        pb: Sender<TxHash>,
    ) -> Result<Vec<Transaction>, ProviderError> {
        let addr = match addr.into() {
            NameOrAddress::Name(ens_name) => self.resolve_name(&ens_name).await?,
            NameOrAddress::Address(addr) => addr,
        };
        let mut block_number = match block_number {
            Some(block_number) => block_number,
            None => U64::zero(),
        };

        let mut txs: Vec<Transaction> = Vec::new();

        let mut completed = false;
        while !completed {
            let mut page_size = 100;
            let mut page: Option<OtsSearchTransactionsResponse> = None;

            while page.is_none() {
                if page_size == 0 {
                    break;
                }

                match self
                    .ots_search_transactions_after(addr, block_number.as_u64(), page_size)
                    .await
                {
                    Ok(p) => page = Some(p),
                    Err(_) => {
                        page_size /= 2;
                        println!("Reducing page size to {:?} for {:?}", page_size, addr);
                    }
                }
            }

            let page = match page {
                Some(page) => page,
                None => {
                    break;
                }
            };

            let mut internal_to_fetch = Vec::new();

            page.txs.iter().for_each(|tx| {
                internal_to_fetch.push(self.ots_get_internal_operations(tx.hash));
            });

            let res: DashMap<TxHash, OtsInternals> = DashMap::new();
            tokio_stream::iter(internal_to_fetch)
                .for_each_concurrent(25, |call| async {
                    if let Ok(ops) = call.await {
                        res.insert(ops.tx_hash, ops);
                    }
                })
                .await;

            for i in 0..page.txs.len() {
                let tx = page.txs[i].clone();
                if block_number < tx.block_number.unwrap() {
                    block_number = tx.block_number.unwrap();
                }
                let receipt = page.receipts[i].clone();
                let internals = match res.get(&tx.hash) {
                    Some(internals) => internals.clone(),
                    None => OtsInternals {
                        tx_hash: tx.hash,
                        ops: Vec::new(),
                    },
                };
                pb.send(tx.hash).unwrap();
                txs.push(Transaction::from((internals, tx, receipt)));
            }
            completed = page.first_page;
        }

        Ok(txs)
    }

    async fn get_originating_tx_hashes<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
        start_nonce: U256,
    ) -> Result<Vec<TxHash>, ProviderError> {
        let addr = match addr.into() {
            NameOrAddress::Name(ens_name) => self.resolve_name(&ens_name).await?,
            NameOrAddress::Address(addr) => addr,
        };

        let nonce = self.get_transaction_count(addr, None).await?;
        let mut tx_hashes: Vec<TxHash> = Vec::new();

        for i in start_nonce.as_u64()..(nonce.as_u64() + 1) {
            if let Some(tx) = self
                .ots_get_transaction_by_sender_and_nonce(addr, i)
                .await?
            {
                tx_hashes.push(tx);
            }
        }
        Ok(tx_hashes)
    }

    async fn get_account_info<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
    ) -> Result<(Address, U256, usize), ProviderError> {
        let addr = match addr.into() {
            NameOrAddress::Name(ens_name) => self.resolve_name(&ens_name).await?,
            NameOrAddress::Address(addr) => addr,
        };

        let nonce = self.get_transaction_count(addr, None);
        let code = self.get_code(addr, None);

        let (nonce, code) = join!(nonce, code);

        let nonce = nonce?;
        let code = code?;

        Ok((addr, nonce, code.len()))
    }
}
