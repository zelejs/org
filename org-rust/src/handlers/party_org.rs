use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    api::state::AppState,
    models::{
        error::AppError,
        party_org::{PartyOrgRelation, PartyOrgRequest},
        response::Tip,
    },
};

#[derive(Debug, Deserialize)]
pub struct DeletePartyOrgQuery {
    pub id: i64,
}

pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<PartyOrgRequest>,
) -> Result<Json<Tip<i64>>, AppError> {
    let id = state.party_org_service.add(req).await?;
    Ok(Json(Tip::success(id)))
}

pub async fn delete(
    State(state): State<AppState>,
    Query(query): Query<DeletePartyOrgQuery>,
) -> Result<Json<Tip<i64>>, AppError> {
    let id = state.party_org_service.delete(query.id).await?;
    Ok(Json(Tip::success(id)))
}

pub async fn update(
    State(state): State<AppState>,
    Json(req): Json<PartyOrgRequest>,
) -> Result<Json<Tip<i64>>, AppError> {
    let id = state.party_org_service.update(req).await?;
    Ok(Json(Tip::success(id)))
}

pub async fn list(State(state): State<AppState>) -> Result<Json<Tip<Vec<PartyOrgRelation>>>, AppError> {
    let data = state.party_org_service.list().await?;
    Ok(Json(Tip::success(data)))
}

pub async fn sync(State(state): State<AppState>) -> Result<Json<Tip<i64>>, AppError> {
    let data = state.party_org_service.sync().await?;
    Ok(Json(Tip::success(data)))
}

