use {super::Remote, velox_hyperlane_types::Addr32, velox_math::Uint128};

/// Each bridge contract must implement this execute API.
#[velox_primitives::derive(Serde)]
pub enum ExecuteMsg {
    // NOTE: Bridge contract must ensure only the Gateway contract can call this.
    Bridge(BridgeMsg),
}

#[velox_primitives::derive(Serde)]
pub enum BridgeMsg {
    TransferRemote {
        remote: Remote,
        amount: Uint128,
        recipient: Addr32,
    },
}
