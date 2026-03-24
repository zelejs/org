use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtOrgRequest {
    pub id: i64,
    pub parent_id: i64,
    pub name: String,
    pub short_name: Option<String>,
    pub org_num: Option<String>,
    pub r#type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ExtOrgRelation {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub org_num: Option<String>,
    pub r#type: Option<i32>,
    pub org_id: Option<i64>,
    pub tenant_org_id: Option<i64>,
    pub tenant_flag: Option<bool>,
}

