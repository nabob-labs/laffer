use {
    crate::{
        account_factory::{AccountIndex, UserIndex, Username},
        auth::Key,
    },
    velox_primitives::{Addr, Hash256},
};

/// An event indicating a new user has registered.
#[velox_primitives::derive(Serde)]
#[velox_primitives::event("user_registered")]
pub struct UserRegistered {
    pub user_index: UserIndex,
    pub key_hash: Hash256,
    pub key: Key,
}

/// An event indicating a new address has been created.
#[velox_primitives::derive(Serde)]
#[velox_primitives::event("account_registered")]
pub struct AccountRegistered {
    pub account_index: AccountIndex,
    pub address: Addr,
    pub owner: UserIndex,
}

/// An event indicating a username begins to own an account.
#[velox_primitives::derive(Serde)]
#[velox_primitives::event("account_owned")]
pub struct AccountOwned {
    pub user_index: UserIndex,
    pub address: Addr,
}

/// An event indicating a username begins to own a key.
#[velox_primitives::derive(Serde)]
#[velox_primitives::event("key_owned")]
pub struct KeyOwned {
    pub user_index: UserIndex,
    pub key_hash: Hash256,
    pub key: Key,
}

/// An event indicating a username ceases to own a key.
#[velox_primitives::derive(Serde)]
#[velox_primitives::event("key_disowned")]
pub struct KeyDisowned {
    pub user_index: UserIndex,
    pub key_hash: Hash256,
}

/// An event indicating a user has set a custom username.
#[velox_primitives::derive(Serde)]
#[velox_primitives::event("username_updated")]
pub struct UsernameUpdated {
    pub user_index: UserIndex,
    pub username: Username,
}
