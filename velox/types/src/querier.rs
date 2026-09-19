use {
    crate::config::AppConfig,
    bolt::{Addr, Coins, Querier, QuerierExt, StdResult},
};

/// An extension trait that adds some useful, Velox-specific methods to
/// [`QuerierWrapper`](bolt::QuerierWrapper).
pub trait VeloxQuerier {
    fn query_velox_config(&self) -> StdResult<AppConfig>;

    fn query_account_factory(&self) -> StdResult<Addr> {
        self.query_velox_config()
            .map(|app_cfg| app_cfg.addresses.account_factory)
    }

    fn query_dex(&self) -> StdResult<Addr> {
        self.query_velox_config()
            .map(|app_cfg| app_cfg.addresses.dex)
    }

    fn query_gateway(&self) -> StdResult<Addr> {
        self.query_velox_config()
            .map(|app_cfg| app_cfg.addresses.gateway)
    }

    fn query_oracle(&self) -> StdResult<Addr> {
        self.query_velox_config()
            .map(|app_cfg| app_cfg.addresses.oracle)
    }

    fn query_perps(&self) -> StdResult<Addr> {
        self.query_velox_config()
            .map(|app_cfg| app_cfg.addresses.perps)
    }

    fn query_warp(&self) -> StdResult<Addr> {
        self.query_velox_config()
            .map(|app_cfg| app_cfg.addresses.warp)
    }

    fn query_minimum_deposit(&self) -> StdResult<Coins> {
        self.query_velox_config()
            .map(|app_cfg| app_cfg.minimum_deposit)
    }
}

impl<Q> VeloxQuerier for Q
where
    Q: Querier,
{
    fn query_velox_config(&self) -> StdResult<AppConfig> {
        self.query_app_config()
    }
}
