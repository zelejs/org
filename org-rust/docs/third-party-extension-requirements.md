# 组织扩展接口需求规格说明书

## 文档信息

| 项目 | 内容 |
|------|------|
| 文档版本 | v1.0 |
| 创建日期 | 2026-04-11 |
| 适用框架 | org-rust (Rust) |
| 目标读者 | 第三方业务系统开发者 |

---

## 1. 需求概述

### 1.1 背景

组织服务（org-rust）在创建组织时，需要支持第三方业务系统同步创建自己的业务表记录，并通过 `ext_org_id` 字段进行关联。

### 1.2 目标

- 定义统一的扩展接口规范
- 支持第三方系统实现业务逻辑
- 保证数据一致性和事务完整性

### 1.3 范围

- 组织创建时的扩展处理
- 组织删除前的扩展处理
- 按层级自动分发扩展调用

---

## 2. 功能需求

### 2.1 核心功能

| 功能编号 | 功能名称 | 描述 |
|----------|----------|------|
| F-001 | 扩展接口定义 | 提供 `OrgExtension` trait 供第三方实现 |
| F-002 | 层级匹配 | 按组织的 `level` 或 `node_level` 自动匹配扩展处理器 |
| F-003 | 创建钩子 | 组织创建成功后调用扩展接口 |
| F-004 | 删除钩子 | 组织删除前调用扩展接口进行约束检查 |
| F-005 | ext_org_id 关联 | 扩展返回的 ID 自动存入 `t_sys_org_ext` 表 |

### 2.2 接口契约

#### 2.2.1 OrgExtension Trait

```rust
#[async_trait]
pub trait OrgExtension: Send + Sync {
    /// 组织创建后的扩展处理
    ///
    /// # 返回值
    /// - Ok(Some(id)): 业务表 ID，将存入 ext_org_id
    /// - Ok(None): 不需要扩展，跳过关联
    /// - Err(e): 扩展失败，事务回滚
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>>;

    /// 组织删除前的扩展处理
    ///
    /// # 返回值
    /// - Ok(()): 允许删除
    /// - Err(e): 不允许删除，组织将不被删除
    async fn on_org_delete(
        &self,
        org: &SysOrg,
        ctx: &RequestContext,
    ) -> AppResult<()>;

    /// 声明支持的层级
    fn supported_levels(&self) -> Vec<OrgLevel>;
}
```

#### 2.2.2 OrgLevel 枚举

```rust
pub enum OrgLevel {
    /// 业务层级标识，如 "college", "major", "class"
    BusinessLevel(String),

    /// 物理层级，如 1, 2, 3
    NodeLevel(i32),
}
```

### 2.3 层级匹配规则

| 优先级 | 匹配方式 | 说明 | 示例 |
|--------|----------|------|------|
| 1 | BusinessLevel | 按 `org.level` 字段匹配 | "college", "major", "class" |
| 2 | NodeLevel | 按 `org.node_level` 字段匹配 | 1, 2, 3 |

---

## 3. 数据结构

### 3.1 输入参数

#### SysOrg（组织信息）

| 字段 | 类型 | 说明 | 示例 |
|------|------|------|------|
| id | i64 | 组织 ID | 123 |
| pid | Option\<i64\> | 父组织 ID | Some(100) |
| name | String | 组织名称 | "计算机学院" |
| full_name | Option\<String\> | 组织全称 | Some("计算机科学与技术学院") |
| org_code | Option\<String\> | 组织代码 | Some("CS001") |
| node_level | Option\<i32\> | 物理层级 | Some(2) |
| level | Option\<String\> | 业务层级标识 | Some("college") |
| org_type | Option\<i32\> | 组织类型 | Some(2) |
| tenant_id | Option\<i64\> | 租户 ID | Some(1) |
| tenant_org_id | Option\<i64\> | 租户组织 ID | Some(10) |

#### CreateOrgRequest（创建请求）

| 字段 | 类型 | 说明 |
|------|------|------|
| name | String | 组织名称 |
| full_name | Option\<String\> | 组织全称 |
| org_code | Option\<String\> | 组织代码 |
| note | Option\<String\> | 备注 |
| org_type | Option\<i32\> | 组织类型 |
| level | Option\<String\> | 业务层级 |

#### RequestContext（请求上下文）

| 字段 | 类型 | 说明 |
|------|------|------|
| org_id | Option\<i64\> | 当前组织 ID |
| tenant_org_id | Option\<i64\> | 租户组织 ID |
| appid | Option\<String\> | 应用 ID |

### 3.2 输出参数

