# org-rust

参考 `eav/eav-rust` 的 `axum + sqlx` 分层风格重建的组织服务脚手架，接口与原 Java `auth/org` 的核心路由保持一致。

## 更新日志

### 2026-04-11 - 第三方扩展接口

**新增功能：第三方业务扩展接口**

组织创建时支持调用第三方业务接口，实现业务组织表与 `t_sys_org.ext_org_id` 的关联。

- 定义通用 `OrgExtension` trait，支持第三方实现
- 按层级（`node_level` 或 `level`）自动分发扩展调用
- 支持组织创建和删除的扩展钩子
- 完全可选，不影响现有功能
- 详细文档：[第三方业务扩展实现指南](docs/third-party-extension-guide.md)

## 功能特性

- **组织树管理**：基于 Nested Set Model（嵌套集合模型）的高效层级查询
- **CLI 工具**：完整的命令行工具，支持组织树的增删改查、导入导出
- **REST API**：与原 Java 版本兼容的 HTTP 接口
- **多租户支持**：通过 `appid` 实现多应用组织隔离
- **第三方扩展**：支持业务系统在组织创建/删除时扩展自定义逻辑（2026-04-11 新增）

## 快速开始

### 1. 构建

```bash
# 使用构建脚本（推荐）
./build.sh

# 或使用 cargo
cargo build --release
```

### 2. 运行模式

#### 服务器模式（默认）
```bash
# 使用构建脚本
./build.sh --run-server

# 或直接运行
cargo run
```

#### CLI 模式
```bash
# 查看帮助
cargo run -- --help
cargo run -- org --help

# 或使用构建的二进制
./target/release/org-cli --help
```

## CLI 使用指南

### 全局选项

```bash
org-cli [OPTIONS] <COMMAND>

OPTIONS:
  --db-url <URL>      数据库连接 URL（默认从 DATABASE_URL 环境变量读取）
  --appid <APPID>     应用 ID，用于多租户隔离

COMMANDS:
  org                 组织管理命令
```

### 组织管理命令

#### 1. 初始化根组织

```bash
# 初始化平台根组织（appid=null）
org-cli org init

# 初始化指定应用的根组织
org-cli org init --appid "myapp"

# 初始化带名称的根组织
org-cli org init --appid "myapp" --name "我的应用组织"
```

#### 2. 查看组织树

```bash
# 显示 ID=1 的组织子树（JSON 格式）
org-cli org show 1

# 显示树形图
org-cli org show 1 --format tree
```

#### 3. 列出所有根组织

```bash
# 列出所有根组织（表格格式）
org-cli org roots --format table

# 列出所有根组织（JSON 格式）
org-cli org roots
```

#### 4. 插入子组织

```bash
# 在 ID=5 的组织下插入 "技术部"
org-cli org insert 5 --name "技术部"

# 插入带类型的组织
org-cli org insert 5 --name "北京分公司" --org-type 2 --note "华北地区"

# 组织类型：0=平台, 1=租户, 2=分支机构, 3=部门, 4=用户
```

#### 5. 更新组织

```bash
# 更新组织名称
org-cli org update 10 --name "新名称"

# 更新多个属性
org-cli org update 10 --name "新名称" --note "更新备注" --org-type 3
```

#### 6. 搜索组织

```bash
# 按名称或代码搜索
org-cli org search "技术"

# 限制结果数量
org-cli org search "部" --limit 10

# JSON 格式输出
org-cli org search "技术" --format json
```

#### 7. 移动组织

```bash
# 将 ID=10 的组织移动到 ID=5 下
org-cli org move 10 5
```

#### 8. 移除组织

```bash
# 移除组织（如果有子组织则拒绝）
org-cli org remove 10

# 强制移除（包括子组织）
org-cli org remove 10 --force
```

#### 9. 验证树结构

```bash
# 验证所有组织树的完整性
org-cli org validate

# 验证指定组织的子树
org-cli org validate --org-id 1
```

#### 10. 导出组织树

```bash
# 导出全部组织树到文件
org-cli org export --output org-tree.json

# 导出指定组织的子树
org-cli org export --org-id 5 --output subtree.json
```

