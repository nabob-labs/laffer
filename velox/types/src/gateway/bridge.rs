use {super::Remote, bolt::Uint128, hyperlane_types::Addr32};

/// Each bridge contract must implement this execute API.
#[bolt::derive(Serde)]
pub enum ExecuteMsg {
    // NOTE: Bridge contract must ensure only the Gateway contract can call this.
    Bridge(BridgeMsg),
}

#[bolt::derive(Serde)]
pub enum BridgeMsg {
    TransferRemote {
        remote: Remote,
        amount: Uint128,
        recipient: Addr32,
    },
}
