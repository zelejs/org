# Org-Tree Skill for Claude Code

Organization tree management for `t_sys_org` table using the **Nested Set Model** with `left_num`/`right_num`.

## Overview

The `t_sys_org` table uses a **hybrid tree structure approach**:

1. **Adjacency List Model** - For parent-child relationships
   - `pid`: Parent organization id (NULL = root)
   - `node_level`: Depth level (0, 1, 2, ...)

2. **Nested Set Model** - For query optimization
   - `left_num`: Left boundary (pre-order traversal)
   - `right_num`: Right boundary (pre-order traversal)

## Features

- **Table Structure Analysis**: Describe `t_sys_org` table structure and organization logic
- **Export to Tree JSON**: Convert table data to hierarchical tree structure
- **Import from Tree JSON**: Generate SQL INSERT statements from tree JSON
- **Nested Set Model**: Support for `left_num`/`right_num` tree traversal optimization
- **Validation**: Validate tree structure integrity

## Nested Set Model Logic

### Visual Representation

```
                    [Root Org: left=1, right=14]
                            /          \
              [IT Dept: 2-7]           [HR Dept: 8-13]
                 /      \                    /
        [Dev: 3-4]  [QA: 5-6]       [Recruit: 9-10]
                                            \
                                         [B1: 11-12]
```

### Key Properties

| Property | Formula/Rule |
|----------|-------------|
| Parent-Child | `parent.left < child.left < child.right < parent.right` |
| Leaf Node | `right_num = left_num + 1` |
| Descendant Range | All nodes within `(left_num, right_num)` |
| Count Descendants | `(right_num - left_num - 1) / 2` |
| Subtree Width | `right_num - left_num + 1` |

### Query Operations

```sql
-- Get all descendants of org_id
SELECT * FROM t_sys_org
WHERE left_num > (SELECT left_num FROM t_sys_org WHERE id = :org_id)
  AND right_num < (SELECT right_num FROM t_sys_org WHERE id = :org_id);

-- Get all ancestors of org_id
SELECT * FROM t_sys_org parent, t_sys_org child
WHERE parent.left_num < child.left_num
  AND parent.right_num > child.right_num
  AND child.id = :org_id;

-- Check if leaf node
SELECT CASE WHEN right_num = left_num + 1 THEN 'leaf' ELSE 'branch' END
FROM t_sys_org WHERE id = :org_id;
```

## Table Structure: t_sys_org

### Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `id` | bigint | Primary key, AUTO_INCREMENT |
| `pid` | bigint | Parent organization id (NULL = root) |
| `name` | varchar(255) | Organization name (short form) |
| `full_name` | varchar(255) | Full organization name |
| `org_code` | varchar(255) | Organization code (unique) |
| `appid` | varchar(50) | Application ID, represents top-level organization (null = default) |
| `org_type` | int | Organization type (0=default, 1=tenant, 2=platform) |
| `node_level` | int | Tree depth level (0, 1, 2, ...) |
| **`left_num`** | int | **Nested set left boundary** |
| **`right_num`** | int | **Nested set right boundary** |
| `note` | varchar(500) | Organization description |
| `status` | varchar(50) | Status (default: 'NORMAL') |
| `create_time` | datetime | Creation timestamp |
| `update_time` | datetime | Update timestamp |
| `b_type` | varchar(50) | Business type |
| `icon` | varchar(255) | Icon URL |
| `is_visible` | tinyint(1) | Visibility flag (default: 1) |
| `need_validate` | tinyint(1) | Require validation (default: 0) |
| `delete_flag` | tinyint(1) | Soft delete flag (0=normal, 1=deleted) |
| `tenant_id` | bigint | Parent tenant organization id |
| `tenant_org_id` | bigint | Tenant org as top-level organization |

### Constraints

```sql
PRIMARY KEY (`id`)
KEY `idx_pid` (`pid`)
KEY `idx_tenant_id` (`tenant_id`)
KEY `idx_tenant_org_id` (`tenant_org_id`)
KEY `idx_org_code` (`org_code`)
```

### Table Options

```sql
ENGINE=InnoDB
AUTO_INCREMENT=1015
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_unicode_ci
COMMENT='系统组织表'
```

## Create Node Operation

Implementation in `SysOrgServiceImpl.java:92-114`:

