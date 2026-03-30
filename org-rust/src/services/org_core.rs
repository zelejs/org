use rand::{distributions::Alphanumeric, Rng};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

use crate::models::{
    error::{AppError, AppResult},
    org::{SysOrg, SysOrgTreeItem, CreateOrgRequest},
};
use crate::services::RequestContext;

const ORG_TYPE_TENANT: i32 = 1;
const SYS_ORG_ID: i64 = 1;

/// Root organization status
#[derive(Debug, Clone)]
pub enum RootOrgStatus {
    AlreadyExists(SysOrg, i64),  // (organization, children count)
    Created(SysOrg),
}

/// Root organization information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootOrgInfo {
    pub org_id: i64,
    pub appid: Option<String>,
    pub name: String,
    pub org_type: Option<i32>,
    pub children_count: i64,
}

/// Initialize root organization
pub async fn init_root_org(
    pool: &PgPool,
    appid: Option<String>,
    name: String,
) -> AppResult<RootOrgStatus> {
    // Check if root org already exists
    if let Some(existing) = get_root_org_by_appid(pool, appid.as_deref()).await? {
        let count = count_children(pool, existing.id).await?;
        return Ok(RootOrgStatus::AlreadyExists(existing, count));
    }

    // Create new root organization (pid = NULL, left_num = 1, right_num = 2, node_level = 1)
    let org_code = random_org_code();
    let new_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO t_sys_org
        (pid, name, full_name, org_code, node_level, left_num, right_num, note, org_type, appid, tenant_id, tenant_org_id, delete_flag, create_time, update_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 0, NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(None::<i64>)  // pid = NULL for root
    .bind(&name)
    .bind(&name)  // full_name = name
    .bind(&org_code)
    .bind(1i32)  // node_level = 1
    .bind(1i32)  // left_num = 1
    .bind(2i32)  // right_num = 2
    .bind(None::<String>)  // note
    .bind(0i32)  // org_type = 0 (platform)
    .bind(&appid)
    .bind(None::<i64>)  // tenant_id
    .bind(None::<i64>)  // tenant_org_id
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let new_org = get_by_id(pool, new_id)
        .await?
        .ok_or_else(|| AppError::Internal("Failed to retrieve created organization".to_string()))?;

    Ok(RootOrgStatus::Created(new_org))
}

/// Get root organization by appid
pub async fn get_root_org_by_appid(
    pool: &PgPool,
    appid: Option<&str>,
) -> AppResult<Option<SysOrg>> {
    let result = if let Some(a) = appid {
        sqlx::query_as::<_, SysOrg>(
            "SELECT * FROM t_sys_org WHERE pid IS NULL AND appid = $1 AND delete_flag = 0"
        )
        .bind(a)
        .fetch_optional(pool)
        .await
    } else {
        sqlx::query_as::<_, SysOrg>(
            "SELECT * FROM t_sys_org WHERE pid IS NULL AND appid IS NULL AND delete_flag = 0"
        )
        .fetch_optional(pool)
        .await
    }
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(result)
}

/// List all root organizations
pub async fn list_all_root_orgs(pool: &PgPool) -> AppResult<Vec<RootOrgInfo>> {
    let orgs = sqlx::query_as::<_, SysOrg>(
        "SELECT * FROM t_sys_org WHERE pid IS NULL AND delete_flag = 0 ORDER BY id"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut result = Vec::new();
    for org in orgs {
        let children_count = count_children(pool, org.id).await?;
        result.push(RootOrgInfo {
            org_id: org.id,
            appid: org.appid,
            name: org.name,
            org_type: org.org_type,
            children_count,
        });
    }

    Ok(result)
}

/// Insert child organization
pub async fn insert_child_org(
    pool: &PgPool,
    parent_id: i64,
    req: CreateOrgRequest,
    ctx: &RequestContext,
) -> AppResult<i64> {
    let parent = get_by_id(pool, parent_id)
        .await?
        .ok_or_else(|| AppError::BadRequest("Parent organization does not exist".to_string()))?;

    if parent.org_type.unwrap_or(2) <= ORG_TYPE_TENANT {
        return Err(AppError::BadRequest(
            "Cannot create under platform or tenant type organizations".to_string(),
        ));
    }

    let right_num = parent
        .right_num
        .ok_or_else(|| AppError::Internal("Parent right_num is null".to_string()))?;
    let node_level = parent.node_level.unwrap_or(0);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query(
        "UPDATE t_sys_org SET left_num = left_num + 2 WHERE left_num >= $1 AND delete_flag = 0",
    )
    .bind(right_num)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query(
        "UPDATE t_sys_org SET right_num = right_num + 2 WHERE right_num >= $1 AND delete_flag = 0",
    )
    .bind(right_num)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let org_code = req.org_code.unwrap_or_else(|| random_org_code());
    let org_type = req.org_type.unwrap_or(2);
    let name = req.name;
    let full_name = req.full_name.unwrap_or_else(|| name.clone());

    let new_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO t_sys_org
        (pid, name, full_name, org_code, node_level, left_num, right_num, note, org_type, appid, tenant_id, tenant_org_id, delete_flag, create_time, update_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 0, NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(parent.id)
    .bind(&name)
    .bind(&full_name)
    .bind(&org_code)
    .bind(node_level + 1)
    .bind(right_num)
    .bind(right_num + 1)
    .bind(&req.note)
    .bind(org_type)
    .bind(&parent.appid)
    .bind(parent.tenant_id)
    .bind(parent.tenant_org_id.or(ctx.tenant_org_id))
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(new_id)
}

/// Remove organization
pub async fn remove_org(
    pool: &PgPool,
    id: i64,
    force: bool,
) -> AppResult<i64> {
    let org = get_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Organization does not exist".to_string()))?;

    if org.org_type.unwrap_or(2) <= ORG_TYPE_TENANT {
        return Err(AppError::BadRequest(
            "Platform and tenant type organizations cannot be deleted".to_string(),
        ));
    }
    if org.id == SYS_ORG_ID {
        return Err(AppError::BadRequest("Top-level organization cannot be deleted".to_string()));
    }

    let left_num = org
        .left_num
        .ok_or_else(|| AppError::Internal("left_num is null".to_string()))?;
    let right_num = org
        .right_num
        .ok_or_else(|| AppError::Internal("right_num is null".to_string()))?;

    let descendants: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM t_sys_org WHERE left_num >= $1 AND right_num <= $2 AND delete_flag = 0",
    )
    .bind(left_num)
    .bind(right_num)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    if descendants > 1 && !force {
        return Err(AppError::BadRequest("Has child organizations, cannot delete. Use --force to cascade delete.".to_string()));
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if force {
        // Cascade delete: mark all descendants as deleted
        sqlx::query("UPDATE t_sys_org SET delete_flag = 1, update_time = NOW() WHERE left_num >= $1 AND right_num <= $2")
            .bind(left_num)
            .bind(right_num)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        sqlx::query("UPDATE t_sys_org_ext SET delete_flag = 1, update_time = NOW() WHERE id IN (SELECT id FROM t_sys_org WHERE left_num >= $1 AND right_num <= $2)")
            .bind(left_num)
            .bind(right_num)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // Calculate the width of the subtree
        let width = right_num - left_num + 1;

        sqlx::query("UPDATE t_sys_org SET left_num = left_num - $1 WHERE left_num > $2")
            .bind(width)
            .bind(right_num)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        sqlx::query("UPDATE t_sys_org SET right_num = right_num - $1 WHERE right_num > $2")
            .bind(width)
            .bind(right_num)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    } else {
        // Single delete
        sqlx::query("UPDATE t_sys_org SET delete_flag = 1, update_time = NOW() WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        sqlx::query("UPDATE t_sys_org_ext SET delete_flag = 1, update_time = NOW() WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        sqlx::query("UPDATE t_sys_org SET left_num = left_num - 2 WHERE left_num > $1")
            .bind(right_num)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        sqlx::query("UPDATE t_sys_org SET right_num = right_num - 2 WHERE right_num > $1")
            .bind(right_num)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(id)
}

/// Get subtree for a given root_id
pub async fn get_subtree(
    pool: &PgPool,
    root_id: i64,
) -> AppResult<SysOrgTreeItem> {
    let list = list_descendants(pool, root_id).await?;

    if list.is_empty() {
        return Err(AppError::NotFound("No organizations found".to_string()));
    }

    let mut items: Vec<SysOrgTreeItem> = list.into_iter().map(Into::into).collect();
    let root = build_tree(&mut items, root_id)
        .ok_or_else(|| AppError::BadRequest("Root node not found".to_string()))?;

    Ok(root)
}

/// Count children using Nested Set Model
pub async fn count_children(
    pool: &PgPool,
    parent_id: i64,
) -> AppResult<i64> {
    let org = get_by_id(pool, parent_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Organization does not exist".to_string()))?;

    let left_num = org
        .left_num
        .ok_or_else(|| AppError::Internal("left_num is null".to_string()))?;
    let right_num = org
        .right_num
        .ok_or_else(|| AppError::Internal("right_num is null".to_string()))?;

    // Using Nested Set Model: (right_num - left_num - 1) / 2
    let count = (right_num - left_num - 1) / 2;
    Ok(count as i64)
}

/// Get organization by ID
pub async fn get_by_id(pool: &PgPool, id: i64) -> AppResult<Option<SysOrg>> {
    let result = sqlx::query_as::<_, SysOrg>("SELECT * FROM t_sys_org WHERE id = $1 AND delete_flag = 0")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(result)
}

/// List all descendants of a given root organization
pub async fn list_descendants(pool: &PgPool, root_id: i64) -> AppResult<Vec<SysOrg>> {
    let root = get_by_id(pool, root_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Root organization does not exist".to_string()))?;

    let left_num = root
        .left_num
        .ok_or_else(|| AppError::Internal("root left_num is null".to_string()))?;
    let right_num = root
        .right_num
        .ok_or_else(|| AppError::Internal("root right_num is null".to_string()))?;

    let data = sqlx::query_as::<_, SysOrg>(
        "SELECT * FROM t_sys_org WHERE left_num >= $1 AND right_num <= $2 AND delete_flag = 0 ORDER BY node_level ASC"
    )
    .bind(left_num)
    .bind(right_num)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(data)
}

/// Build tree structure from flat list
pub fn build_tree(items: &mut Vec<SysOrgTreeItem>, root_id: i64) -> Option<SysOrgTreeItem> {
    use std::collections::HashMap;

    let mut items_map: HashMap<i64, SysOrgTreeItem> = items.drain(..).map(|item| (item.id, item)).collect();
    let mut parent_to_children: HashMap<i64, Vec<i64>> = HashMap::new();

    for (&id, item) in &items_map {
        if let Some(pid) = item.pid {
            parent_to_children.entry(pid).or_default().push(id);
        }
    }

    build_tree_recursive_helper(&mut items_map, &parent_to_children, root_id)
}

fn build_tree_recursive_helper(
    items_map: &mut std::collections::HashMap<i64, SysOrgTreeItem>,
    parent_to_children: &std::collections::HashMap<i64, Vec<i64>>,
    item_id: i64,
) -> Option<SysOrgTreeItem> {
    if let Some(mut item) = items_map.remove(&item_id) {
        if let Some(children_ids) = parent_to_children.get(&item_id) {
            for child_id in children_ids {
                if let Some(child) = build_tree_recursive_helper(items_map, parent_to_children, *child_id) {
                    item.children.push(child);
                }
            }
        }
        Some(item)
    } else {
        None
    }
}

fn random_org_code() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
