use {
    crate::gateway::bridge::BridgeMsg,
    velox_hyperlane_types::recipients::{RecipientMsg, RecipientQuery, RecipientQueryResponse},
    velox_primitives::Addr,
};

#[velox_primitives::derive(Serde)]
pub struct InstantiateMsg {
    /// Address of the mailbox contract.
    pub mailbox: Addr,
}

#[velox_primitives::derive(Serde)]
pub enum ExecuteMsg {
    /// Required Hyperlane recipient interface.
    Recipient(RecipientMsg),
    /// Required Velox Gateway interface.
    Bridge(BridgeMsg),
}

#[velox_primitives::derive(Serde, QueryRequest)]
pub enum QueryMsg {
    /// Query the address of the mailbox contract.
    #[returns(Addr)]
    Mailbox {},
    /// Required Hyperlane recipient interface.
    #[returns(RecipientQueryResponse)]
    Recipient(RecipientQuery),
}
