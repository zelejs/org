use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SysOrg {
    pub id: i64,
    pub pid: Option<i64>,
    pub name: String,
    pub full_name: Option<String>,
    pub org_code: Option<String>,
    pub node_level: Option<i32>,
    pub left_num: Option<i32>,
    pub right_num: Option<i32>,
    pub note: Option<String>,
    pub status: Option<String>,
    pub org_type: Option<i32>,
    pub appid: Option<String>,
    pub icon: Option<String>,
    pub is_visible: Option<bool>,
    pub need_validate: Option<bool>,
    pub delete_flag: i32,
    pub tenant_id: Option<i64>,
    pub tenant_org_id: Option<i64>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrgRequest {
    pub name: String,
    pub full_name: Option<String>,
    pub org_code: Option<String>,
    pub note: Option<String>,
    pub org_type: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrgRequest {
    pub pid: Option<i64>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub org_code: Option<String>,
    pub node_level: Option<i32>,
    pub left_num: Option<i32>,
    pub right_num: Option<i32>,
    pub note: Option<String>,
    pub org_type: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrgListQuery {
    #[serde(default = "default_page_num", alias = "pageNum")]
    pub page_num: i64,
    #[serde(default = "default_page_size", alias = "pageSize")]
    pub page_size: i64,
    pub name: Option<String>,
}

fn default_page_num() -> i64 {
    1
}

fn default_page_size() -> i64 {
    10
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgTreeQuery {
    pub search: Option<String>,
    pub appid: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    pub current: i64,
    pub size: i64,
    pub records: Vec<T>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysOrgTreeItem {
    pub id: i64,
    pub pid: Option<i64>,
    pub name: String,
    pub full_name: Option<String>,
    pub node_level: Option<i32>,
    pub note: Option<String>,
    pub org_type: Option<i32>,
    pub appid: Option<String>,
    pub tenant_id: Option<i64>,
    pub tenant_org_id: Option<i64>,
    pub tenant_flag: bool,
    pub children: Vec<SysOrgTreeItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeTop {
    pub children: Vec<SysOrgTreeItem>,
}

impl From<SysOrg> for SysOrgTreeItem {
    fn from(value: SysOrg) -> Self {
        Self {
            id: value.id,
            pid: value.pid,
            name: value.name,
            full_name: value.full_name,
            node_level: value.node_level,
            note: value.note,
            org_type: value.org_type,
            appid: value.appid,
            tenant_id: value.tenant_id,
            tenant_org_id: value.tenant_org_id,
            tenant_flag: value.tenant_org_id == Some(value.id),
            children: vec![],
        }
    }
}

