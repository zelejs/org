# bType 字段说明文档

## 概述

`bType`（数据库字段名：`b_type`）是组织表 `t_sys_org` 中的核心字段，用于标识组织的**业务类型**，实现系统级组织与用户级组织的逻辑隔离。

---

## 字段定义

| 属性 | 值 |
|-----|-----|
| **数据库字段名** | `b_type` |
| **Java 属性名** | `bType` |
| **数据类型** | `varchar(30)` |
| **默认值** | `'SYSTEM'` |
| **可为空** | 否 |
| **所属表** | `t_sys_org` |

### 数据库定义

```sql
`b_type` varchar(30) default 'SYSTEM' COMMENT '默认为系统平台为 SYSTEM    用户组织为 USER '
```

### 实体类定义

```java
@TableField("b_type")
private String bType;

public String getbType() {
    return bType;
}

public void setbType(String bType) {
    this.bType = bType;
}
```

---

## 取值范围

| 常量值 | 含义 | 说明 |
|--------|-----|-----|
| `SYSTEM` | 系统组织 | 平台级组织，由系统初始化创建 |
| `USER` | 用户组织 | 租户/用户自行创建的组织 |

### 常量定义

位置：`src/main/java/com/jfeat/org/services/domain/model/SysOrgBType.java`

```java
/**
 * 组织业务类型
 * @SYSTEM  系统组织
 * @USER  用户组织
 */
public class SysOrgBType {
    public static final String SYSTEM = "SYSTEM";
    public static final String USER = "USER";
}
```

---

## 业务逻辑

### 1. 继承机制

创建子组织时，`bType` 从父组织继承，保持组织树中类型的一致性。

```java
// src/main/java/com/jfeat/org/services/domain/service/Impl/SysOrgServiceImpl.java:109
entity.setbType(parentOrg.getbType());
```

**规则**：
- 如果父组织是 `SYSTEM` 类型，子组织也是 `SYSTEM` 类型
- 如果父组织是 `USER` 类型，子组织也是 `USER` 类型

### 2. 查询过滤

在组织搜索和查询时，使用 `bType` 作为过滤条件，实现不同类型组织的隔离查询。

```java
// src/main/java/com/jfeat/org/services/domain/service/Impl/SysOrgServiceImpl.java:289-292
if(Platform.ID.getValue().equals(orgId)){
    return uaasOrgDao.searchAllOrgRecord(search, "SYSTEM");
} else {
    return uaasOrgDao.searchDescendantOrgRecord(orgId, search, "SYSTEM");
}
```

对应的 SQL 查询：

```xml
<!-- src/main/java/com/jfeat/org/services/domain/dao/mapping/UaasOrgDao.xml -->
select * from (...) t_sys_org
where t_sys_org.b_type = #{bType}
```

### 3. DAO 接口

```java
// src/main/java/com/jfeat/org/services/domain/dao/UaasOrgDao.java
List<SysOrgRecord> searchAllOrgRecord(@Param("search") String search, @Param("bType") String bType);
List<SysOrgRecord> searchDescendantOrgRecord(@Param("orgId") Long orgId, @Param("search") String search, @Param("bType") String bType);
```

---

## 设计目的

### 多租户隔离

`bType` 字段是多租户架构中组织分层的关键机制：

```
┌─────────────────────────────────────────┐
│           系统 (SYSTEM)                  │
│  ┌──────────┐      ┌──────────┐         │
│  │ 平台组织  │      │ 系统部门  │         │
│  └──────────┘      └──────────┘         │
└─────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────┐
│          用户 (USER)                     │
│  ┌──────────┐      ┌──────────┐         │
│  │ 租户A组织 │      │ 租户B组织  │         │
│  └──────────┘      └──────────┘         │
└─────────────────────────────────────────┘
```

### 典型应用场景

| 场景 | bType | 说明 |
|-----|-------|-----|
| 系统初始化 | `SYSTEM` | 创建平台级根组织 |
| 平台管理 | `SYSTEM` | 系统管理员可见和操作 |
| 租户注册 | `USER` | 新租户创建自己的组织 |
| 组织管理 | `USER` | 租户管理员管理自己的组织树 |

---

## 相关文件

| 文件 | 说明 |
|-----|-----|
| `src/main/resources/sql/org-mysql-schema.sql:18` | 数据库表结构定义 |
| `src/main/java/com/jfeat/org/services/persistence/model/SysOrg.java:82-83` | 实体类字段定义 |
| `src/main/java/com/jfeat/org/services/domain/model/SysOrgBType.java` | 常量定义类 |
| `src/main/java/com/jfeat/org/services/domain/dao/UaasOrgDao.java:40-42` | DAO 查询接口 |
| `src/main/java/com/jfeat/org/services/domain/dao/mapping/UaasOrgDao.xml:163,184` | MyBatis SQL 映射 |
| `src/main/java/com/jfeat/org/services/domain/service/Impl/SysOrgServiceImpl.java:109` | 业务逻辑使用 |

---

## 注意事项

1. **不可随意修改**：`bType` 在组织创建时确定，建议不要后续修改
2. **一致性保证**：同一组织树内的所有节点应保持相同的 `bType`
3. **查询必选条件**：在查询组织时，应始终指定 `bType` 条件以避免数据混淆
4. **权限控制**：`SYSTEM` 类型组织通常需要更高权限才能操作
