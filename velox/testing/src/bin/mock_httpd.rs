use {
    velox_genesis::GenesisOption,
    velox_testing::{BlockCreation, HttpdError, Preset, TestOption},
};

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<(), HttpdError> {
    velox_testing::mock_httpd_run(
        8080,
        BlockCreation::OnBroadcast,
        None,
        TestOption {
            chain_id: "localvelox-1".to_string(),
            ..Preset::preset_test()
        },
        GenesisOption::preset_test(),
        None,
    )
    .await
}
