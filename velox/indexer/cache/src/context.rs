#[cfg(feature = "s3")]
use crate::S3Config;
use {
    crate::IndexerPath,
    std::sync::{Arc, Mutex},
    velox_primitives::TransactionsHttpdRequest,
};

#[derive(Clone, Default)]
pub struct Context {
    pub transactions_http_request_details: Arc<Mutex<TransactionsHttpdRequest>>,
    pub indexer_path: IndexerPath,
    #[cfg(feature = "s3")]
    pub s3: S3Config,
}
