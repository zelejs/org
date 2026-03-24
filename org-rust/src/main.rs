use std::{net::SocketAddr, sync::Arc};

use org_rust::{
    api::{self, state::AppState},
    services::{OrgService, PartyOrgService},
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
    let party_service = PartyOrgService::new(pool.clone(), org_service.clone());

    let state = AppState {
        pool: pool.clone(),
        org_service: Arc::new(org_service),
        party_org_service: Arc::new(party_service),
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

