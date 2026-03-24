use sqlx::{PgPool, Row};

use crate::models::{
    error::{AppError, AppResult},
    org::CreateOrgRequest,
    ext_org::{ExtOrgRelation, ExtOrgRequest},
};
use crate::services::{OrgService, RequestContext};

#[derive(Clone)]
pub struct ExtOrgService {
    pool: PgPool,
    org_service: OrgService,
}

impl ExtOrgService {
    pub fn new(pool: PgPool, org_service: OrgService) -> Self {
        Self { pool, org_service }
    }

    pub async fn add(&self, req: ExtOrgRequest) -> AppResult<i64> {
        if req.id == 0 || req.parent_id == 0 {
            return Err(AppError::BadRequest("扩展组织id或父id不能为空".to_string()));
        }

        let parent_org_id: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM t_sys_org_ext WHERE ext_org_id = $1 AND delete_flag = 0",
        )
        .bind(req.parent_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        let parent_org_id =
            parent_org_id.ok_or_else(|| AppError::BadRequest("父扩展组织不存在".to_string()))?;

        let exists: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM t_sys_org_ext WHERE ext_org_id = $1 AND delete_flag = 0",
        )
        .bind(req.id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        if exists.is_some() {
            return Err(AppError::BadRequest("扩展组织已存在，不可重复创建".to_string()));
        }

        let new_org_id = self
            .org_service
            .create_child(
                parent_org_id,
                CreateOrgRequest {
                    name: req.short_name.clone().unwrap_or_else(|| req.name.clone()),
                    full_name: Some(req.name.clone()),
                    org_code: req.org_num.clone(),
                    note: None,
                    org_type: Some(2),
                },
                &RequestContext {
                    org_id: None,
                    tenant_org_id: None,
                    appid: None,
                },
            )
            .await?;

        sqlx::query(
            "INSERT INTO t_sys_org_ext (id, ext_org_id, ext_org_type, delete_flag, create_time, update_time) VALUES ($1, $2, $3, 0, NOW(), NOW())",
        )
        .bind(new_org_id)
        .bind(req.id)
        .bind(req.r#type)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(new_org_id)
    }

    pub async fn delete(&self, ext_org_id: i64) -> AppResult<i64> {
        let org_id: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM t_sys_org_ext WHERE ext_org_id = $1 AND delete_flag = 0",
        )
        .bind(ext_org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        let org_id = org_id.ok_or_else(|| AppError::NotFound("扩展组织不存在".to_string()))?;

        sqlx::query("UPDATE t_sys_org_ext SET delete_flag = 1, update_time = NOW() WHERE id = $1")
            .bind(org_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        self.org_service.delete_node(org_id).await
    }

    pub async fn update(&self, req: ExtOrgRequest) -> AppResult<i64> {
        let org_id: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM t_sys_org_ext WHERE ext_org_id = $1 AND delete_flag = 0",
        )
        .bind(req.id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        let org_id = org_id.ok_or_else(|| AppError::NotFound("扩展组织不存在".to_string()))?;

        sqlx::query("UPDATE t_sys_org_ext SET ext_org_type = $1, update_time = NOW() WHERE id = $2")
            .bind(req.r#type)
            .bind(org_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        sqlx::query(
            "UPDATE t_sys_org SET name = $1, full_name = $2, org_code = $3, update_time = NOW() WHERE id = $4",
        )
        .bind(req.short_name.unwrap_or_else(|| req.name.clone()))
        .bind(req.name)
        .bind(req.org_num)
        .bind(org_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(org_id)
    }

    pub async fn list(&self) -> AppResult<Vec<ExtOrgRelation>> {
        let rows = sqlx::query(
            r#"
            SELECT
              ext.ext_org_id AS id,
              parent_ext.ext_org_id AS parent_id,
              org.full_name AS name,
              org.name AS short_name,
              org.org_code AS org_num,
              ext.ext_org_type AS type,
              org.id AS org_id,
              org.tenant_org_id AS tenant_org_id,
              (org.id = org.tenant_org_id) AS tenant_flag
            FROM t_sys_org_ext ext
            LEFT JOIN t_sys_org org ON ext.id = org.id AND org.delete_flag = 0
            LEFT JOIN t_sys_org parent_org ON org.pid = parent_org.id AND parent_org.delete_flag = 0
            LEFT JOIN t_sys_org_ext parent_ext ON parent_org.id = parent_ext.id AND parent_ext.delete_flag = 0
            WHERE ext.delete_flag = 0
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        let result = rows
            .into_iter()
            .map(|row| ExtOrgRelation {
                id: row.get("id"),
                parent_id: row.try_get("parent_id").ok(),
                name: row.try_get("name").ok(),
                short_name: row.try_get("short_name").ok(),
                org_num: row.try_get("org_num").ok(),
                r#type: row.try_get("type").ok(),
                org_id: row.try_get("org_id").ok(),
                tenant_org_id: row.try_get("tenant_org_id").ok(),
                tenant_flag: row.try_get("tenant_flag").ok(),
            })
            .collect();

        Ok(result)
    }

    pub async fn sync(&self) -> AppResult<i64> {
        Ok(0)
    }
}