| 返回值 | 说明 | 后续处理 |
|--------|------|----------|
| Ok(Some(id)) | 返回业务表 ID | 插入 t_sys_org_ext 表 |
| Ok(None) | 不需要扩展 | 跳过 ext_org_id 关联 |
| Err(e) | 扩展失败 | 整个事务回滚 |

---

## 4. 业务流程

### 4.1 组织创建流程

```
┌─────────────────────────────────────────────────────────────┐
│                    创建组织请求                              │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    开始数据库事务                             │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              更新 left_num / right_num                       │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                 插入 t_sys_org 记录                           │
│              获取新组织的 ID 和完整信息                        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              [扩展管理器] 查找匹配的扩展处理器                   │
│              按 level 或 node_level 匹配                       │
└─────────────────────────────────────────────────────────────┘
                            │
                    ┌───────┴───────┐
                    │               │
                    ▼               ▼
            ┌───────────┐   ┌───────────┐
            │  找到扩展  │   │  未找到   │
            └───────────┘   └───────────┘
                    │               │
                    ▼               ▼
        ┌───────────────┐   ┌───────────────┐
        │ 调用扩展接口   │   │   跳过扩展    │
        │ on_org_created│   │              │
        └───────────────┘   └───────────────┘
                │
        ┌───────┼───────┐
        │       │       │
        ▼       ▼       ▼
   ┌─────┐ ┌─────┐ ┌─────┐
   │成功 │ │None │ │失败 │
   └─────┘ └─────┘ └─────┘
        │       │       │
        ▼       ▼       ▼
   ┌─────┐ ┌─────┐ ┌─────┐
   │插入 │ │跳过 │ │回滚 │
   │ext  │ │关联 │ │事务 │
   │表   │ │     │ │     │
   └─────┘ └─────┘ └─────┘
        │       │       │
        └───────┴───────┘
                │
                ▼
┌─────────────────────────────────────────────────────────────┐
│                    提交事务                                   │
└─────────────────────────────────────────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────────────────────────┐
│                  返回组织 ID                                  │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 组织删除流程

```
┌─────────────────────────────────────────────────────────────┐
│                    删除组织请求                              │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              [扩展管理器] 查找匹配的扩展处理器                   │
└─────────────────────────────────────────────────────────────┘
                            │
                    ┌───────┴───────┐
                    │               │
                    ▼               ▼
            ┌───────────┐   ┌───────────┐
            │  找到扩展  │   │  未找到   │
            └───────────┘   └───────────┘
                    │               │
                    ▼               ▼
        ┌───────────────┐   ┌───────────────┐
        │ 调用扩展接口   │   │   继续删除    │
        │ on_org_delete │   │              │
        └───────────────┘   └───────────────┘
                │
        ┌───────┴───────┐
        │               │
        ▼               ▼
   ┌─────────┐   ┌─────────┐
   │ 通过检查 │   │ 检查失败 │
   └─────────┘   └─────────┘
        │               │
        ▼               ▼
