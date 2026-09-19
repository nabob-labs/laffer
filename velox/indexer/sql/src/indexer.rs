use {
    crate::context::Context,
    async_trait::async_trait,
    velox_indexer_sql_migration::{Migrator, MigratorTrait},
    bolt::{BlockAndBlockOutcomeWithHttpDetails, Config, Json, Storage},
};
#[cfg(feature = "metrics")]
use {
    metrics::counter,
    metrics::{describe_histogram, histogram},
    std::time::Instant,
};

mod accounts;
mod perps_events;
pub mod perps_trades;
mod transfers;

pub struct Indexer {
    pub context: Context,
}

impl Indexer {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}

#[async_trait]
impl bolt_app::Indexer for Indexer {
    async fn last_indexed_block_height(&self) -> bolt_app::IndexerResult<Option<u64>> {
        // TODO: Implement last_indexed_block_height
        Ok(None)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    async fn start(&mut self, _storage: &dyn Storage) -> bolt_app::IndexerResult<()> {
        #[cfg(feature = "metrics")]
        let start = Instant::now();

        Migrator::up(&self.context.db, None)
            .await
            .map_err(|e| bolt_app::IndexerError::database(e.to_string()))?;

        #[cfg(feature = "metrics")]
        {
            transfers::init_metrics();
            accounts::init_metrics();
            perps_events::init_metrics();
            init_metrics();

            histogram!("indexer.velox.start.duration").record(start.elapsed().as_secs_f64());
        }

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    async fn post_indexing(
        &self,
        block_height: u64,
        _cfg: Config,
        app_cfg: Json,
        ctx: &mut bolt_app::IndexerContext,
    ) -> bolt_app::IndexerResult<()> {
        #[cfg(feature = "metrics")]
        let start = Instant::now();

        let block_to_index = ctx.get::<BlockAndBlockOutcomeWithHttpDetails>().ok_or(
            bolt_app::IndexerError::hook(
                "BlockAndBlockOutcomeWithHttpDetails not found".to_string(),
            ),
        )?;

        // Run transfer, account, and perps event processing in parallel
        let (transfers_result, accounts_result, perps_result) = tokio::join!(
            transfers::save_transfers(&self.context, block_height),
            accounts::save_accounts(&self.context, block_to_index, app_cfg.clone()),
            perps_events::save_perps_events(&self.context, block_to_index, app_cfg)
        );

        // Handle errors and increment counters
        if transfers_result.is_err() {
            #[cfg(feature = "metrics")]
            counter!("indexer.velox.hooks.transfers.errors.total").increment(1);
        }
        if accounts_result.is_err() {
            #[cfg(feature = "metrics")]
            counter!("indexer.velox.hooks.accounts.errors.total").increment(1);
        }
        if perps_result.is_err() {
            #[cfg(feature = "metrics")]
            counter!("indexer.velox.hooks.perps_events.errors.total").increment(1);
        }

        // Return the first error if any
        transfers_result?;
        accounts_result?;
        perps_result?;

        self.context
            .pubsub
            .publish(block_height)
            .await
            .map_err(|e| bolt_app::IndexerError::hook(e.to_string()))
            .inspect_err(|_| {
                #[cfg(feature = "metrics")]
                counter!("indexer.velox.hooks.pubsub.errors.total").increment(1);
            })?;

        #[cfg(feature = "metrics")]
        histogram!("indexer.velox.hooks.duration").record(start.elapsed().as_secs_f64());

        Ok(())
    }
}

#[cfg(feature = "metrics")]
pub fn init_metrics() {
    describe_histogram!("indexer.velox.hooks.duration", "Hook duration in seconds");
}
