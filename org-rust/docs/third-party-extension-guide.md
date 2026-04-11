# 第三方业务扩展实现指南

## 概述

本指南面向第三方业务系统开发者，说明如何基于 `org-rust` 框架实现组织扩展功能，实现业务组织表与 `t_sys_org.ext_org_id` 的关联。

### 应用场景

当组织在 `org-rust` 中创建时，第三方业务系统需要：
1. 同时创建自己的业务表记录（如学院表、专业表等）
2. 获取业务表生成的 ID
3. 将该 ID 作为 `ext_org_id` 关联回 `t_sys_org_ext` 表

### 设计原则

- **业务无关**: 框架不绑定具体业务，仅提供扩展接口
- **可选性**: 不实现扩展时，系统正常工作
- **事务一致性**: 扩展操作在同一事务中，失败则回滚

## 架构说明

```
┌─────────────────────────────────────────────────────────────┐
│                    org-rust 核心服务                          │
│  ┌──────────────┐         ┌──────────────────────────────┐  │
│  │ OrgService   │────────▶│ OrgExtensionManager          │  │
│  └──────────────┘         │  - 按 level 分发              │  │
│                           │  - 管理扩展处理器              │  │
│                           └──────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                                    │
                                    │ on_org_created()
                                    ▼
┌─────────────────────────────────────────────────────────────┐
│                 第三方业务系统（你的实现）                      │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ struct MyExtension                                   │  │
│  │   - on_org_created()    创建业务记录，返回 ext_org_id │  │
│  │   - on_org_delete()     清理业务记录                 │  │
│  │   - supported_levels()  声明支持的层级                │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                                    │
                                    │ INSERT
                                    ▼
┌─────────────────────────────────────────────────────────────┐
│                    第三方业务数据库                            │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │ t_college  │  │ t_major    │  │ t_class    │  ...        │
│  └────────────┘  └────────────┘  └────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

## 依赖配置

### Cargo.toml

```toml
[dependencies]
# org-rust 核心库（假设已发布到私有仓库）
org-rust = "0.1.0"

# 异步运行时
tokio = { version = "1.35", features = ["full"] }

# 异步 trait 支持
async-trait = "0.1"

# 数据库（根据你的系统选择）
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono"] }

# 错误处理
anyhow = "1.0"
thiserror = "1.0"

# 序列化
serde = { version = "1.0", features = ["derive"] }
```

## 接口定义

### OrgExtension Trait

```rust
use async_trait::async_trait;
use org_rust::models::org::{SysOrg, CreateOrgRequest};
use org_rust::services::RequestContext;
use org_rust::models::error::AppResult;

/// 第三方组织扩展接口
#[async_trait]
pub trait OrgExtension: Send + Sync {
    /// 组织创建后的扩展处理
    ///
    /// # 参数
    /// - `org`: 新创建的组织信息
    /// - `req`: 创建请求的原始数据
    /// - `ctx`: 请求上下文
    ///
    /// # 返回
    /// - `Ok(Some(id))`: 返回业务表生成的 ID，将存入 ext_org_id
    /// - `Ok(None)`: 不需要扩展，跳过 ext_org_id 关联
    /// - `Err(_)`: 扩展失败，事务将回滚
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>>;

    /// 组织删除前的扩展处理
    ///
    /// # 注意
    /// - 在删除组织之前调用
    /// - 用于处理业务数据约束（如外键依赖）
    /// - 返回错误将阻止组织删除
    async fn on_org_delete(
        &self,
        org: &SysOrg,
        ctx: &RequestContext,
    ) -> AppResult<()>;

