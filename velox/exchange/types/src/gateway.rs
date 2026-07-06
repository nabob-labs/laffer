//! Gateway is Velox's bridge aggregator contract. Velox will be equipped with
//! the following bridge protocols:
//!
//! - Hyperlane;
//! - Velox's own bitcoin bridge (name TBD).
//!
//! There are logics common to both protocols. Instead of duplicating them in
//! each protocol, they are extracted into the Gateway contract. These logics
//! include:
//!
//! - minting and burning of tokens;
//! - alloying;
//! - withdraw fee;
//! - withdraw rate limit.

pub mod bridge;
mod msgs;
mod origin;
mod remote;

pub use {
    msgs::*,
    origin::*,
    remote::*,
    velox_hyperlane_types::{Addr32, mailbox::Domain},
};

use {
    std::sync::LazyLock,
    velox_math::Udec128,
    velox_primitives::{Bounded, Part, ZeroInclusiveOneExclusive},
};

pub type RateLimit = Bounded<Udec128, ZeroInclusiveOneExclusive>;

pub static NAMESPACE: LazyLock<Part> = LazyLock::new(|| Part::new_unchecked("bridge"));
