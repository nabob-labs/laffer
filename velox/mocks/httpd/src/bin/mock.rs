use {
    velox_genesis::GenesisOption,
    velox_mock_httpd::{BlockCreation, Error, TestOption},
    velox_testing::Preset,
};

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<(), Error> {
    velox_mock_httpd::run(
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
