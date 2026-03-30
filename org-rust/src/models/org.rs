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
    pub level: Option<String>,           // 业务层级标识 (如: school/college/major/class)
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
    pub icon: Option<String>,      // 图标
    pub level: Option<String>,     // 业务层级
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
    pub icon: Option<String>,      // 图标
    pub level: Option<String>,     // 业务层级
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysOrgTreeItem {
    pub id: i64,
    pub pid: Option<i64>,
    pub name: String,
    pub full_name: Option<String>,
    pub node_level: Option<i32>,
    pub left_num: Option<i32>,
    pub right_num: Option<i32>,
    pub note: Option<String>,
    pub org_type: Option<i32>,
    pub appid: Option<String>,
    pub tenant_id: Option<i64>,
    pub tenant_org_id: Option<i64>,
    pub tenant_flag: bool,
    pub icon: Option<String>,              // 图标
    pub level: Option<String>,             // 业务层级
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
            left_num: value.left_num,
            right_num: value.right_num,
            note: value.note,
            org_type: value.org_type,
            appid: value.appid,
            tenant_id: value.tenant_id,
            tenant_org_id: value.tenant_org_id,
            tenant_flag: value.tenant_org_id == Some(value.id),
            icon: value.icon,
            level: value.level,
            children: vec![],
        }
    }
}

/// 通用组织导入结构 - 支持多种字段别名
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportOrgTreeItem {
    // ID (支持字符串或数字)
    #[serde(alias = "id")]
    pub id: Option<serde_json::Value>,

    // 名称
    #[serde(alias = "name")]
    pub name: Option<String>,

    // 组织代码
    #[serde(alias = "code", alias = "org_code")]
    pub org_code: Option<String>,

    // 业务层级 (school/college/major/class 等)
    #[serde(alias = "level", alias = "type", alias = "biz_type")]
    pub level: Option<String>,

    // 图标
    #[serde(alias = "icon", alias = "emoji")]
    pub icon: Option<String>,

    // 备注/描述
    #[serde(alias = "note", alias = "description", alias = "desc", alias = "remark")]
    pub note: Option<String>,

    // 组织层级 (数字)
    #[serde(alias = "node_level", alias = "depth")]
    pub node_level: Option<i32>,

    // 组织类型
    #[serde(alias = "org_type", alias = "type_code")]
    pub org_type: Option<i32>,

    // 子节点
    #[serde(alias = "children", alias = "childes", alias = "items")]
    pub children: Option<Vec<ImportOrgTreeItem>>,
}

impl ImportOrgTreeItem {
    /// 转换为 SysOrgTreeItem (自动分配 ID)
    pub fn to_sys_org_item(&self, assigned_id: i64, parent_id: Option<i64>) -> SysOrgTreeItem {
        SysOrgTreeItem {
            id: assigned_id,
            pid: parent_id,
            name: self.name.clone().unwrap_or_default(),
            full_name: self.name.clone(),
            node_level: self.node_level,
            left_num: None,
            right_num: None,
            note: self.note.clone(),
            org_type: self.org_type,
            appid: None,
            tenant_id: None,
            tenant_org_id: None,
            tenant_flag: false,
            icon: self.icon.clone(),
            level: self.level.clone(),
            children: vec![],
        }
    }
}

