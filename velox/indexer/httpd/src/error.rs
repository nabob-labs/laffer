use {std::io, velox_indexer_sql::error::IndexerError};

#[velox_backtrace::backtrace]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    #[backtrace(new)]
    Io(io::Error),

    #[error(transparent)]
    Indexer(IndexerError),
}
