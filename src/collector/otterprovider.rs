use async_trait::async_trait;
use ethers::providers::ProviderError;
use ethers::{
    providers::{Http as HttpProvider, Middleware, Provider},
    types::{Address, NameOrAddress, Transaction, TransactionReceipt, TxHash, U256},
    utils,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtsSearchTransactionsResponse {
    pub txs: Vec<Transaction>,
    pub receipts: Vec<TransactionReceipt>,
    pub first_page: bool,
    pub last_page: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OtsInternals {
    pub tx_hash: TxHash,
    pub ops: Vec<OtsOperation>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OtsOperation {
    #[serde(rename = "type")]
    pub tx_type: u64,
    pub from: Address,
    pub to: Address,
    pub value: U256,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OtsContractCreator {
    pub hash: TxHash,
    pub creator: Address,
}

#[async_trait]
pub trait OtterProvider {
    async fn ots_get_internal_operations(
        &self,
        tx_hash: TxHash,
    ) -> Result<OtsInternals, ProviderError>;
    async fn ots_search_transactions_after<T>(
        &self,
        addr: T,
        block_number: u64,
        page_size: u64,
    ) -> Result<OtsSearchTransactionsResponse, ProviderError>
    where
        T: Into<NameOrAddress> + Send + Sync;
    async fn ots_search_transactions_before<T>(
        &self,
        addr: T,
        block_number: u64,
        page_size: u64,
    ) -> Result<OtsSearchTransactionsResponse, ProviderError>
    where
        T: Into<NameOrAddress> + Send + Sync;
    async fn ots_get_transaction_by_sender_and_nonce<T>(
        &self,
        addr: T,
        nonce: u64,
    ) -> Result<Option<TxHash>, ProviderError>
    where
        T: Into<NameOrAddress> + Send + Sync;
    async fn ots_get_contract_creator<T>(
        &self,
        addr: T,
    ) -> Result<Option<OtsContractCreator>, ProviderError>
    where
        T: Into<NameOrAddress> + Send + Sync;
}

#[async_trait]
impl OtterProvider for Provider<HttpProvider> {
    async fn ots_get_internal_operations(
        &self,
        tx_hash: TxHash,
    ) -> Result<OtsInternals, ProviderError> {
        let tx_hash_value = utils::serialize(&tx_hash);
        let ops = self
            .request("ots_getInternalOperations", [tx_hash_value])
            .await;
        match ops {
            Ok(ops) => Ok(OtsInternals { tx_hash, ops }),
            Err(err) => Err(err),
        }
    }

    async fn ots_search_transactions_after<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
        block_number: u64,
        page_size: u64,
    ) -> Result<OtsSearchTransactionsResponse, ProviderError> {
        let addr = match addr.into() {
            NameOrAddress::Name(ens_name) => self.resolve_name(&ens_name).await?,
            NameOrAddress::Address(addr) => addr,
        };
        let addr = utils::serialize(&addr);
        let block_number = utils::serialize(&block_number);
        let page_size = utils::serialize(&page_size);
        self.request(
            "ots_searchTransactionsAfter",
            [addr, block_number, page_size],
        )
        .await
    }

    async fn ots_search_transactions_before<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
        block_number: u64,
        page_size: u64,
    ) -> Result<OtsSearchTransactionsResponse, ProviderError> {
        let addr = match addr.into() {
            NameOrAddress::Name(ens_name) => self.resolve_name(&ens_name).await?,
            NameOrAddress::Address(addr) => addr,
        };
        let addr = utils::serialize(&addr);
        let block_number = utils::serialize(&block_number);
        let page_size = utils::serialize(&page_size);
        self.request(
            "ots_searchTransactionsBefore",
            [addr, block_number, page_size],
        )
        .await
    }

    async fn ots_get_transaction_by_sender_and_nonce<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
        nonce: u64,
    ) -> Result<Option<TxHash>, ProviderError> {
        let addr = match addr.into() {
            NameOrAddress::Name(ens_name) => self.resolve_name(&ens_name).await?,
            NameOrAddress::Address(addr) => addr,
        };
        let addr = utils::serialize(&addr);
        let nonce = utils::serialize(&nonce);
        self.request("ots_getTransactionBySenderAndNonce", [addr, nonce])
            .await
    }

    async fn ots_get_contract_creator<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        addr: T,
    ) -> Result<Option<OtsContractCreator>, ProviderError> {
        let addr = match addr.into() {
            NameOrAddress::Name(ens_name) => self.resolve_name(&ens_name).await?,
            NameOrAddress::Address(addr) => addr,
        };
        let addr = utils::serialize(&addr);
        self.request("ots_getContractCreator", [addr]).await
    }
}
