use {async_graphql::SimpleObject, velox_primitives::QueryResponse};

#[derive(SimpleObject)]
pub struct QueryResponseWithBlockHeight {
    pub response: QueryResponse,
    pub block_height: u64,
}
