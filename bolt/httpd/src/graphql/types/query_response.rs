use {async_graphql::SimpleObject, bolt_types::QueryResponse};

#[derive(SimpleObject)]
pub struct QueryResponseWithBlockHeight {
    pub response: QueryResponse,
    pub block_height: u64,
}
