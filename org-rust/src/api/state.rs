use sqlx::PgPool;
use std::sync::Arc;

use crate::services::{OrgService, PartyOrgService};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub org_service: Arc<OrgService>,
    pub party_org_service: Arc<PartyOrgService>,
}

