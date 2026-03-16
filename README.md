# Org 模块

组织管理模块，提供组织架构的增删改查、树形结构查询等功能。

## 功能特性

- 组织的 CRUD 操作
- 树形结构查询
- 基于 Nested Set 模型的层级管理
- 多租户支持
- 应用级别（appid）的数据隔离

## 核心概念

### filterType 查询过滤类型

`filterType` 用于控制组织查询的过滤行为，定义在 `BaseQueryParam` 中：

| filterType | 常量名 | 触发条件 | 查询范围 | 说明 |
|------------|--------|----------|----------|------|
| **0** | - | `orgId = 1` 或显式设置 | **所有匹配 appid 的组织** | 无额外过滤，返回完整组织树 |
| **1** | `FILTER_CHILDREN` | `orgId != 1 && filterType == 1` | **指定节点的所有子孙节点** | 只按组织层级过滤，不限制租户 |
| **2** | `FILTER_CHILDREN_AND_TENANT` | `orgId != 1 && filterType == 2` | **指定节点的子孙节点 + 同租户** | 按层级和租户过滤，最严格 |
| **null** | - | 不匹配任何条件 | 所有匹配 appid 的组织 | 相当于 0 |

### filterType 详细说明

#### filterType = 2 (默认值，租户隔离)

这是**默认的过滤类型**，用于多租户系统中的租户管理员场景。

**SQL 逻辑：**
```sql
AND id IN (
    SELECT c.id
    FROM t_sys_org f
    LEFT JOIN t_sys_org c ON (
        f.left_num <= c.left_num
        AND c.right_num <= f.right_num
        AND c.delete_flag = 0
        AND c.tenant_org_id = #{tenantOrgId}  -- 租户过滤
    )
    WHERE f.id = #{orgId}
    AND f.delete_flag = 0
)
```

**查询意义：** 返回指定组织节点（orgId）下的所有子孙节点，**且这些节点的 `tenant_org_id` 必须等于当前用户的租户ID**。

**使用场景：** 多租户系统中，租户管理员只能看到自己租户下的组织数据。

**示例：**
```java
// 租户管理员查询自己租户下的组织树
BaseQueryParam param = new BaseQueryParam();
param.setOrgId(userOrgId);           // 用户所在组织
param.setTenantOrgId(userTenantId);  // 用户租户ID
param.setFilterType(2);              // 租户隔离过滤
```

#### filterType = 1 (层级过滤)

**SQL 逻辑：**
```sql
AND id IN (
    SELECT c.id
    FROM t_sys_org f
    LEFT JOIN t_sys_org c ON (
        f.left_num <= c.left_num
        AND c.right_num <= f.right_num
        AND c.delete_flag = 0
        -- 没有 tenant_org_id 限制
    )
    WHERE f.id = #{orgId}
    AND f.delete_flag = 0
)
```

**查询意义：** 返回指定组织节点下的所有子孙节点，**不限制租户**。

**使用场景：** 超级管理员或需要跨租户查看数据的场景。

**示例：**
```java
// 超级管理员查看某个组织的所有子组织（跨租户）
BaseQueryParam param = new BaseQueryParam();
param.setOrgId(targetOrgId);
param.setFilterType(1);  // 只按层级过滤
```

#### filterType = 0 (无过滤)

**SQL 逻辑：**
```sql
-- 无额外 WHERE 子句
SELECT * FROM t_sys_org
WHERE delete_flag = 0
AND (appid = 'school-app' OR appid IS NULL)
ORDER BY node_level ASC
```

**查询意义：** 返回所有匹配 `appid` 的组织，不进行层级或租户过滤。

**使用场景：** 需要获取完整组织树时。

**示例：**
```java
// 获取某个 appid 的完整组织树
BaseQueryParam param = new BaseQueryParam();
param.setAppid("school-app");
param.setOrgId(1L);        // 设置为 1 以跳过子树过滤
param.setFilterType(0);    // 无过滤
```