#### 11. 导入组织树

```bash
# 预演导入（仅验证）
org-cli org import --file org-tree.json --dry-run

# 从文件导入
org-cli org import --file org-tree.json

# 从标准输入导入
cat org-tree.json | org-cli org import
```

## 使用示例

### 完整工作流示例

```bash
# 1. 初始化平台根组织
org-cli org init

# 2. 查看根组织
org-cli org roots --format table

# 3. 创建公司组织
org-cli org insert 1 --name "总公司" --org-type 1

# 4. 创建部门
org-cli org insert 2 --name "技术部" --org-type 3
org-cli org insert 2 --name "市场部" --org-type 3

# 5. 查看树形结构
org-cli org show 1 --format tree

# 6. 搜索部门
org-cli org search "部"

# 7. 导出组织树
org-cli org export --output company.json

# 8. 验证树结构
org-cli org validate
```

### 输出示例

**树形图输出：**
```
[1] 根组织 (Platform)
└── [2] 总公司 (Tenant)
    ├── [3] 技术部 (Department)
    └── [4] 市场部 (Department)
```

**表格输出：**
```
ID  | AppID   | Name     | Type        | Children
----|---------|----------|-------------|----------
1   | NULL    | 根组织   | Platform    | 1
2   | myapp   | 总公司   | Tenant      | 2
```

## 核心 API 接口

### 组织管理
- `GET /health` - 健康检查
- `POST /api/adm/org/:id/children` - 创建子组织
- `DELETE /api/adm/org/:id` - 删除组织
- `PUT /api/adm/org/:id` - 更新组织
- `GET /api/adm/org/:id` - 获取组织详情
- `GET /api/adm/org` - 分页查询组织
- `GET /api/adm/org/tree` - 获取组织树

### 外部组织同步
- `POST /api/adm/sys/extOrg/add` - 添加外部组织
- `DELETE /api/adm/sys/extOrg/delete?id=...` - 删除外部组织
- `PUT /api/adm/sys/extOrg/update` - 更新外部组织
- `GET /api/adm/sys/extOrg/list` - 查询外部组织
- `POST /api/adm/sys/extOrg/sync` - 同步外部组织

## 第三方业务扩展

### 概述

第三方业务扩展允许在组织创建/删除时执行自定义业务逻辑，实现业务组织表与 `t_sys_org.ext_org_id` 的关联。

### 应用场景

- 创建组织时同步创建业务表（如学院表、专业表）
- 删除组织前检查业务数据约束
- 通过 HTTP 调用外部系统同步组织数据

### 快速开始

#### 1. 实现 OrgExtension Trait

```rust
use async_trait::async_trait;
use org_rust::services::ext::{OrgExtension, OrgLevel};
use org_rust::models::{org::{SysOrg, CreateOrgRequest}, error::AppResult};
use org_rust::services::RequestContext;

pub struct MyBusinessExtension {
    db_pool: sqlx::PgPool,
}

#[async_trait]
impl OrgExtension for MyBusinessExtension {
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>> {
        // 创建业务记录，返回业务 ID
        let business_id: i64 = sqlx::query_scalar(
            "INSERT INTO t_my_business (org_id, name) VALUES ($1, $2) RETURNING id"
        )
        .bind(org.id)
        .bind(&org.name)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(Some(business_id))
    }

    async fn on_org_delete(
        &self,
        org: &SysOrg,
        ctx: &RequestContext,
    ) -> AppResult<()> {
        // 删除前检查
        Ok(())
    }

    fn supported_levels(&self) -> Vec<OrgLevel> {
        // 声明支持的层级
        vec![
            OrgLevel::BusinessLevel("college".to_string()),
            OrgLevel::NodeLevel(2),
        ]
    }
}
```

#### 2. 注册扩展管理器

```rust
use org_rust::services::{OrgService, ext::OrgExtensionManager};
use std::sync::Arc;

// 创建扩展管理器
let mut extension_manager = OrgExtensionManager::new();
extension_manager.register(Arc::new(MyBusinessExtension::new(db_pool.clone())));

// 创建 OrgService 并注入扩展管理器
let org_service = OrgService::new(db_pool)
    .with_extension_manager(Arc::new(extension_manager));
```

