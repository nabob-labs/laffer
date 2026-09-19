use {
    metrics::{describe_counter, describe_histogram},
    std::sync::Once,
};

pub const LABEL_TX_PER_BLOCK: &str = "bolt.app.tx_per_block";

pub const LABEL_SUCCESSFUL_TX: &str = "bolt.app.successful_tx_count";

pub const LABEL_FAILED_TX: &str = "bolt.app.failed_tx_count";

pub const LABEL_PROCESSED_MSGS: &str = "bolt.app.processed_msgs_count";

pub const LABEL_PROCESSED_QUERIES: &str = "bolt.app.processed_queries_count";

pub const LABEL_DURATION_BLOCK: &str = "bolt.app.block.duration";

pub const LABEL_DURATION_TX: &str = "bolt.app.tx.duration";

pub const LABEL_DURATION_PREPARE_PROPOSAL: &str = "bolt.app.prepare_proposal.duration";

pub const LABEL_DURATION_COMMIT: &str = "bolt.app.commit.duration";

pub(crate) fn init_metrics() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        describe_counter!(LABEL_SUCCESSFUL_TX, "Number of successful transactions");

        describe_counter!(LABEL_FAILED_TX, "Number of failed transactions");

        describe_counter!(LABEL_PROCESSED_MSGS, "Number of processed messages");

        describe_counter!(LABEL_PROCESSED_QUERIES, "Number of processed queries");

        describe_histogram!(LABEL_TX_PER_BLOCK, "Number of transactions per block");

        describe_histogram!(LABEL_DURATION_BLOCK, "Duration of finalized block");

        describe_histogram!(LABEL_DURATION_TX, "Duration of a transaction");

        describe_histogram!(
            LABEL_DURATION_PREPARE_PROPOSAL,
            "Duration of prepare proposal"
        );

        describe_histogram!(LABEL_DURATION_COMMIT, "Duration of commit");
    });
}