```java
// 1. Find parent organization
SysOrg parentOrg = getVisibleOrg(opsOrgId, entity.getPid());

// 2. Update existing left_num values (shift right by 2)
UPDATE t_sys_org
SET left_num = left_num + 2
WHERE left_num >= parentOrg.right_num;

// 3. Update existing right_num values (shift right by 2)
UPDATE t_sys_org
SET right_num = right_num + 2
WHERE right_num >= parentOrg.right_num;

// 4. Insert new node
entity.setLeftNum(parentOrg.getRightNum());
entity.setRightNum(parentOrg.getRightNum() + 1);
entity.setNodeLevel(parentOrg.getNodeLevel() + 1);
```

## Delete Node Operation

Implementation in `SysOrgServiceImpl.java:124-159`:

```java
// 1. Verify no children exist
// 2. Calculate subtree width
int width = sysOrg.getRightNum() - sysOrg.getLeftNum() + 1;

// 3. Update left_num (shift left)
UPDATE t_sys_org
SET left_num = left_num - width
WHERE left_num > deleted_left_num;

// 4. Update right_num (shift left)
UPDATE t_sys_org
SET right_num = right_num - width
WHERE right_num > deleted_right_num;

// 5. Soft delete the node
sysOrg.setDeleteFlag(true);
```

## Example Data

```sql
-- Root organization (Platform)
INSERT INTO t_sys_org (id, pid, org_code, name, node_level, left_num, right_num, org_type)
VALUES (1, NULL, 'ROOT', 'Platform', 0, 1, 14, 2);

-- First level children (Tenants)
INSERT INTO t_sys_org (id, pid, org_code, name, node_level, left_num, right_num, org_type)
VALUES
  (2, 1, 'TENANT_A', 'Tenant A', 1, 2, 7, 1),
  (3, 1, 'TENANT_B', 'Tenant B', 1, 8, 13, 1);

-- Second level (Departments under Tenant A)
INSERT INTO t_sys_org (id, pid, org_code, name, node_level, left_num, right_num, org_type)
VALUES
  (4, 2, 'IT', 'IT Department', 2, 3, 4, 0),
  (5, 2, 'HR', 'HR Department', 2, 5, 6, 0);
```

Resulting tree structure:
```
[Platform: 1-14]
    ├── [Tenant A: 2-7]
    │   ├── [IT Dept: 3-4]  (leaf)
    │   └── [HR Dept: 5-6]  (leaf)
    └── [Tenant B: 8-13]
        └── [Sales: 9-12]
            └── [...]
```

## Usage

### 1. Describe Table Structure

```bash
/org-tree describe_org_table
```

### 2. Export to Tree JSON

```bash
/org-tree export_tree_json --output-file org-tree.json
```

Output format:
```json
{
  "nested": {
    "id": 1,
    "pid": null,
    "name": "Platform",
    "left_num": 1,
    "right_num": 14,
    "children": [...]
  },
  "flat": [
    {"id": 1, "pid": null, "left_num": 1, "right_num": 14},
    {"id": 2, "pid": 1, "left_num": 2, "right_num": 7}
  ]
}
```

### 3. Import from Tree JSON

```bash
/org-tree import_tree_json --input-file org-tree.json --output-sql insert-org.sql
```

### 4. Audit Tree

```bash
# Audit tree structure for data consistency
node scripts/audit-tree.js audit org-tree.json

# Audit without checking nested set values
node scripts/audit-tree.js audit org-tree.json --no-nested

# Fix and recalculate tree fields
node scripts/audit-tree.js fix org-tree.json org-fixed.json

# Fix with specific output format
node scripts/audit-tree.js fix org-tree.json org-fixed.json --format=flat
```

**Audit Output:**

```json
{
  "is_valid": false,
  "error_count": 2,
  "warning_count": 1,
  "errors": [
    {
      "category": "NESTED_SET",
      "message": "Node not within parent nested set range",
      "node_id": 5,
      "details": {
        "node": "(6, 7)",
        "parent": "(2, 9)",
        "parent_id": 2
      }
    }
  ],
  "warnings": [...],
  "stats": {
    "total_nodes": 10,
    "root_nodes": 1,
    "leaf_nodes": 6,
    "max_depth": 3
  }
}
```

**Audit Checks:**

| Category | Description |
|----------|-------------|
| `REQUIRED_FIELD` | Missing required fields (id, name) |
| `PARENT_REF` | Parent node not found |
| `CYCLE` | Circular reference detected |
| `TREE_STRUCTURE` | Multiple roots or no root |
| `CONNECTIVITY` | Nodes not reachable from roots |
| `NESTED_SET` | left_num/right_num violations |
| `DUPLICATE` | Duplicate left_num or right_num values |
| `LEAF_NODE` | Leaf node with wrong nested set values |
| `NODE_LEVEL` | node_level inconsistent with parent |

