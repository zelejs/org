use clap::Parser;
use org_rust::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "org_rust=info".to_string()),
        )
        .init();

    let cli = Cli::try_parse()?;
    org_rust::cli::run(cli).await
}
