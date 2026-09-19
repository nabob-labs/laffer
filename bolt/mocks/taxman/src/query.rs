use {
    crate::{CONFIG, Config},
    bolt_types::{StdResult, Storage},
};

pub fn query_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}
