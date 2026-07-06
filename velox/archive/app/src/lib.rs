//! Archive app orchestration.
//!
//! Wires a single [`BlockSource`](velox_archive_block_source::BlockSource),
//! the shared [`Committer`](velox_archive_projection::Committer), and a
//! fixed set of [`Projection`](velox_archive_projection::Projection)s,
//! drives them in cooperating tasks, and surfaces failures.

mod app;
mod committer;
mod metrics;
mod projection_loop;

pub use {
    crate::metrics::init_metrics, app::App, committer::PgChCommitter,
    projection_loop::projection_loop,
};
