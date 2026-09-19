use {
    actix_web::{
        App,
        body::MessageBody,
        dev::{ServiceFactory, ServiceRequest, ServiceResponse},
        web,
    },
    velox_httpd::{graphql::build_schema, server::config_app},
    bolt_httpd::{
        routes::graphql::GraphqlRequestTimeout, subscription_limiter::SubscriptionLimiter,
    },
    serde::{Serialize, de::DeserializeOwned},
    std::time::Duration,
};

// Re-export PaginationDirection from indexer_testing
pub use indexer_testing::PaginationDirection;

// Re-export query modules from velox_graphql_types for use in tests
pub use velox_graphql_types::{
    Accounts, Blocks, Events, Messages, Transactions, Transfers, accounts as accounts_query,
    blocks as blocks_query, events as events_query, messages as messages_query,
    transactions as transactions_query, transfers as transfers_query,
};

mod accounts;
mod candles;
mod metrics;
mod pair_stats;
mod perps_candles;
mod perps_events;
mod perps_pair_stats;
mod shutdown;
mod trades;
mod transfers;
mod users;

pub fn build_actix_app(
    velox_httpd_context: velox_httpd::context::Context,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = actix_web::Error,
    >,
> {
    let graphql_schema = build_schema(velox_httpd_context.clone());

    App::new()
        .app_data(web::Data::new(SubscriptionLimiter::new(10, 5000)))
        .app_data(web::Data::new(GraphqlRequestTimeout(Duration::from_secs(
            30,
        ))))
        .app_data(web::Data::new(velox_httpd_context.clone()))
        .app_data(web::Data::new(
            velox_httpd_context.indexer_httpd_context.clone(),
        ))
        .app_data(web::Data::new(graphql_schema.clone()))
        .configure(config_app(velox_httpd_context, graphql_schema))
}

/// Helper function to make GraphQL queries in tests.
///
/// This reduces boilerplate by handling:
/// - Building the velox actix app from context
/// - Delegating to indexer_testing::call_graphql_query
///
/// # Example
/// ```ignore
/// let response = call_graphql_query::<_, accounts::ResponseData>(
///     velox_httpd_context,
///     Accounts::build_query(accounts::Variables::default()),
/// ).await?;
/// ```
pub async fn call_graphql_query<V, R>(
    context: velox_httpd::context::Context,
    query_body: graphql_client::QueryBody<V>,
) -> anyhow::Result<graphql_client::Response<R>>
where
    V: Serialize,
    R: DeserializeOwned,
{
    let app = build_actix_app(context);
    indexer_testing::call_graphql_query(app, query_body).await
}

// Generate pagination test helpers using the shared macro from indexer_testing
indexer_testing::impl_paginate!(
    paginate_accounts,
    velox_httpd::context::Context,
    Accounts,
    accounts_query,
    accounts,
    AccountsAccountsNodes,
    build_actix_app
);
indexer_testing::impl_paginate!(
    paginate_transfers,
    velox_httpd::context::Context,
    Transfers,
    transfers_query,
    transfers,
    TransfersTransfersNodes,
    build_actix_app
);
indexer_testing::impl_paginate!(
    paginate_transactions,
    velox_httpd::context::Context,
    Transactions,
    transactions_query,
    transactions,
    TransactionsTransactionsNodes,
    build_actix_app
);
indexer_testing::impl_paginate!(
    paginate_blocks,
    velox_httpd::context::Context,
    Blocks,
    blocks_query,
    blocks,
    BlocksBlocksNodes,
    build_actix_app
);
indexer_testing::impl_paginate!(
    paginate_events,
    velox_httpd::context::Context,
    Events,
    events_query,
    events,
    EventsEventsNodes,
    build_actix_app
);
indexer_testing::impl_paginate!(
    paginate_messages,
    velox_httpd::context::Context,
    Messages,
    messages_query,
    messages,
    MessagesMessagesNodes,
    build_actix_app
);
