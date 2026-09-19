use {crate::WasmVm, bolt_testing::TestVm, bolt_types::Binary};

impl TestVm for WasmVm {
    fn default_account_code() -> Binary {
        let code: &[u8] = include_bytes!("../testdata/bolt_mock_account.wasm");
        code.into()
    }

    fn default_bank_code() -> Binary {
        let code: &[u8] = include_bytes!("../testdata/bolt_mock_bank.wasm");
        code.into()
    }

    fn default_taxman_code() -> Binary {
        let code: &[u8] = include_bytes!("../testdata/bolt_mock_taxman.wasm");
        code.into()
    }
}
