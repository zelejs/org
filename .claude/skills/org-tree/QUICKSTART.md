# Org-Tree Skill - Quick Start

## Installation

The skill uses the `chatdb` skill for database operations. Ensure chatdb is installed and configured.

```bash
# Check chatdb is installed
ls ~/.claude/skills/chatdb

# Configure chatdb database connection
cp ~/.claude/skills/chatdb/config.yaml.example ~/.claude/skills/chatdb/scripts/config.yaml
# Edit config.yaml with your database credentials
```

## Quick Start

### 1. Describe Table Structure

```bash
# Via skill
/org-tree describe_org_table

# Via Python script
cd ~/.claude/skills/org-tree/scripts
python3 describe.py
```

### 2. Export Data to Tree JSON

```bash
# Query data from database
cd ~/.claude/skills/org-tree/scripts
python3 query.py --tree --output org-data.json

# Or export with Node.js (if you have flat data)
node export-tree.js flat-data.json org-tree.json --format=both --calculate-nested
```

### 3. Validate Tree

```bash
cd ~/.claude/skills/org-tree/scripts
node validate-tree.js org-data.json
```

### 4. Import Tree JSON to SQL

```bash
cd ~/.claude/skills/org-tree/scripts
node import-tree.js org-tree.json insert-org.sql --mode=insert
```

### 5. Execute SQL (via chatdb)

```bash
# Using chatdb skill
/chatdb import_sql --sql-file=insert-org.sql
```

## Complete Workflow Example

```bash
# 1. Export from database
cd ~/.claude/skills/org-tree/scripts
python3 query.py --tree --output my-org.json

# 2. Validate exported data
node validate-tree.js my-org.json

# 3. Generate SQL (if needed)
node import-tree.js my-org.json import.sql --mode=insert

# 4. Import to another database (via chatdb skill)
/chatdb import_sql --sql-file=import.sql
```

## File Format

### Input (Flat List from Database)
```json
[
  {"id": 1, "pid": null, "name": "HQ", "org_code": "HQ", "node_level": 0, "left_num": 1, "right_num": 4},
  {"id": 2, "pid": 1, "name": "IT", "org_code": "IT", "node_level": 1, "left_num": 2, "right_num": 3},
  {"id": 3, "pid": 1, "name": "HR", "org_code": "HR", "node_level": 1, "left_num": 4, "right_num": 5}
]
```

### Output (Tree Format)
```json
{
  "nested": {
    "id": 1,
    "pid": null,
    "name": "HQ",
    "children": [
      {"id": 2, "pid": 1, "name": "IT", "children": []},
      {"id": 3, "pid": 1, "name": "HR", "children": []}
    ]
  },
  "flat": [...]
}
```

## Troubleshooting

### ChatDB not found
```
Error: Failed to import chatdb modules
```
Solution: Install and configure chatdb skill first.

### Database connection error
```
Error: Access denied for user
```
Solution: Check chatdb config.yaml credentials.

### Validation errors
```
Error: Record has invalid parent reference
```
Solution: Check that all pid values reference existing ids.

## Next Steps

- Read README.md for detailed documentation
- Review t_sys_org table schema in your database
- Check existing tree data using query.py