    /// 声明此扩展处理器支持的层级
    ///
    /// # 返回
    /// 层级列表，匹配时此处理器将被调用
    fn supported_levels(&self) -> Vec<OrgLevel>;
}
```

### OrgLevel 枚举

```rust
/// 组织层级定义
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrgLevel {
    /// 物理层级（node_level）
    /// 用于按树的深度匹配
    NodeLevel(i32),

    /// 业务层级标识（level 字段）
    /// 用于按业务类型匹配，如 "school", "college", "major", "class"
    BusinessLevel(String),
}
```

### SysOrg 关键字段

```rust
pub struct SysOrg {
    pub id: i64,                    // org-rust 中的组织 ID
    pub pid: Option<i64>,           // 父组织 ID
    pub name: String,               // 组织简称
    pub full_name: Option<String>,  // 组织全称
    pub org_code: Option<String>,   // 组织代码
    pub node_level: Option<i32>,    // 物理层级（1,2,3...）
    pub level: Option<String>,      // 业务层级标识
    pub org_type: Option<i32>,      // 组织类型
    pub tenant_id: Option<i64>,     // 租户 ID
    pub tenant_org_id: Option<i64>, // 租户组织 ID
    // ... 其他字段
}
```

## 实现步骤

### 步骤 1: 创建扩展结构体

```rust
use org_rust::services::ext::{OrgExtension, OrgLevel};
use org_rust::models::{org::{SysOrg, CreateOrgRequest}, error::AppResult};
use org_rust::services::RequestContext;
use async_trait::async_trait;

/// 你的业务扩展处理器
pub struct MyBusinessExtension {
    // 可以注入数据库连接池、HTTP 客户端等依赖
    db_pool: sqlx::PgPool,
}

impl MyBusinessExtension {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }
}
```

### 步骤 2: 实现 OrgExtension Trait

```rust
#[async_trait]
impl OrgExtension for MyBusinessExtension {
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>> {
        // 1. 根据组织信息创建业务记录
        let business_id = self.create_business_record(org, req).await?;

        // 2. 返回业务 ID，将自动关联到 ext_org_id
        Ok(Some(business_id))
    }

    async fn on_org_delete(
        &self,
        org: &SysOrg,
        ctx: &RequestContext,
    ) -> AppResult<()> {
        // 删除前检查业务数据约束
        self.check_business_constraints(org.id).await?;
        Ok(())
    }

