use {
    velox_hyperlane_types::{IncrementalMerkleTree, mailbox::Config},
    velox_primitives::Hash256,
    velox_storage::{Counter, Item, Set},
};

pub const CONFIG: Item<Config> = Item::new("config");

pub const NONCE: Counter<u32> = Counter::new("nonce", 0, 1);

pub const MERKLE_TREE: Item<IncrementalMerkleTree> = Item::new("merkle_tree");

pub const DELIVERIES: Set<Hash256> = Set::new("delivery");