#### 3. 创建组织

```rust
// 创建组织时，扩展会自动被调用
let req = CreateOrgRequest {
    name: "计算机学院".to_string(),
    level: Some("college".to_string()),
    ..Default::default()
};

let org_id = org_service.create_child(parent_id, req, &ctx).await?;
// ext_org_id 会自动关联到 t_sys_org_ext 表
```

### 扩展接口说明

#### OrgExtension Trait

```rust
#[async_trait]
pub trait OrgExtension: Send + Sync {
    /// 组织创建后的扩展处理
    async fn on_org_created(
        &self,
        org: &SysOrg,
        req: &CreateOrgRequest,
        ctx: &RequestContext,
    ) -> AppResult<Option<i64>>;

    /// 组织删除前的扩展处理
    async fn on_org_delete(
        &self,
        org: &SysOrg,
        ctx: &RequestContext,
    ) -> AppResult<()>;

    /// 声明支持的层级
    fn supported_levels(&self) -> Vec<OrgLevel>;
}
```

#### 层级匹配规则

扩展管理器按以下优先级匹配扩展处理器：

1. **业务层级（level）**：如 `school`、`college`、`major`、`class`
2. **物理层级（node_level）**：如 `1`、`2`、`3`

```rust
// 方式1: 按业务层级标识匹配（推荐）
OrgLevel::BusinessLevel("college".to_string())

// 方式2: 按物理层级匹配
OrgLevel::NodeLevel(2) // 第2层
```

### 调用流程

```
创建组织请求
    ↓
insert_child_org (开始事务)
    ↓
更新 left_num/right_num
    ↓
插入 t_sys_org 记录
    ↓
[扩展管理器] 按 level 查找匹配的扩展处理器
    ↓
[扩展处理器] on_org_created() → 返回 ext_org_id
    ↓
插入 t_sys_org_ext 记录
    ↓
提交事务
    ↓
返回 org_id
```

### 事务处理

- 扩展调用在同一事务中执行
- 扩展返回错误会导致整个事务回滚
- 扩展返回 `Ok(None)` 跳过 `ext_org_id` 关联

### 完整文档

详细的实现指南和示例代码请参考：[第三方业务扩展实现指南](docs/third-party-extension-guide.md)

## 鉴权上下文

当前用请求头透传 JWT 上下文（用于替代 Java 版 `JWTKit`）：

- `x-org-id` - 组织 ID
- `x-tenant-org-id` - 租户组织 ID
- `x-appid` - 应用 ID

## 环境变量

```bash
# 数据库连接
DATABASE_URL=postgresql://postgres:postgres@127.0.0.1:5432/enrollment

# 服务器配置
HOST=0.0.0.0
PORT=8082

# 日志级别
RUST_LOG=org_rust=info,tower_http=info
```

## 项目结构

```
org-rust/
├── src/
│   ├── cli/           # CLI 命令实现
│   │   ├── init.rs    # 初始化命令
│   │   ├── show.rs    # 显示命令
│   │   ├── insert.rs  # 插入命令
│   │   ├── remove.rs  # 移除命令
│   │   ├── update.rs  # 更新命令
│   │   ├── search.rs  # 搜索命令
│   │   ├── move.rs    # 移动命令
│   │   ├── export.rs  # 导出命令
│   │   ├── import.rs  # 导入命令
│   │   └── validate.rs # 验证命令
│   ├── services/
│   │   ├── ext/               # 第三方扩展模块 (2026-04-11 新增)
│   │   │   ├── mod.rs         # 扩展模块声明
│   │   │   ├── org_extension.rs       # OrgExtension trait 定义
│   │   │   └── org_extension_manager.rs  # 扩展管理器
│   │   ├── org_core.rs    # 核心业务逻辑
│   │   ├── org_service.rs # 服务层
│   │   └── ext_org_service.rs # 扩展组织服务
│   └── ...
├── docs/
│   └── third-party-extension-guide.md  # 第三方扩展实现指南
├── build.sh            # 构建脚本
└── Cargo.toml
```
