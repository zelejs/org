use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use sqlx::query_scalar;

use crate::api::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let healthy = query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .is_ok();

    if healthy {
        (
            StatusCode::OK,
            Json(HealthResponse {
                status: "healthy".to_string(),
            }),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "unhealthy".to_string(),
            }),
        )
    }
}