    fn supported_levels(&self) -> Vec<OrgLevel> {
        vec![
            // 方式1: 按业务层级标识匹配（推荐）
            OrgLevel::BusinessLevel("college".to_string()),
            OrgLevel::BusinessLevel("major".to_string()),
            OrgLevel::BusinessLevel("class".to_string()),
        ]
    }
}
```

### 步骤 3: 实现业务逻辑

```rust
impl MyBusinessExtension {
    /// 创建业务记录
    async fn create_business_record(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
    ) -> AppResult<i64> {
        match org.level.as_deref() {
            Some("college") => {
                // 创建学院记录
                let college_id: i64 = sqlx::query_scalar(
                    r#"
                    INSERT INTO t_college (org_id, name, code, tenant_id)
                    VALUES ($1, $2, $3, $4)
                    RETURNING id
                    "#
                )
                .bind(org.id)
                .bind(&org.name)
                .bind(&org.org_code)
                .bind(org.tenant_id)
                .fetch_one(&self.db_pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                Ok(college_id)
            }
            Some("major") => {
                // 创建专业记录
                let major_id: i64 = sqlx::query_scalar(
                    r#"
                    INSERT INTO t_major (org_id, name, code, college_id, tenant_id)
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING id
                    "#
                )
                .bind(org.id)
                .bind(&org.name)
                .bind(&org.org_code)
                .bind(org.pid) // 父组织 ID 即所属学院
                .bind(org.tenant_id)
                .fetch_one(&self.db_pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                Ok(major_id)
            }
            Some("class") => {
                // 创建班级记录
                let class_id: i64 = sqlx::query_scalar(
                    r#"
                    INSERT INTO t_class (org_id, name, code, major_id, tenant_id)
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING id
                    "#
                )
                .bind(org.id)
                .bind(&org.name)
                .bind(&org.org_code)
                .bind(org.pid) // 父组织 ID 即所属专业
                .bind(org.tenant_id)
                .fetch_one(&self.db_pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                Ok(class_id)
            }
            _ => {
                // 不支持的层级，返回 None 跳过扩展
                Ok(0)
            }
        }
    }

    /// 检查业务数据约束
    async fn check_business_constraints(&self, org_id: i64) -> AppResult<()> {
        // 检查是否有学生关联到该组织
        let student_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(1) FROM t_student WHERE class_id = $1"
        )
        .bind(org_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        if student_count > 0 {
            return Err(AppError::BadRequest(
                format!("无法删除：该组织下还有 {} 名学生", student_count)
            ));
        }

        Ok(())
    }
}
```

### 步骤 4: 注册扩展管理器

```rust
use org_rust::services::{OrgService, ext::OrgExtensionManager};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化数据库连接
    let db_pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    // 创建扩展管理器
    let mut extension_manager = OrgExtensionManager::new();

    // 注册你的业务扩展
    extension_manager.register(Arc::new(MyBusinessExtension::new(db_pool.clone())));

    let extension_manager = Arc::new(extension_manager);

    // 创建 OrgService 并注入扩展管理器
    let org_service = OrgService::new(db_pool)
        .with_extension_manager(extension_manager);

    // 现在创建组织时，会自动调用你的扩展实现
    // ...

    Ok(())
}
```

## 完整示例

### 示例 1: 学院管理扩展

```rust
use async_trait::async_trait;
use sqlx::PgPool;
use org_rust::services::ext::{OrgExtension, OrgLevel};
use org_rust::models::{org::{SysOrg, CreateOrgRequest}, error::{AppError, AppResult}};
use org_rust::services::RequestContext;

/// 学院扩展处理器
pub struct CollegeExtension {
    db_pool: PgPool,
}

impl CollegeExtension {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl OrgExtension for CollegeExtension {
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>> {
        // 插入学院表
        let college_id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO t_college (
                org_id, name, full_name, code,
                dean_name, office_phone, address,
                tenant_id, create_time
            ) VALUES (
                $1, $2, $3, $4,
                $5, $6, $7,
                $8, NOW()
            )
            RETURNING id
            "#
        )
        .bind(org.id)
        .bind(&org.name)
        .bind(&org.full_name)
        .bind(&org.org_code)
        .bind("") // dean_name，可从 req 扩展字段获取
        .bind("") // office_phone
        .bind("") // address
        .bind(org.tenant_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| AppError::Internal(format!("创建学院记录失败: {}", e)))?;

        Ok(Some(college_id))
    }

    async fn on_org_delete(
        &self,
        org: &SysOrg,
        ctx: &RequestContext,
    ) -> AppResult<()> {
        // 检查学院下是否有专业
        let major_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(1) FROM t_major WHERE college_id = $1"
        )
        .bind(org.id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        if major_count > 0 {
            return Err(AppError::BadRequest(
                format!("无法删除学院：该学院下还有 {} 个专业", major_count)
            ));
        }

        Ok(())
    }

    fn supported_levels(&self) -> Vec<OrgLevel> {
        vec![
            // 支持业务层级标识为 "college" 的组织
            OrgLevel::BusinessLevel("college".to_string()),
        ]
    }
}
```

### 示例 2: 多层级统一处理

```rust
/// 统一的业务扩展处理器
pub struct UnifiedBusinessExtension {
    db_pool: PgPool,
}

#[async_trait]
impl OrgExtension for UnifiedBusinessExtension {
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>> {
        // 统一业务表结构
        let business_id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO t_business_org (
                org_id, name, code, level,
                parent_id, tenant_id, create_time
            ) VALUES ($1, $2, $3, $4, $5, $6, NOW())
            RETURNING id
            "#
        )
        .bind(org.id)
        .bind(&org.name)
        .bind(&org.org_code)
        .bind(org.level.as_deref().unwrap_or("unknown"))
        .bind(org.pid)
        .bind(org.tenant_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(Some(business_id))
    }

    async fn on_org_delete(&self, org: &SysOrg, ctx: &RequestContext) -> AppResult<()> {
        // 删除业务记录
        sqlx::query("DELETE FROM t_business_org WHERE org_id = $1")
            .bind(org.id)
            .execute(&self.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    fn supported_levels(&self) -> Vec<OrgLevel> {
        vec![
            // 支持所有业务层级
            OrgLevel::BusinessLevel("college".to_string()),
            OrgLevel::BusinessLevel("major".to_string()),
            OrgLevel::BusinessLevel("class".to_string()),
        ]
    }
}
```

### 示例 3: HTTP 调用外部服务

```rust
use reqwest::Client;

/// 通过 HTTP 调用外部系统的扩展
pub struct HttpExtension {
    http_client: Client,
    base_url: String,
}

impl HttpExtension {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl OrgExtension for HttpExtension {
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>> {
        // 构造请求体
        let payload = serde_json::json!({
            "org_id": org.id,
            "name": org.name,
            "code": org.org_code,
            "level": org.level,
            "parent_id": org.pid,
        });

        // 调用外部 API
        let response = self.http_client
            .post(format!("{}/api/orgs", self.base_url))
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("HTTP 请求失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Internal(
                format!("外部服务返回错误: {}", response.status())
            ));
        }

        // 解析响应获取外部 ID
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let external_id = result["id"]
            .as_i64()
            .ok_or_else(|| AppError::Internal("无效的响应格式".to_string()))?;

        Ok(Some(external_id))
    }

    async fn on_org_delete(&self, org: &SysOrg, ctx: &RequestContext) -> AppResult<()> {
        let response = self.http_client
            .delete(format!("{}/api/orgs/{}", self.base_url, org.id))
            .send()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::Internal(
                format!("删除外部组织失败: {}", response.status())
            ));
        }

        Ok(())
    }

    fn supported_levels(&self) -> Vec<OrgLevel> {
        // 支持所有层级
        vec![]
    }
}
```

## 测试验证

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> PgPool {
        sqlx::PgPool::connect("postgresql://test:test@localhost/test_db")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_college_extension() {
        let pool = setup_test_db().await;
        let extension = CollegeExtension::new(pool);

        // 模拟组织数据
        let org = SysOrg {
            id: 1,
            name: "计算机学院".to_string(),
            full_name: Some("计算机科学与技术学院".to_string()),
            org_code: Some("CS001".to_string()),
            level: Some("college".to_string()),
            tenant_id: Some(1),
            ..Default::default()
        };

        let req = CreateOrgRequest {
            name: "计算机学院".to_string(),
            ..Default::default()
        };

        let ctx = RequestContext::default();

        // 测试创建
        let result = extension.on_org_created(&org, &req, &ctx).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // 测试删除约束检查
        let delete_result = extension.on_org_delete(&org, &ctx).await;
        assert!(delete_result.is_ok());
    }
}
```

### 集成测试

```rust
#[tokio::test]
async fn test_extension_integration() {
    // 1. 设置测试环境
    let pool = setup_test_db().await;
    let mut extension_manager = OrgExtensionManager::new();
    extension_manager.register(Arc::new(CollegeExtension::new(pool.clone())));

    // 2. 创建组织
    let org_service = OrgService::new(pool.clone())
        .with_extension_manager(Arc::new(extension_manager));

    let req = CreateOrgRequest {
        name: "测试学院".to_string(),
        level: Some("college".to_string()),
        ..Default::default()
    };

    let ctx = RequestContext {
        org_id: Some(1),
        ..Default::default()
    };

    // 3. 验证扩展被调用
    let new_org_id = org_service.create_child(1, req, ctx).await.unwrap();

    // 4. 验证 ext_org_id 被正确设置
    let ext_org: Option<(i64,)> = sqlx::query_as(
        "SELECT ext_org_id FROM t_sys_org_ext WHERE id = $1"
    )
    .bind(new_org_id)
    .fetch_optional(&pool)
    .await
    .unwrap();

    assert!(ext_org.is_some());
    assert!(ext_org.unwrap().0 > 0);
}
```

## 调试技巧

### 1. 日志记录

```rust
use tracing::{info, error, debug};

#[async_trait]
impl OrgExtension for MyExtension {
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>> {
        info!(
            org_id = org.id,
            org_name = %org.name,
            org_level = ?org.level,
            "Creating business record"
        );

        match self.create_business_record(org).await {
            Ok(id) => {
                info!(business_id = id, "Business record created successfully");
                Ok(Some(id))
            }
            Err(e) => {
                error!(error = %e, "Failed to create business record");
                Err(e)
            }
        }
    }
}
```

### 2. 事务回滚处理

```rust
async fn on_org_created(
    &self,
    org: &SysOrg,
    req: &CreateOrgRequest,
    ctx: &RequestContext,
) -> AppResult<Option<i64>> {
    // 使用事务确保业务操作原子性
    let mut tx = self.db_pool.begin().await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let result = self.create_with_transaction(&mut tx, org).await;

    match result {
        Ok(id) => {
            tx.commit().await.map_err(|e| AppError::Internal(e.to_string()))?;
            Ok(Some(id))
        }
        Err(e) => {
            // 自动回滚
            Err(e)
        }
    }
}
```

## 常见问题

### Q1: 如何区分不同层级的处理？

**A**: 使用 `OrgLevel` 声明支持的层级，框架会自动匹配：

```rust
fn supported_levels(&self) -> Vec<OrgLevel> {
    vec![
        // 方式1: 业务层级标识（推荐）
        OrgLevel::BusinessLevel("college".to_string()),

        // 方式2: 物理层级
        OrgLevel::NodeLevel(2), // 第2层
    ]
}
```

### Q2: 扩展执行失败会怎样？

**A**: 扩展返回错误会导致整个组织创建事务回滚：

```rust
// 返回错误 - 组织创建将回滚
return Err(AppError::Internal("创建业务记录失败".to_string()));

// 返回 None - 跳过扩展，继续创建
return Ok(None);
```

### Q3: 如何在扩展中访问请求数据？

**A**: 通过 `CreateOrgRequest` 和 `RequestContext`：

```rust
async fn on_org_created(
    &self,
    org: &SysOrg,
    req: &CreateOrgRequest,
    ctx: &RequestContext,
) -> AppResult<Option<i64>> {
    // 访问请求数据
    let name = &req.name;
    let tenant_id = ctx.tenant_org_id;

    // 访问组织数据
    let org_id = org.id;
    let level = &org.level;

    // ...
}
```

### Q4: 如何处理扩展间的依赖？

**A**: 使用 `supported_levels` 控制执行顺序：

```rust
// 学院先执行
impl OrgExtension for CollegeExtension {
    fn supported_levels(&self) -> Vec<OrgLevel> {
        vec![OrgLevel::NodeLevel(2)]
    }
}

// 专业后执行（依赖学院）
impl OrgExtension for MajorExtension {
    fn supported_levels(&self) -> Vec<OrgLevel> {
        vec![OrgLevel::NodeLevel(3)]
    }
}
```

### Q5: 如何实现条件扩展？

**A**: 在 `on_org_created` 中根据条件返回：

```rust
async fn on_org_created(
    &self,
    org: &SysOrg,
    req: &CreateOrgRequest,
    ctx: &RequestContext,
) -> AppResult<Option<i64>> {
    // 条件判断
    if org.level.as_ref() != Some(&"college".to_string()) {
        return Ok(None); // 跳过扩展
    }

    // 执行扩展逻辑
    let business_id = self.create_business_record(org).await?;
    Ok(Some(business_id))
}
```

## 最佳实践

### 1. 错误处理

```rust
// 使用具体错误信息
return Err(AppError::Internal(
    format!("创建学院记录失败: {}", e)
));

// 而不是
return Err(AppError::Internal("失败".to_string()));
```

### 2. 幂等性

```rust
async fn on_org_created(
    &self,
    org: &SysOrg,
    req: &CreateOrgRequest,
    ctx: &RequestContext,
) -> AppResult<Option<i64>> {
    // 先检查是否已存在
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM t_college WHERE org_id = $1"
    )
    .bind(org.id)
    .fetch_optional(&self.db_pool)
    .await?;

    if let Some(id) = existing {
        return Ok(Some(id)); // 返回已存在的 ID
    }

    // 创建新记录
    // ...
}
```

### 3. 性能优化

```rust
// 批量处理
async fn on_org_created(
    &self,
    org: &SysOrg,
    req: &CreateOrgRequest,
    ctx: &RequestContext,
) -> AppResult<Option<i64>> {
    // 使用批量插入
    let business_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO t_business_org (org_id, name, code)
        VALUES ($1, $2, $3)
        ON CONFLICT (org_id) DO UPDATE
        SET name = EXCLUDED.name,
            code = EXCLUDED.code
        RETURNING id
        "#
    )
    .bind(org.id)
    .bind(&org.name)
    .bind(&org.org_code)
    .fetch_one(&self.db_pool)
    .await?;

    Ok(Some(business_id))
}
```

## 附录

### 数据库表结构示例

```sql
-- 学院表
CREATE TABLE t_college (
    id BIGSERIAL PRIMARY KEY,
    org_id BIGINT NOT NULL UNIQUE,  -- 关联 t_sys_org.id
    name VARCHAR(100) NOT NULL,
    full_name VARCHAR(200),
    code VARCHAR(50),
    dean_name VARCHAR(50),
    office_phone VARCHAR(20),
    address VARCHAR(200),
    tenant_id BIGINT,
    create_time TIMESTAMP DEFAULT NOW(),
    update_time TIMESTAMP DEFAULT NOW()
);

-- 专业表
CREATE TABLE t_major (
    id BIGSERIAL PRIMARY KEY,
    org_id BIGINT NOT NULL UNIQUE,  -- 关联 t_sys_org.id
    college_id BIGINT,               -- 所属学院
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50),
    tenant_id BIGINT,
    create_time TIMESTAMP DEFAULT NOW()
);

-- 班级表
CREATE TABLE t_class (
    id BIGSERIAL PRIMARY KEY,
    org_id BIGINT NOT NULL UNIQUE,  -- 关联 t_sys_org.id
    major_id BIGINT,                 -- 所属专业
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50),
    tenant_id BIGINT,
    create_time TIMESTAMP DEFAULT NOW()
);
```

### 扩展注册完整示例

```rust
use org_rust::services::{OrgService, ext::OrgExtensionManager};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 初始化数据库
    let db_pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    // 2. 创建扩展管理器
    let mut extension_manager = OrgExtensionManager::new();

    // 3. 注册所有扩展处理器
    extension_manager.register(Arc::new(CollegeExtension::new(db_pool.clone())));
    extension_manager.register(Arc::new(MajorExtension::new(db_pool.clone())));
    extension_manager.register(Arc::new(ClassExtension::new(db_pool.clone())));

    // 4. 创建 OrgService 并注入扩展管理器
    let org_service = OrgService::new(db_pool)
        .with_extension_manager(Arc::new(extension_manager));

    // 5. 使用 org_service 创建组织
    let req = CreateOrgRequest {
        name: "计算机学院".to_string(),
        level: Some("college".to_string()),
        ..Default::default()
    };

    let ctx = RequestContext {
        org_id: Some(1),
        tenant_org_id: Some(1),
        appid: None,
    };

    let new_org_id = org_service.create_child(1, req, ctx).await?;
    println!("创建组织成功，ID: {}", new_org_id);

    Ok(())
}
```

## 支持

如有问题，请联系 `org-rust` 维护团队或查阅相关文档。
