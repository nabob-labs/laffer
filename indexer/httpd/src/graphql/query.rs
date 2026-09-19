use {
    async_graphql::MergedObject, block::BlockQuery, event::EventQuery,
    bolt_httpd::graphql::query::bolt::BoltQuery, message::MessageQuery,
    transaction::TransactionQuery,
};

pub mod block;
pub mod event;
pub mod message;
pub mod pagination;
pub mod transaction;

#[derive(MergedObject, Default)]
pub struct IndexerQuery(
    BlockQuery,
    TransactionQuery,
    MessageQuery,
    EventQuery,
    BoltQuery,
);
