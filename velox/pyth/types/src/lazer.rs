use {
    pyth_lazer_protocol::{
        api::{Channel, MarketSession as LazerMarketSession},
        message::LeEcdsaMessage as LazerLeEcdsaMessage,
    },
    velox_primitives::{Binary, ByteArray, Inner, NonEmpty},
};

pub type PythId = u32;

pub type PriceUpdate = NonEmpty<Vec<LeEcdsaMessage>>;

#[velox_primitives::derive(Serde)]
pub struct PythLazerSubscriptionDetails {
    pub id: PythId,
    pub channel: Channel,
}

/// LE-ECDSA format envelope.
#[velox_primitives::derive(Serde, Borsh)]
pub struct LeEcdsaMessage {
    pub payload: Binary,
    pub signature: ByteArray<64>,
    pub recovery_id: u8,
}

impl From<LeEcdsaMessage> for LazerLeEcdsaMessage {
    fn from(message: LeEcdsaMessage) -> Self {
        LazerLeEcdsaMessage {
            payload: message.payload.into_inner(),
            signature: message.signature.into_inner(),
            recovery_id: message.recovery_id,
        }
    }
}

impl From<LazerLeEcdsaMessage> for LeEcdsaMessage {
    fn from(message: LazerLeEcdsaMessage) -> Self {
        LeEcdsaMessage {
            payload: message.payload.into(),
            signature: message.signature.into(),
            recovery_id: message.recovery_id,
        }
    }
}

/// A coarse classification of a Pyth Lazer feed's market session.
///
/// Upstream's `pyth_lazer_protocol::api::MarketSession` has 5 variants
/// (`Regular`, `PreMarket`, `PostMarket`, `OverNight`, `Closed`). Velox
/// only needs to know whether trading is currently in the regular session;
/// every non-regular state collapses into a single `Other` variant.
#[velox_primitives::derive(Serde, Borsh)]
#[derive(Copy)]
pub enum MarketSession {
    Regular,
    Other,
}

impl From<LazerMarketSession> for MarketSession {
    fn from(upstream: LazerMarketSession) -> Self {
        match upstream {
            LazerMarketSession::Regular => MarketSession::Regular,
            LazerMarketSession::PreMarket
            | LazerMarketSession::PostMarket
            | LazerMarketSession::OverNight
            | LazerMarketSession::Closed => MarketSession::Other,
        }
    }
}
