use {
    crate::{Addr32, mailbox::Domain},
    bolt::{Addr, HexBinary},
};

#[bolt::derive(Serde)]
pub enum ExecuteMsg {
    Recipient(RecipientMsg),
}

#[bolt::derive(Serde)]
pub enum RecipientMsg {
    Handle {
        origin_domain: Domain,
        sender: Addr32,
        body: HexBinary,
    },
}

#[bolt::derive(Serde, QueryRequest)]
pub enum QueryMsg {
    #[returns(RecipientQueryResponse)]
    Recipient(RecipientQuery),
}

#[bolt::derive(Serde)]
pub enum RecipientQuery {
    /// Return the ISM this recipient would like to use for verifying incoming
    /// messages.
    /// `None` if the recipient would like to defer to the default ISM.
    InterchainSecurityModule {},
}

#[bolt::derive(Serde)]
pub enum RecipientQueryResponse {
    InterchainSecurityModule(Option<Addr>),
}

impl RecipientQueryResponse {
    pub fn into_interchain_security_module(self) -> Option<Addr> {
        match self {
            RecipientQueryResponse::InterchainSecurityModule(ism) => ism,
        }
    }
}
