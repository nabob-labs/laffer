use bolt::{Addr, Coins};

/// An event indicating a user has sent a transfer of coins.
#[bolt::derive(Serde)]
#[bolt::event("sent")]
pub struct Sent {
    pub user: Addr,
    pub to: Addr,
    pub coins: Coins,
}

/// An event indicating a user has received a transfer of coins.
#[bolt::derive(Serde)]
#[bolt::event("received")]
pub struct Received {
    pub user: Addr,
    pub from: Addr,
    pub coins: Coins,
}

/// An event indicating a user has received newly minted coins.
#[bolt::derive(Serde)]
#[bolt::event("minted")]
pub struct Minted {
    pub user: Addr,
    /// The account that initiated the minting.
    ///
    /// E.g. `user` is providing liquidity to the Velox DEX contract. In this
    /// case, the DEX contract will initiate the minting of LP tokens to `user`'s
    /// wallet.
    pub minter: Addr,
    pub coins: Coins,
}

/// An event indicating a user's coins have been burned from his wallet.
#[bolt::derive(Serde)]
#[bolt::event("burned")]
pub struct Burned {
    pub user: Addr,
    pub burner: Addr,
    pub coins: Coins,
}

/// An event indicating a transfer has been attempted of which the recipient
/// account doesn't exist (i.e. an "orphaned transfer").
///
/// The funds are temporarily held in the Velox bank contract, and can be
/// claimed either by the sender or the recipient (once it's been created).
#[bolt::derive(Serde)]
#[bolt::event("transfer_orphaned")]
pub struct TransferOrphaned {
    pub from: Addr,
    pub to: Addr,
    pub coins: Coins,
}
