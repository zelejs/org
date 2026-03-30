use std::{net::SocketAddr, sync::Arc};
use std::env;

use clap::Parser;
use org_rust::{
    api::{self, state::AppState},
    services::{ExtOrgService, OrgService},
};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "org_rust=info,tower_http=info".to_string()),
        )
        .init();

    // Check if running in CLI mode
    let args: Vec<String> = env::args().collect();

    // If first argument is a known CLI command, run in CLI mode
    let is_cli = args.len() > 1 && matches!(
        args[1].as_str(),
        "org" | "org-cli" | "init" | "show" | "roots" | "insert" | "remove" | "export" | "import" | "--help" | "-h"
    );

    if is_cli {
        return run_cli().await;
    }

    // Otherwise, run in server mode (default)
    run_server().await
}

async fn run_server() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@127.0.0.1:5432/enrollment".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8082".to_string())
        .parse::<u16>()?;

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    let org_service = OrgService::new(pool.clone());
    let ext_org_service = ExtOrgService::new(pool.clone(), org_service.clone());

    let state = AppState {
        pool: pool.clone(),
        org_service: Arc::new(org_service),
        ext_org_service: Arc::new(ext_org_service),
    };
    let app = api::create_router(state).layer(
        tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any),
    );

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    info!("org-rust listening on {}", addr);
    info!("health: http://{}/health", addr);
    info!("org api: http://{}/api/adm/org/tree", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn run_cli() -> anyhow::Result<()> {
    let cli = org_rust::cli::Cli::try_parse()?;
    org_rust::cli::run(cli).await
}
