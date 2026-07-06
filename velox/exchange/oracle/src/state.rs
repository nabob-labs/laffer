use {
    velox_primitives::{Denom, Timestamp},
    velox_pyth_types::PythId,
    velox_storage::{Map, Serde},
    velox_types::oracle::{Price, PriceConfig},
};

pub const PRICE_SOURCES: Map<&Denom, PriceConfig, Serde> = Map::new("price_source");

pub const PYTH_TRUSTED_SIGNERS: Map<&[u8], Timestamp> = Map::new("pyth_trusted_signer");

pub const PYTH_PRICES: Map<PythId, Price> = Map::new("pyth_price");
