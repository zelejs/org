use axum::{
    extract::{Query, State},
    Json,
};

use crate::{
    api::state::AppState,
    models::{
        error::AppError,
        response::Tip,
        tenant::TenantQuery,
    },
};

pub async fn page_tenants(
    State(state): State<AppState>,
    Query(query): Query<TenantQuery>,
) -> Result<Json<Tip<crate::models::org::PageResult<crate::models::tenant::SysTenantDTO>>>, AppError> {
    let page = state.org_service.page_tenants(query).await?;
    Ok(Json(Tip::success(page)))
}
