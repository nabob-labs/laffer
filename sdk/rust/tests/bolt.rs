use {
    assertor::*,
    velox_genesis::GenesisOption,
    velox_mock_httpd::{BlockCreation, TestOption, get_mock_socket_addr, wait_for_server_ready},
    velox_sdk::HttpClient,
    velox_testing::Preset,
    velox_types::config::AppConfig,
    bolt::QueryClientExt,
};

#[tokio::test]
async fn graphql_returns_config() -> anyhow::Result<()> {
    let port = get_mock_socket_addr();

    // Spawn server in separate thread with its own runtime
    let _server_handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            tracing::info!("Starting mock HTTP server on port {port}");

            if let Err(error) = velox_mock_httpd::run(
                port,
                BlockCreation::OnBroadcast,
                None,
                TestOption::default(),
                GenesisOption::preset_test(),
                None,
            )
            .await
            {
                // Using println! so even without `setup_tracing_subscriber` we can see the error
                println!("Error running mock HTTP server: {error}");
            }
        });
    });

    wait_for_server_ready(port).await?;

    let client = HttpClient::new(format!("http://localhost:{port}"))?;
    let res = client.query_app_config::<AppConfig>(None).await;

    assert_that!(res).is_ok();

    Ok(())
}
