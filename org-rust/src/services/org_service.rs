use sqlx::{Postgres, PgPool, QueryBuilder};
use std::sync::Arc;

use crate::models::{
    error::{AppError, AppResult},
    org::{
        CreateOrgRequest, OrgListQuery, OrgTreeQuery, PageResult, SysOrg, SysOrgTreeItem,
        SysOrgTenantTreeItem, SysOrgTenantTreeItemRow, TreeTop, TreeTopTenant, UpdateOrgRequest,
    },
    tenant::{SysTenantDTO, TenantQuery},
};
use crate::services::{
    org_core,
    RequestContext,
    ext::OrgExtensionManager,
};

const ORG_TYPE_TENANT: i32 = 1;

#[derive(Clone)]
pub struct OrgService {
    pool: PgPool,
    extension_manager: Option<Arc<OrgExtensionManager>>,
}

impl OrgService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            extension_manager: None,
        }
    }

    /// 设置扩展管理器
    pub fn with_extension_manager(mut self, manager: Arc<OrgExtensionManager>) -> Self {
        self.extension_manager = Some(manager);
        self
    }

    /// Initialize root organization
    pub async fn init_root(
        &self,
        appid: Option<String>,
        name: String,
    ) -> AppResult<org_core::RootOrgStatus> {
        org_core::init_root_org(&self.pool, appid, name).await
    }

    /// List all root organizations
    pub async fn list_roots(&self) -> AppResult<Vec<org_core::RootOrgInfo>> {
        org_core::list_all_root_orgs(&self.pool).await
    }

    /// Get root organization by appid
    pub async fn get_root_by_appid(&self, appid: Option<&str>) -> AppResult<Option<SysOrg>> {
        org_core::get_root_org_by_appid(&self.pool, appid).await
    }

    pub async fn create_child(
        &self,
        parent_id: i64,
        req: CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<i64> {
        org_core::insert_child_org(
            &self.pool,
            parent_id,
            req,
            ctx,
            self.extension_manager.as_deref(),
        ).await
    }

    pub async fn delete_node(&self, id: i64) -> AppResult<i64> {
        org_core::remove_org(
            &self.pool,
            id,
            false,
            self.extension_manager.as_deref(),
        ).await
    }

    pub async fn update_node(&self, id: i64, req: UpdateOrgRequest) -> AppResult<i64> {
        let existing = self
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization does not exist".to_string()))?;

        if existing.org_type.unwrap_or(2) <= ORG_TYPE_TENANT
            && req.org_type.is_some()
            && req.org_type != existing.org_type
        {
            return Err(AppError::BadRequest(
                "Platform and tenant type organizations cannot change type".to_string(),
            ));
        }
        if req.pid.is_some_and(|v| Some(v) != existing.pid)
            || req.node_level.is_some_and(|v| Some(v) != existing.node_level)
            || req.left_num.is_some_and(|v| Some(v) != existing.left_num)
            || req.right_num.is_some_and(|v| Some(v) != existing.right_num)
        {
            return Err(AppError::BadRequest(
                "Invalid data: Cannot modify hierarchy related fields".to_string(),
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
            .ok_or_else(|| AppError::NotFound("Organization does not exist".to_string()))
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

        let root = org_core::build_tree(&mut items, selected_root)
            .ok_or_else(|| AppError::BadRequest("Root node not found".to_string()))?;

        Ok(TreeTop {
            children: vec![root],
        })
    }

    pub async fn get_by_id(&self, id: i64) -> AppResult<Option<SysOrg>> {
        org_core::get_by_id(&self.pool, id).await
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

    /// Page query tenants with organization information
    pub async fn page_tenants(&self, query: TenantQuery) -> AppResult<PageResult<SysTenantDTO>> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"SELECT
                t.id,
                t.name,
                t.org_id,
                o.org_code,
                o.name as org_name,
                t.domain,
                t.status,
                t.app_id
            FROM t_sys_tenant t
            LEFT JOIN t_sys_org o ON t.org_id = o.id
            WHERE t.delete_flag = 0"#,
        );

        if let Some(search) = query.search {
            qb.push(" AND t.name LIKE ");
            qb.push_bind(format!("%{}%", search));
        }

        qb.push(" ORDER BY t.id");
        qb.push(" LIMIT ");
        qb.push_bind(query.page_size);
        qb.push(" OFFSET ");
        qb.push_bind((query.page_num - 1) * query.page_size);

        let records = qb
            .build_query_as::<SysTenantDTO>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(PageResult {
            current: query.page_num,
            size: query.page_size,
            records,
        })
    }

    /// Query organization tree with tenant information
    pub async fn tree_with_tenant(
        &self,
        query: OrgTreeQuery,
        ctx: &RequestContext,
    ) -> AppResult<TreeTopTenant> {
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
            .list_org_with_tenant(query.search.as_deref(), final_appid.as_deref(), root_org_id, filter_type, ctx.tenant_org_id)
            .await?;

        if list.is_empty() {
            return Ok(TreeTopTenant { children: vec![] });
        }

        let mut items: Vec<SysOrgTenantTreeItem> = list.into_iter().map(Into::into).collect();
        let selected_root = if by_param_appid {
            items
                .iter()
                .find(|v| v.pid.is_none() && v.appid == query.appid)
                .map(|v| v.id)
                .unwrap_or(root_org_id)
        } else {
            root_org_id
        };

        let root = org_core::build_tenant_tree(&mut items, selected_root)
            .ok_or_else(|| AppError::BadRequest("Root node not found".to_string()))?;

        Ok(TreeTopTenant {
            children: vec![root],
        })
    }

    /// Internal method: query organizations with tenant information
    async fn list_org_with_tenant(
        &self,
        search: Option<&str>,
        appid: Option<&str>,
        org_id: i64,
        filter_type: i32,
        tenant_org_id: Option<i64>,
    ) -> AppResult<Vec<SysOrgTenantTreeItemRow>> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"SELECT
                o.id,
                o.pid,
                o.name,
                o.full_name,
                o.node_level,
                o.note,
                o.org_type,
                o.appid,
                o.tenant_id,
                o.tenant_org_id,
                o.left_num,
                o.right_num,
                o.org_code,
                o.status,
                o.is_visible,
                o.need_validate,
                o.icon,
                o.level,
                o.create_time,
                o.update_time,
                t.id as tenant_id,
                t.name as tenant_name,
                o.org_code as tenant_code
            FROM t_sys_org o
            LEFT JOIN t_sys_tenant t ON o.tenant_org_id = t.org_id
            WHERE o.delete_flag = 0"#,
        );

        if let Some(s) = search {
            qb.push(" AND o.name LIKE ");
            qb.push_bind(format!("%{}%", s));
        }
        if let Some(a) = appid {
            qb.push(" AND (o.appid = ");
            qb.push_bind(a);
            qb.push(" OR o.appid IS NULL)");
        }
        if org_id != 1 && filter_type == 2 {
            qb.push(
                " AND o.id IN (
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
            qb.push(") WHERE f.id = ",
            );
            qb.push_bind(org_id);
            qb.push(" AND f.delete_flag = 0)");
        }
        if org_id != 1 && filter_type == 1 {
            qb.push(
                " AND o.id IN (
                    SELECT c.id FROM t_sys_org f
                    LEFT JOIN t_sys_org c
                    ON (f.left_num <= c.left_num AND c.right_num <= f.right_num AND c.delete_flag = 0)
                    WHERE f.id = ",
            );
            qb.push_bind(org_id);
            qb.push(" AND f.delete_flag = 0)");
        }
        qb.push(" ORDER BY o.node_level ASC");

        let data = qb
            .build_query_as::<SysOrgTenantTreeItemRow>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(data)
    }
}
