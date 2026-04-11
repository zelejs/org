use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Tenant entity from t_sys_tenant table
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SysTenant {
    pub id: i64,
    pub name: String,
    pub org_id: Option<i64>,
    pub app_id: Option<String>,
    pub domain: Option<String>,
    pub status: Option<i32>,
    pub start_time: Option<NaiveDateTime>,
    pub delete_flag: i32,
}

/// Tenant DTO with organization information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SysTenantDTO {
    pub id: i64,
    pub name: String,
    pub org_id: Option<i64>,
    pub org_code: Option<String>,
    pub org_name: Option<String>,
    pub domain: Option<String>,
    pub status: Option<i32>,
    pub app_id: Option<String>,
}

/// Tenant list query parameters
#[derive(Debug, Clone, Deserialize)]
pub struct TenantQuery {
    #[serde(default = "default_page_num", alias = "pageNum")]
    pub page_num: i64,
    #[serde(default = "default_page_size", alias = "pageSize")]
    pub page_size: i64,
    pub search: Option<String>,
}

fn default_page_num() -> i64 {
    1
}

fn default_page_size() -> i64 {
    10
}
