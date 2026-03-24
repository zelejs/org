use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    api::state::AppState,
    models::{
        error::AppError,
        org::{CreateOrgRequest, OrgListQuery, OrgTreeQuery, UpdateOrgRequest},
        response::Tip,
    },
    services::RequestContext,
};

pub async fn create_org_child(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    ctx: RequestContext,
    Json(req): Json<CreateOrgRequest>,
) -> Result<Json<Tip<i64>>, AppError> {
    let org_id = state.org_service.create_child(id, req, &ctx).await?;
    Ok(Json(Tip::success(org_id)))
}

pub async fn delete_org(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Tip<i64>>, AppError> {
    let deleted = state.org_service.delete_node(id).await?;
    Ok(Json(Tip::success(deleted)))
}

pub async fn update_org(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateOrgRequest>,
) -> Result<Json<Tip<i64>>, AppError> {
    let updated = state.org_service.update_node(id, req).await?;
    Ok(Json(Tip::success(updated)))
}

pub async fn get_org(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Tip<crate::models::org::SysOrg>>, AppError> {
    let org = state.org_service.get_org(id).await?;
    Ok(Json(Tip::success(org)))
}

pub async fn page_orgs(
    State(state): State<AppState>,
    Query(query): Query<OrgListQuery>,
) -> Result<Json<Tip<crate::models::org::PageResult<crate::models::org::SysOrg>>>, AppError> {
    let page = state.org_service.page_orgs(query).await?;
    Ok(Json(Tip::success(page)))
}

pub async fn tree_orgs(
    State(state): State<AppState>,
    ctx: RequestContext,
    Query(query): Query<OrgTreeQuery>,
) -> Result<Json<Tip<crate::models::org::TreeTop>>, AppError> {
    let tree = state.org_service.tree(query, &ctx).await?;
    Ok(Json(Tip::success(tree)))
}

