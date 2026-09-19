use {
    async_graphql::*, block::BlockSubscription, event::EventSubscription, bolt::BoltSubscription,
    message::MessageSubscription, transaction::TransactionSubscription,
};

pub mod block;
pub mod event;
pub mod bolt;
pub mod message;
pub mod transaction;

pub const MAX_PAST_BLOCKS: usize = 100;

#[derive(MergedSubscription, Default)]
pub struct IndexerSubscription(
    BlockSubscription,
    TransactionSubscription,
    MessageSubscription,
    EventSubscription,
    BoltSubscription,
);
