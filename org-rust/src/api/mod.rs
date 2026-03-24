pub mod state;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::{health, org, party_org};
use crate::api::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let org_router = Router::new()
        .route("/", get(org::page_orgs))
        .route("/tree", get(org::tree_orgs))
        .route("/:id", get(org::get_org))
        .route("/:id", put(org::update_org))
        .route("/:id", delete(org::delete_org))
        .route("/:id/children", post(org::create_org_child));

    let party_router = Router::new()
        .route("/add", post(party_org::add))
        .route("/delete", delete(party_org::delete))
        .route("/update", put(party_org::update))
        .route("/list", get(party_org::list))
        .route("/sync", post(party_org::sync));

    Router::new()
        .route("/health", get(health::health_check))
        .nest("/api/adm/org", org_router)
        .nest("/api/adm/sys/partyOrg", party_router)
        .with_state(state)
}

