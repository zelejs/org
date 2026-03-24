use rand::{distributions::Alphanumeric, Rng};
use sqlx::{Postgres, PgPool, QueryBuilder};

use crate::models::{
    error::{AppError, AppResult},
    org::{
        CreateOrgRequest, OrgListQuery, OrgTreeQuery, PageResult, SysOrg, SysOrgTreeItem, TreeTop,
        UpdateOrgRequest,
    },
};
use crate::services::RequestContext;

const ORG_TYPE_TENANT: i32 = 1;
const SYS_ORG_ID: i64 = 1;

#[derive(Clone)]
pub struct OrgService {
    pool: PgPool,
}

impl OrgService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_child(
        &self,
        parent_id: i64,
        req: CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<i64> {
        let parent = self
            .get_by_id(parent_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("父组织不存在".to_string()))?;

        if parent.org_type.unwrap_or(2) <= ORG_TYPE_TENANT {
            return Err(AppError::BadRequest(
                "平台跟租户类型的组织不可创建".to_string(),
            ));
        }

        let right_num = parent
            .right_num
            .ok_or_else(|| AppError::Internal("父组织right_num为空".to_string()))?;
        let node_level = parent.node_level.unwrap_or(0);

        let mut tx = self
            .pool
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

        let org_code = if let Some(code) = req.org_code {
            code
        } else {
            Self::random_org_code()
        };

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

    pub async fn delete_node(&self, id: i64) -> AppResult<i64> {
        let org = self
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("组织不存在".to_string()))?;

        if org.org_type.unwrap_or(2) <= ORG_TYPE_TENANT {
            return Err(AppError::BadRequest(
                "平台跟租户类型的组织不可删除".to_string(),
            ));
        }
        if org.id == SYS_ORG_ID {
            return Err(AppError::BadRequest("顶层组织不能删除".to_string()));
        }

        let left_num = org
            .left_num
            .ok_or_else(|| AppError::Internal("left_num为空".to_string()))?;
        let right_num = org
            .right_num
            .ok_or_else(|| AppError::Internal("right_num为空".to_string()))?;

        let descendants: i64 = sqlx::query_scalar(
            "SELECT COUNT(1) FROM t_sys_org WHERE left_num >= $1 AND right_num <= $2 AND delete_flag = 0",
        )
        .bind(left_num)
        .bind(right_num)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        if descendants > 1 {
            return Err(AppError::BadRequest("有子组织，不可删除".to_string()));
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

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

        tx.commit()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(id)
    }

    pub async fn update_node(&self, id: i64, req: UpdateOrgRequest) -> AppResult<i64> {
        let existing = self
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("组织不存在".to_string()))?;

        if existing.org_type.unwrap_or(2) <= ORG_TYPE_TENANT
            && req.org_type.is_some()
            && req.org_type != existing.org_type
        {
            return Err(AppError::BadRequest(
                "平台跟租户类型的组织不能修改类型".to_string(),
            ));
        }
        if req.pid.is_some_and(|v| Some(v) != existing.pid)
            || req.node_level.is_some_and(|v| Some(v) != existing.node_level)
            || req.left_num.is_some_and(|v| Some(v) != existing.left_num)
            || req.right_num.is_some_and(|v| Some(v) != existing.right_num)
        {
            return Err(AppError::BadRequest(
                "非法数据:不能修改组织层级相关字段".to_string(),
            ));
        }

        sqlx::query(
            r#"
            UPDATE t_sys_org
            SET name = $1, full_name = $2, note = $3, org_type = $4, org_code = $5, update_time = NOW()
            WHERE id = $6
            "#,
        )
        .bind(req.name.unwrap_or(existing.name))
        .bind(req.full_name.or(existing.full_name))
        .bind(req.note.or(existing.note))
        .bind(req.org_type.or(existing.org_type))
        .bind(req.org_code.or(existing.org_code))
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(id)
    }

    pub async fn get_org(&self, id: i64) -> AppResult<SysOrg> {
        self.get_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("组织不存在".to_string()))
    }

    pub async fn page_orgs(&self, query: OrgListQuery) -> AppResult<PageResult<SysOrg>> {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM t_sys_org WHERE delete_flag = 0");
        if let Some(name) = query.name {
            qb.push(" AND name LIKE ");
            qb.push_bind(format!("%{}%", name));
        }
        qb.push(" ORDER BY node_level ASC");
        qb.push(" LIMIT ");
        qb.push_bind(query.page_size);
        qb.push(" OFFSET ");
        qb.push_bind((query.page_num - 1) * query.page_size);

        let records = qb
            .build_query_as::<SysOrg>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(PageResult {
            current: query.page_num,
            size: query.page_size,
            records,
        })
    }

    pub async fn tree(&self, query: OrgTreeQuery, ctx: &RequestContext) -> AppResult<TreeTop> {
        let jwt_appid = ctx.appid.clone();
        let jwt_org_id = ctx.org_id;

        let final_appid = jwt_appid.clone().or(query.appid.clone());
        let mut org_id = jwt_org_id;
        if org_id.is_none() && final_appid.is_none() {
            org_id = Some(1);
        }

        let by_param_appid = jwt_appid.is_none() && query.appid.is_some();
        let filter_type = if by_param_appid { 0 } else { 2 };
        let root_org_id = if by_param_appid { 1 } else { org_id.unwrap_or(1) };

        let list = self
            .list_like_name_and_org(
                query.search.as_deref(),
                final_appid.as_deref(),
                root_org_id,
                filter_type,
                ctx.tenant_org_id,
            )
            .await?;

        if list.is_empty() {
            return Ok(TreeTop { children: vec![] });
        }

        let mut items: Vec<SysOrgTreeItem> = list.into_iter().map(Into::into).collect();
        let selected_root = if by_param_appid {
            items
                .iter()
                .find(|v| v.pid.is_none() && v.appid == query.appid)
                .map(|v| v.id)
                .unwrap_or(root_org_id)
        } else {
            root_org_id
        };

        let root = Self::build_tree(&mut items, selected_root)
            .ok_or_else(|| AppError::BadRequest("找不到顶级节点".to_string()))?;

        Ok(TreeTop {
            children: vec![root],
        })
    }

    pub async fn get_by_id(&self, id: i64) -> AppResult<Option<SysOrg>> {
        let result = sqlx::query_as::<_, SysOrg>("SELECT * FROM t_sys_org WHERE id = $1 AND delete_flag = 0")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(result)
    }

    pub async fn list_like_name_and_org(
        &self,
        search: Option<&str>,
        appid: Option<&str>,
        org_id: i64,
        filter_type: i32,
        tenant_org_id: Option<i64>,
    ) -> AppResult<Vec<SysOrg>> {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM t_sys_org WHERE delete_flag = 0");

        if let Some(s) = search {
            qb.push(" AND name LIKE ");
            qb.push_bind(format!("%{}%", s));
        }
        if let Some(a) = appid {
            qb.push(" AND (appid = ");
            qb.push_bind(a);
            qb.push(" OR appid IS NULL)");
        }
        if org_id != 1 && filter_type == 2 {
            qb.push(
                " AND id IN (
                    SELECT c.id FROM t_sys_org f
                    LEFT JOIN t_sys_org c
                    ON (f.left_num <= c.left_num AND c.right_num <= f.right_num AND c.delete_flag = 0",
            );
            if let Some(tenant) = tenant_org_id {
                qb.push(" AND c.tenant_org_id = ");
                qb.push_bind(tenant);
            } else {
                qb.push(" AND 1 = 0");
            }
            qb.push(") WHERE f.id = ");
            qb.push_bind(org_id);
            qb.push(" AND f.delete_flag = 0)");
        }
        if org_id != 1 && filter_type == 1 {
            qb.push(
                " AND id IN (
                    SELECT c.id FROM t_sys_org f
                    LEFT JOIN t_sys_org c
                    ON (f.left_num <= c.left_num AND c.right_num <= f.right_num AND c.delete_flag = 0)
                    WHERE f.id = ",
            );
            qb.push_bind(org_id);
            qb.push(" AND f.delete_flag = 0)");
        }
        qb.push(" ORDER BY node_level ASC");

        let data = qb
            .build_query_as::<SysOrg>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(data)
    }

    fn random_org_code() -> String {
        rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }

    fn build_tree(items: &mut Vec<SysOrgTreeItem>, root_id: i64) -> Option<SysOrgTreeItem> {
        use std::collections::HashMap;

        // Create a map of all items by their ID
        let mut items_map: HashMap<i64, SysOrgTreeItem> = items.drain(..).map(|item| (item.id, item)).collect();

        // Build a map of parent ID to children IDs
        let mut parent_to_children: HashMap<i64, Vec<i64>> = HashMap::new();

        for (&id, item) in &items_map {
            if let Some(pid) = item.pid {
                parent_to_children.entry(pid).or_default().push(id);
            }
        }

        // Recursively build the tree
        Self::build_tree_recursive_helper(&mut items_map, &parent_to_children, root_id)
    }

    fn build_tree_recursive_helper(
        items_map: &mut std::collections::HashMap<i64, SysOrgTreeItem>,
        parent_to_children: &std::collections::HashMap<i64, Vec<i64>>,
        item_id: i64,
    ) -> Option<SysOrgTreeItem> {
        if let Some(mut item) = items_map.remove(&item_id) {
            // Get children IDs for this item
            if let Some(children_ids) = parent_to_children.get(&item_id) {
                for child_id in children_ids {
                    if let Some(child) = Self::build_tree_recursive_helper(items_map, parent_to_children, *child_id) {
                        item.children.push(child);
                    }
                }
            }
            Some(item)
        } else {
            None
        }
    }
}

