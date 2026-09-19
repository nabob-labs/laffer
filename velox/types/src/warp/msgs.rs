use {
    crate::gateway::bridge::BridgeMsg,
    bolt::Addr,
    hyperlane_types::recipients::{RecipientMsg, RecipientQuery, RecipientQueryResponse},
};

#[bolt::derive(Serde)]
pub struct InstantiateMsg {
    /// Address of the mailbox contract.
    pub mailbox: Addr,
}

#[bolt::derive(Serde)]
pub enum ExecuteMsg {
    /// Required Hyperlane recipient interface.
    Recipient(RecipientMsg),
    /// Required Velox Gateway interface.
    Bridge(BridgeMsg),
}

#[bolt::derive(Serde, QueryRequest)]
pub enum QueryMsg {
    /// Query the address of the mailbox contract.
    #[returns(Addr)]
    Mailbox {},
    /// Required Hyperlane recipient interface.
    #[returns(RecipientQueryResponse)]
    Recipient(RecipientQuery),
}
