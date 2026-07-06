use {
    crate::Codes,
    velox_primitives::{Binary, Empty, StdResult},
    velox_types::config::Hyperlane,
    velox_vm_rust::{ContractBuilder, ContractWrapper, RustVm},
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

        // Previously this was the DEX (spot exchange) code hash, now removed.
        let _dex = ContractBuilder::new(Box::new(|_, _: Empty| -> StdResult<_> {
            unreachable!("the dex contract has been deleted");
        }))
        .build();

        let gateway = ContractBuilder::new(Box::new(velox_gateway::instantiate))
            .with_execute(Box::new(velox_gateway::execute))
            .with_query(Box::new(velox_gateway::query))
            .with_cron_execute(Box::new(velox_gateway::cron_execute))
            .build();

        let ism = ContractBuilder::new(Box::new(velox_hyperlane_ism::instantiate))
            .with_execute(Box::new(velox_hyperlane_ism::execute))
            .with_query(Box::new(velox_hyperlane_ism::query))
            .build();

        let mailbox = ContractBuilder::new(Box::new(velox_hyperlane_mailbox::instantiate))
            .with_execute(Box::new(velox_hyperlane_mailbox::execute))
            .with_query(Box::new(velox_hyperlane_mailbox::query))
            .build();

        let va = ContractBuilder::new(Box::new(velox_hyperlane_va::instantiate))
            .with_execute(Box::new(velox_hyperlane_va::execute))
            .with_query(Box::new(velox_hyperlane_va::query))
            .build();

        let oracle = ContractBuilder::new(Box::new(velox_oracle::instantiate))
            .with_execute(Box::new(velox_oracle::execute))
            .with_authenticate(Box::new(velox_oracle::authenticate))
            .with_query(Box::new(velox_oracle::query))
            .build();

        // Previously this was the taxman code hash, now removed. Gas fees are
        // handled directly by the state machine.
        let _taxman = ContractBuilder::new(Box::new(|_, _: Empty| -> StdResult<_> {
            unreachable!("the taxman contract has been deleted");
        }))
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
            .with_authenticate(Box::new(velox_perps::authenticate))
            .build();

        #[cfg(feature = "metrics")]
        {
            velox_oracle::metrics::init_metrics();
            velox_perps::metrics::init_metrics();
            // TODO: add other contracts that emit metrics
        }

        Codes {
            account,
            account_factory,
            bank,
            gateway,
            hyperlane: Hyperlane { ism, mailbox, va },
            oracle,
            perps,
            vesting,
            warp,
        }
    }
}

// TODO: implement `GenesisCodes` for `WasmVm` and `HybridVm`.
// For now, we don't want to include them here because wasmer v6 has changed to
// the Business Source License. We want to make sure anything in the `velox/`
// directory does NOT have dependency on it.
