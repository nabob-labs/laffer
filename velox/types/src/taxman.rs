use {
    crate::account_factory::UserIndex,
    bolt::{Addr, Coins, Denom, Timestamp, Udec128, Udec128_6},
    std::collections::BTreeMap,
};

#[bolt::derive(Serde, Borsh)]
pub struct Config {
    pub fee_denom: Denom,
    /// Units of the fee token for each unit of gas consumed.
    pub fee_rate: Udec128,
}

#[bolt::derive(Serde)]
#[derive(Copy)]
pub enum FeeType {
    /// Gas Fee.
    Gas,
    /// Protocol fee for trading in Velox DEX.
    ///
    /// Not to be confused with liquidity fee, which is paid to liquidity
    /// providers when using Velox DEX's instant swap feature.
    Trade,
    /// Fee for bridging assets out of Velox chain.
    Withdraw,
}

impl FeeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FeeType::Gas => "gas",
            FeeType::Trade => "trade",
            FeeType::Withdraw => "withdraw",
        }
    }
}

#[bolt::derive(Serde)]
pub struct InstantiateMsg {
    pub config: Config,
}

#[bolt::derive(Serde)]
pub enum ExecuteMsg {
    /// Update the fee configurations.
    /// Can only be called by the chain's owner.
    Configure { new_cfg: Config },
    /// Forward protocol fee to the taxman.
    Pay {
        #[serde(rename = "type")]
        ty: FeeType,
        payments: BTreeMap<Addr, Coins>,
    },
    /// Report trading volumes of users.
    /// Can only be called by the spot and perp DEX contracts.
    ReportVolumes(BTreeMap<Addr, Udec128_6>),
}

#[bolt::derive(Serde, QueryRequest)]
pub enum QueryMsg {
    /// Query the fee configurations.
    #[returns(Config)]
    Config {},
    /// Returns the trading volume of a user since the specified timestamp.
    #[returns(Udec128)]
    VolumeByUser {
        /// The user to query trading volume for.
        user: UserIndex,
        /// The start timestamp to query trading volume for. If not provided,
        /// user's total trading volume since genesis will be returned.
        since: Option<Timestamp>,
    },
}

#[bolt::derive(Serde)]
#[bolt::event("receive_fee")]
pub struct ReceiveFee {
    /// The Velox smart contract that handled this fee.
    pub handler: Addr,
    pub user: Addr,
    #[serde(rename = "type")]
    pub ty: FeeType,
    pub amount: Coins,
}