┌───────────┐   ┌───────────┐
│  执行删除  │   │ 返回错误   │
└───────────┘   └───────────┘
```

---

## 5. 集成要求

### 5.1 依赖要求

```toml
[dependencies]
org-rust = "0.1.0"
async-trait = "0.1"
tokio = { version = "1.35", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono"] }
```

### 5.2 实现要求

| 要求编号 | 要求描述 | 优先级 |
|----------|----------|--------|
| R-001 | 实现 `OrgExtension` trait | 必须 |
| R-002 | `on_org_created` 返回业务表 ID 或 None | 必须 |
| R-003 | `on_org_delete` 检查业务约束 | 必须 |
| R-004 | `supported_levels` 声明支持的层级 | 必须 |
| R-005 | 异步方法使用 `async/await` | 必须 |
| R-006 | 错误返回 `AppError` 类型 | 必须 |

### 5.3 性能要求

| 指标 | 要求 | 说明 |
|------|------|------|
| 响应时间 | < 500ms | 单次扩展调用 |
| 并发支持 | 支持 | 必须线程安全（Send + Sync） |
| 事务一致性 | 强一致 | 扩展失败需回滚整个事务 |

### 5.4 可靠性要求

| 要求 | 说明 |
|------|------|
| 幂等性 | 重复调用应返回相同结果 |
| 错误处理 | 扩展失败必须返回错误，不能静默失败 |
| 日志记录 | 关键操作需记录日志 |

---

## 6. 验收标准

### 6.1 功能验收

| 测试项 | 测试内容 | 预期结果 |
|--------|----------|----------|
| TC-001 | 创建支持层级的组织 | 扩展被正确调用 |
| TC-002 | 创建不支持层级的组织 | 扩展不被调用 |
| TC-003 | 扩展返回 Some(id) | ext_org_id 正确关联 |
| TC-004 | 扩展返回 None | 跳过 ext_org_id 关联 |
| TC-005 | 扩展返回错误 | 组织创建失败，事务回滚 |
| TC-006 | 删除有约束的组织 | 返回错误，组织不被删除 |
| TC-007 | 删除无约束的组织 | 成功删除 |

### 6.2 集成验收

```rust
// 验收测试示例
#[tokio::test]
async fn test_extension_integration() {
    // 1. 注册扩展
    let mut manager = OrgExtensionManager::new();
    manager.register(Arc::new(MyExtension::new()));

    // 2. 创建服务
    let service = OrgService::new(pool)
        .with_extension_manager(Arc::new(manager));

    // 3. 创建组织
    let req = CreateOrgRequest {
        name: "测试组织".to_string(),
        level: Some("college".to_string()),
        ..Default::default()
    };

    let org_id = service.create_child(1, req, &ctx).await.unwrap();

    // 4. 验证 ext_org_id
    let ext_id: Option<i64> = sqlx::query_scalar(
        "SELECT ext_org_id FROM t_sys_org_ext WHERE id = $1"
    )
    .bind(org_id)
    .fetch_optional(&pool)
    .await
    .unwrap();

    assert!(ext_id.is_some()); // 验收通过
}
```

---

## 7. 错误码

| 错误码 | 错误信息 | 说明 |
|--------|----------|------|
| EXT-001 | 扩展调用失败 | on_org_created 返回错误 |
| EXT-002 | 扩展返回无效 ID | 返回的 ID <= 0 |
| EXT-003 | 扩展删除检查失败 | on_org_delete 返回错误 |
| EXT-004 | 扩展未注册 | 没有匹配的扩展处理器 |

---

## 8. 附录

### 8.1 完整实现示例

```rust
use async_trait::async_trait;
use sqlx::PgPool;
use org_rust::services::ext::{OrgExtension, OrgLevel};
use org_rust::models::{org::{SysOrg, CreateOrgRequest}, error::{AppError, AppResult}};
use org_rust::services::RequestContext;

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
        .map_err(|e| AppError::Internal(format!("创建学院记录失败: {}", e)))?;

        Ok(Some(college_id))
    }

    async fn on_org_delete(
        &self,
        org: &SysOrg,
        ctx: &RequestContext,
    ) -> AppResult<()> {
        // 检查学院下是否有专业
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(1) FROM t_major WHERE college_id = $1"
        )
        .bind(org.id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        if count > 0 {
            return Err(AppError::BadRequest("学院下还有专业，无法删除".to_string()));
        }

        Ok(())
    }

    fn supported_levels(&self) -> Vec<OrgLevel> {
        vec![OrgLevel::BusinessLevel("college".to_string())]
    }
}
```

### 8.2 注册示例

```rust
use org_rust::services::{OrgService, ext::OrgExtensionManager};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    // 创建扩展管理器
    let mut extension_manager = OrgExtensionManager::new();

    // 注册扩展
    extension_manager.register(Arc::new(CollegeExtension::new(db_pool.clone())));
    extension_manager.register(Arc::new(MajorExtension::new(db_pool.clone())));

    // 创建服务
    let org_service = OrgService::new(db_pool)
        .with_extension_manager(Arc::new(extension_manager));

    Ok(())
}
```

### 8.3 数据库表结构示例

```sql
-- t_sys_org_ext 表（系统提供）
CREATE TABLE t_sys_org_ext (
    id BIGINT PRIMARY KEY,           -- 关联 t_sys_org.id
    ext_org_id BIGINT NOT NULL,      -- 业务表 ID
    ext_org_type INT,                -- 扩展组织类型
    delete_flag INT DEFAULT 0,
    create_time TIMESTAMP DEFAULT NOW(),
    update_time TIMESTAMP DEFAULT NOW()
);

-- 业务表示例（第三方自行设计）
CREATE TABLE t_college (
    id BIGSERIAL PRIMARY KEY,
    org_id BIGINT NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50),
    tenant_id BIGINT,
    create_time TIMESTAMP DEFAULT NOW()
);
```

---

## 9. 联系方式

| 项目 | 内容 |
|------|------|
| 技术支持 | org-rust 维护团队 |
| 文档地址 | `/home/ubuntu/workspace/edu/auth/org/org-rust/docs/` |
| 实现指南 | `third-party-extension-guide.md` |
