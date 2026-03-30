use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use org_rust::{
    api::{self, state::AppState},
    services::{ExtOrgService, OrgService},
};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

/// Org-rust HTTP server for organization tree management
#[derive(Parser, Debug)]
#[command(name = "org-rust")]
#[command(about = "Organization tree management HTTP server", long_about = None)]
struct ServerArgs {
    /// Host to listen on
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen on
    #[arg(long, default_value = "8082")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "org_rust=info,tower_http=info".to_string()),
        )
        .init();

    let args = ServerArgs::parse();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@127.0.0.1:5432/enrollment".to_string());

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

    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;
    info!("org-rust listening on {}", addr);
    info!("health: http://{}/health", addr);
    info!("org api: http://{}/api/adm/org/tree", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
