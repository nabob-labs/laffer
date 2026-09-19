use {
    crate::Codes,
    velox_types::config::Hyperlane,
    bolt::{Binary, ContractBuilder, ContractWrapper, Empty, StdResult},
    bolt_vm_rust::RustVm,
};

/// Get the binary codes for Velox smart contracts, for use in building the
/// genesis state.
pub trait GenesisCodes {
    type Code: Clone + Into<Binary>;

    fn genesis_codes() -> Codes<Self::Code>;
}

impl GenesisCodes for RustVm {
    type Code = ContractWrapper;

    fn genesis_codes() -> Codes<ContractWrapper> {
        let account_factory = ContractBuilder::new(Box::new(velox_account_factory::instantiate))
            .with_execute(Box::new(velox_account_factory::execute))
            .with_query(Box::new(velox_account_factory::query))
            .with_authenticate(Box::new(velox_account_factory::authenticate))
            .build();

        // Previously this was the multisig account code hash, now removed.
        let _account_multi = ContractBuilder::new(Box::new(|_, _: Empty| -> StdResult<_> {
            unreachable!("the multisig contract has been deleted");
        }))
        .build();

        let account = ContractBuilder::new(Box::new(velox_account::instantiate))
            .with_authenticate(Box::new(velox_account::authenticate))
            .with_receive(Box::new(velox_account::receive))
            .with_query(Box::new(velox_account::query))
            .build();

        let bank = ContractBuilder::new(Box::new(velox_bank::instantiate))
            .with_execute(Box::new(velox_bank::execute))
            .with_query(Box::new(velox_bank::query))
            .with_bank_execute(Box::new(velox_bank::bank_execute))
            .with_bank_query(Box::new(velox_bank::bank_query))
            .build();

        let dex = ContractBuilder::new(Box::new(velox_dex::instantiate))
            .with_execute(Box::new(velox_dex::execute))
            .with_cron_execute(Box::new(velox_dex::cron_execute))
            .with_query(Box::new(velox_dex::query))
            .with_reply(Box::new(velox_dex::reply))
            .build();

        let gateway = ContractBuilder::new(Box::new(velox_gateway::instantiate))
            .with_execute(Box::new(velox_gateway::execute))
            .with_query(Box::new(velox_gateway::query))
            .with_cron_execute(Box::new(velox_gateway::cron_execute))
            .build();

        let ism = ContractBuilder::new(Box::new(hyperlane_ism::instantiate))
            .with_execute(Box::new(hyperlane_ism::execute))
            .with_query(Box::new(hyperlane_ism::query))
            .build();

        let mailbox = ContractBuilder::new(Box::new(hyperlane_mailbox::instantiate))
            .with_execute(Box::new(hyperlane_mailbox::execute))
            .with_query(Box::new(hyperlane_mailbox::query))
            .build();

        let va = ContractBuilder::new(Box::new(hyperlane_va::instantiate))
            .with_execute(Box::new(hyperlane_va::execute))
            .with_query(Box::new(hyperlane_va::query))
            .build();

        let oracle = ContractBuilder::new(Box::new(velox_oracle::instantiate))
            .with_execute(Box::new(velox_oracle::execute))
            .with_authenticate(Box::new(velox_oracle::authenticate))
            .with_query(Box::new(velox_oracle::query))
            .with_reply(Box::new(velox_oracle::reply))
            .build();

        let taxman = ContractBuilder::new(Box::new(velox_taxman::instantiate))
            .with_execute(Box::new(velox_taxman::execute))
            .with_query(Box::new(velox_taxman::query))
            .with_withhold_fee(Box::new(velox_taxman::withhold_fee))
            .with_finalize_fee(Box::new(velox_taxman::finalize_fee))
            .build();

        let vesting = ContractBuilder::new(Box::new(velox_vesting::instantiate))
            .with_execute(Box::new(velox_vesting::execute))
            .with_query(Box::new(velox_vesting::query))
            .build();

        let warp = ContractBuilder::new(Box::new(velox_warp::instantiate))
            .with_execute(Box::new(velox_warp::execute))
            .with_query(Box::new(velox_warp::query))
            .build();

        let perps = ContractBuilder::new(Box::new(velox_perps::instantiate))
            .with_execute(Box::new(velox_perps::execute))
            .with_query(Box::new(velox_perps::query))
            .with_cron_execute(Box::new(velox_perps::cron_execute))
            .build();

        #[cfg(feature = "metrics")]
        {
            velox_dex::metrics::init_metrics();
            velox_oracle::metrics::init_metrics();
            velox_perps::metrics::init_metrics();
            velox_taxman::metrics::init_metrics();
            // TODO: add other contracts that emit metrics
        }

        Codes {
            account,
            account_factory,
            bank,
            dex,
            gateway,
            hyperlane: Hyperlane { ism, mailbox, va },
            oracle,
            perps,
            taxman,
            vesting,
            warp,
        }
    }
}

// TODO: implement `GenesisCodes` for `WasmVm` and `HybridVm`.
// For now, we don't want to include them here because wasmer v6 has changed to
// the Business Source License. We want to make sure anything in the `velox/`
// directory does NOT have dependency on it.
