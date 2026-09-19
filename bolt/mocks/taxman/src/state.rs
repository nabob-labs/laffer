use {crate::Config, bolt_math::Uint128, bolt_storage::Item};

pub const CONFIG: Item<Config> = Item::new("config");

pub const WITHHELD_FEE: Item<(Config, Uint128)> = Item::new("withheld_fee");