**Fix Command:**

The `fix` command recalculates all tree fields based on the hierarchical structure:
- `left_num` / `right_num`: Calculated using pre-order traversal
- `node_level`: Calculated based on depth from root
- `pid`: Verified against parent-child relationships

### 5. Validate Tree

```bash
/org-tree validate_tree --input-file org-tree.json
```

## Benefits of Nested Set Model

| Operation | Adjacency List | Nested Set |
|-----------|---------------|------------|
| Get all descendants | Recursive CTE | Single WHERE clause |
| Get subtree depth | Recursive query | `node_level` field |
| Count descendants | Count recursion | `(right_num - left_num - 1) / 2` |
| Check ancestry | Recursive/path | Range check |

## Scripts

### Node.js Scripts

Located in `scripts/` directory:

- `export-tree.js` - Export table to tree JSON
- `import-tree.js` - Import tree JSON to SQL
- `audit-tree.js` - Audit and fix tree structure (NEW)
- `validate-tree.js` - Validate tree structure

### Python Scripts

- `describe.py` - Describe table structure
- `query.py` - Query data

## Configuration

### Database Configuration

**Location:** `config/application-dev.yaml`

```yaml
spring:
  datasource:
    url: jdbc:mysql://sh-cynosdbmysql-grp-mlyunquo.sqls.cdnline.cn:25133/enrollment
    username: root
    password: zb2014@8888
    driver-class-name: com.mysql.cj.jdbc.Driver
```

**Schema Reference:** `src/main/resources/sql/org-mysql-schema.sql`

### API Testing

**Tree API Endpoint:** `/api/adm/org/tree`

**Required Parameters:**
- `appid` (optional if JWT contains appid): Application identifier for filtering organizations

**Request Example:**
```bash
curl "http://localhost:8080/api/adm/org/tree?appid=school-app"
```

**Response Format:** `SysOrgTreeItemDTO` with nested children structure
```json
{
  "code": 200,
  "data": {
    "children": [
      {
        "id": 1,
        "pid": null,
        "name": "华东科技大学",
        "nodeLevel": 0,
        "orgType": 2,
        "type": 0,
        "children": [...]
      }
    ]
  }
}
```

### Mock Data Reference

**HTML Mock Data Location:** `edu-admin/smart-enrollment-admin.html` (lines 1038-1058)

**14-Node University Structure Example:**
```
华东科技大学 (🏫, 6842人)
├── 计算机学院 (💻, 1203人)
│   ├── 软件工程 (📁, 312人)
│   │   ├── 软件工程1班 (👥, 42人)
│   │   └── 软件工程2班 (👥, 41人)
│   └── 人工智能 (📁, 240人)
│       └── 人工智能1班 (👥, 39人)
├── 机械工程学院 (⚙️, 987人)
├── 经济管理学院 (📈, 1456人)
├── 外国语学院 (🌐, 834人)
├── 医学院 (🏥, 678人)
├── 艺术设计学院 (🎨, 542人)
├── 法学院 (⚖️, 421人)
└── 理学院 (🔬, 721人)
```

**Mock JSON Example:** See `/mock-org-tree.json` in project root for complete 14-node structure with:
- id (1-14)
- pid (parent relationships)
- org_code (unique identifiers)
- node_level (0-3)
- left_num/right_num (nested set values)
- appid ('school-app' for all nodes)

**Import Mock Data:**
```bash
# Generate SQL from JSON
node scripts/import-tree.js /path/to/mock-org-tree.json /path/to/insert-org.sql --mode=replace

# Import SQL using chatdb skill
/chatdb import-sql /path/to/insert-org.sql

# Or execute individual INSERT statements
python3 ~/.claude/skills/chatdb/scripts/cursor_db_cli.py execute-sql --sql "INSERT INTO ..."
```

## Dependencies

- Node.js (for tree export/import scripts)
- Python (for database queries)

## Error Handling

All functions return standardized format:

**Success:**
```json
{
  "success": true,
  "data": {...}
}
```

**Error:**
```json
{
  "success": false,
  "error": "Error message",
  "error_code": "ERROR_CODE"
}
```

## References

- Schema: `src/main/resources/sql/org-mysql-schema.sql:4-26`
- Model: `src/main/java/.../persistence/model/SysOrg.java`
- Service: `src/main/java/.../service/Impl/SysOrgServiceImpl.java`
- DAO: `src/main/java/.../dao/mapping/UaasOrgDao.xml`

## License

MIT License
