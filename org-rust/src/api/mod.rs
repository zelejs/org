pub mod state;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::{ext_org, health, org, tenant};
use crate::api::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let org_router = Router::new()
        .route("/", get(org::page_orgs))
        .route("/tree", get(org::tree_orgs))
        .route("/tenant/tree", get(org::tree_orgs_with_tenant))
        .route("/:id", get(org::get_org))
        .route("/:id", put(org::update_org))
        .route("/:id", delete(org::delete_org))
        .route("/:id/children", post(org::create_org_child));

    let ext_org_router = Router::new()
        .route("/add", post(ext_org::add))
        .route("/delete", delete(ext_org::delete))
        .route("/update", put(ext_org::update))
        .route("/list", get(ext_org::list))
        .route("/sync", post(ext_org::sync));

    let tenant_router = Router::new()
        .route("/", get(tenant::page_tenants));

    Router::new()
        .route("/health", get(health::health_check))
        .nest("/api/adm/org", org_router)
        .nest("/api/adm/tenant", tenant_router)
        .nest("/api/adm/sys/extOrg", ext_org_router)
        .with_state(state)
}

