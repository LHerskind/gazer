use crate::collector::otterprovider::OtterProvider;
use crate::types::Transaction;
use async_trait::async_trait;
use ethers::{
    providers::{Http as HttpProvider, Middleware, Provider, ProviderError},
    types::TxHash,
};
use tokio::join;

#[async_trait]
pub(crate) trait TransactionCollector {
    async fn populate_transaction_from_hash(
        &self,
        tx_hash: TxHash,
    ) -> Result<Transaction, ProviderError>;
}

#[async_trait]
impl TransactionCollector for Provider<HttpProvider> {
    async fn populate_transaction_from_hash(
        &self,
        tx_hash: TxHash,
    ) -> Result<Transaction, ProviderError> {
        let tx = self.get_transaction(tx_hash);
        let tx_receipt = self.get_transaction_receipt(tx_hash);
        let internal = self.ots_get_internal_operations(tx_hash);

        let (tx, tx_receipt, internal) = join!(tx, tx_receipt, internal);

        let tx_fetch = match tx? {
            Some(tx) => tx,
            None => {
                return Err(ProviderError::CustomError("No tx found".to_string()));
            }
        };

        let tx_receipt = match tx_receipt? {
            Some(tx) => tx,
            None => {
                return Err(ProviderError::CustomError(
                    "No tx receipt found".to_string(),
                ));
            }
        };

        Ok(Transaction::from((internal?, tx_fetch, tx_receipt)))
    }
}
