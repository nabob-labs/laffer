use {
    crate::vesting::{Position, Schedule},
    bolt::{Addr, Duration, Uint128},
    std::collections::BTreeMap,
};

#[bolt::derive(Serde)]
pub struct PositionResponse {
    pub position: Position,
    pub claimable: Uint128,
}

#[bolt::derive(Serde)]
pub struct InstantiateMsg {
    pub unlocking_cliff: Duration,
    pub unlocking_period: Duration,
}

#[bolt::derive(Serde)]
pub enum ExecuteMsg {
    /// Create a vesting position for a user with the given schedule.
    ///
    /// Sender must be the chain owner, and attach a non-zero amount of Velox
    /// token and nothing else.
    Create { user: Addr, schedule: Schedule },
    /// Terminate a user's vesting position.
    ///
    /// Sender must be the chain owner.
    Terminate { user: Addr },
    /// Claim the withdrawable amount from the vesting position.
    ///
    /// Sender must have a non-zero amount of claimable tokens.
    Claim {},
}

#[bolt::derive(Serde, QueryRequest)]
pub enum QueryMsg {
    /// Query a single vesting position by user address.
    #[returns(PositionResponse)]
    Position { user: Addr },
    /// Enumerate all vesting positions.
    #[returns(BTreeMap<Addr, PositionResponse>)]
    Positions {
        start_after: Option<Addr>,
        limit: Option<u32>,
    },
}
