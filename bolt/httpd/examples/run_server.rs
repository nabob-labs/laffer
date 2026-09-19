use {
    clap::Parser,
    bolt_httpd::{context::Context, graphql, server},
    bolt_types::HttpdConfig,
    std::sync::Arc,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP address to bind to
    #[arg(long, default_value = "127.0.0.1")]
    ip: String,

    /// Port to bind to
    #[arg(long, default_value = "8080")]
    port: u16,

    /// CORS allowed origin
    #[arg(long)]
    cors_origin: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a mock bolt app for demonstration
    // In a real application, you would create an actual bolt app instance
    let bolt_app = Arc::new(MockBoltApp);
    let context = Context::new(bolt_app);

    let httpd_config = HttpdConfig {
        ip: args.ip,
        port: args.port,
        cors_allowed_origin: args.cors_origin,
        ..Default::default()
    };

    println!(
        "Starting HTTP server on {}:{}",
        httpd_config.ip, httpd_config.port
    );

    let shutdown_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    server::run_server(
        &httpd_config,
        context,
        server::config_app,
        graphql::build_schema,
        shutdown_flag,
    )
    .await?;

    Ok(())
}

// Mock implementation for demonstration
struct MockBoltApp;

#[async_trait::async_trait]
impl bolt_httpd::traits::QueryApp for MockBoltApp {
    async fn query_app(
        &self,
        _raw_req: bolt_types::Query,
        _height: Option<u64>,
    ) -> bolt_app::AppResult<(bolt_types::QueryResponse, u64)> {
        Ok((
            bolt_types::QueryResponse::AppConfig(bolt_types::Json::null()),
            0,
        ))
    }

    async fn query_store(
        &self,
        _key: &[u8],
        _height: Option<u64>,
        _prove: bool,
    ) -> bolt_app::AppResult<(Option<Vec<u8>>, Option<Vec<u8>>, u64)> {
        Ok((Some(b"mock_value".to_vec()), None, 0))
    }

    async fn simulate(
        &self,
        _unsigned_tx: bolt_types::UnsignedTx,
    ) -> bolt_app::AppResult<bolt_types::TxOutcome> {
        Ok(bolt_types::TxOutcome {
            gas_limit: 0,
            gas_used: 0,
            result: Ok(()),
            events: bolt_types::TxEvents::default(),
        })
    }

    async fn chain_id(&self) -> bolt_app::AppResult<String> {
        Ok("test-chain".to_string())
    }

    async fn last_finalized_block(&self) -> bolt_app::AppResult<bolt_types::BlockInfo> {
        Ok(bolt_types::BlockInfo {
            height: 1,
            timestamp: bolt_types::Timestamp::from_seconds(0),
            hash: bolt_types::Hash256::ZERO,
        })
    }
}