### orgId = 1 的特殊含义

在 SQL 映射中，`orgId = 1` 是一个**特殊标志位**：

```xml
<if test="_base.orgId != 1 and ...">
```

- 当 `orgId = 1` 时，无论 `filterType` 是什么，**都不会触发子树过滤**
- 这是**系统级查询**的标志，表示需要获取全局数据
- 通常用于根节点查询或完整组织树导出

### Nested Set 模型

本模块使用 Nested Set（嵌套集）模型来存储和查询组织树结构：

| 字段 | 说明 |
|------|------|
| `left_num` | 节点的左边界值 |
| `right_num` | 节点的右边界值 |
| `node_level` | 节点层级（根节点为 0） |

**判断父子关系的规则：**
- 父节点: `f.left_num <= c.left_num AND c.right_num <= f.right_num`
- 子节点在父节点的边界范围内

## API 端点

### 获取组织树

```
GET /api/adm/org/tree?appid={appid}
```

**参数：**
- `appid`（可选）: 应用 ID，用于数据隔离
- `search`（可选）: 搜索关键词，支持组织名称模糊匹配

**响应：**
```json
{
  "code": 200,
  "message": "Success",
  "data": {
    "children": [
      {
        "id": 1,
        "name": "华东科技大学",
        "nodeLevel": 0,
        "children": [...]
      }
    ]
  }
}
```

## 数据库表

### t_sys_org

| 字段 | 类型 | 说明 |
|------|------|------|
| id | BIGINT | 主键 |
| pid | BIGINT | 父节点 ID（NULL 表示根节点） |
| name | VARCHAR(100) | 组织名称 |
| full_name | VARCHAR(255) | 组织全路径名称 |
| appid | VARCHAR(50) | 应用 ID |
| org_type | INT | 组织类型 |
| node_level | INT | 节点层级 |
| left_num | INT | Nested Set 左边界 |
| right_num | INT | Nested Set 右边界 |
| tenant_org_id | BIGINT | 租户组织 ID |
| delete_flag | INT | 删除标记（0=正常） |

## 使用示例

### 查询完整组织树

```java
BaseQueryParam param = new BaseQueryParam();
param.setAppid("school-app");
param.setOrgId(1L);        // 跳过子树过滤
param.setFilterType(0);    // 无过滤
List<SysOrg> orgs = sysOrgService.listLikeNameAndOrgId(param, null);
```

### 查询租户管理员可见的组织

```java
BaseQueryParam param = BaseQueryParam.create();  // 默认 filterType=2
param.setAppid("school-app");
// orgId 和 tenantOrgId 会从 JWT 自动获取
List<SysOrg> orgs = sysOrgService.listLikeNameAndOrgId(param, null);
```

### 查询指定组织的子组织（跨租户）

```java
BaseQueryParam param = new BaseQueryParam();
param.setOrgId(parentOrgId);
param.setFilterType(1);  // 只按层级过滤
List<SysOrg> orgs = sysOrgService.listLikeNameAndOrgId(param, null);
```

## 配置说明

### JWT 集成

`BaseQueryParam.create()` 会自动从 JWT 中提取以下信息：

```java
tenantOrgId = JWTKit.getOrgId();       // 租户组织 ID
orgId = JWTKit.getTenantOrgId();       // 用户所属组织 ID
appid = JWTKit.getAppid();             // 应用 ID
```

### 默认行为

```java
BaseQueryParam.create()
// 等价于
BaseQueryParam.create(null, null, null)
// 最终：
// - orgId = JWTKit.getTenantOrgId()
// - filterType = 2 (租户隔离)
```

## 相关文件

- `BaseQueryParam.java` - 查询参数定义
- `SysOrgMapper.xml` - MyBatis SQL 映射
- `OrgEndpoint.java` - REST API 端点
- `TreeUtls.java` - 树形结构构建工具
