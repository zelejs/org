use sqlx::PgPool;
use std::sync::Arc;

use crate::services::{ExtOrgService, OrgService};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub org_service: Arc<OrgService>,
    pub ext_org_service: Arc<ExtOrgService>,
}

