# Changelog

All notable changes to the org-tree skill will be documented in this file.

## [1.1.0] - 2026-03-12

### Added
- `audit_tree` function - Comprehensive tree structure auditing
- `scripts/audit-tree.js` - Node.js script for auditing and fixing tree structure
- `scripts/audit_tree.py` - Python script for database tree auditing

### Enhanced
- Updated skill.yaml with detailed Nested Set Model documentation
- Enhanced README.md with comprehensive field descriptions and audit guide
- Added tree structure field descriptions for all t_sys_org columns

### Audit Features
- **Audit Mode**: Validates left_num, right_num, node_level, pid consistency
  - Required field validation (id, name)
  - Parent reference validation
  - Cycle detection
  - Single root validation
  - Tree connectivity check
  - Nested set basic rules (left < right)
  - Parent-child nested set consistency
  - Duplicate value detection
  - Leaf node validation
  - Node level consistency

- **Fix Mode**: Recalculates tree fields based on structure
  - Automatic left_num/right_num calculation via pre-order traversal
  - Automatic node_level calculation based on depth
  - Automatic pid verification and correction
  - Supports both nested and flat JSON formats

### Usage Examples
```bash
# Audit JSON tree
node scripts/audit-tree.js audit org-tree.json

# Fix and recalculate
node scripts/audit-tree.js fix org-tree.json org-fixed.json

# Audit database (Python)
python scripts/audit_tree.py --mode db --verbose
```

## [1.0.0] - 2024-03-12

### Added
- Initial release of org-tree skill
- `describe_org_table` function - Describe t_sys_org table structure
- `export_tree_json` function - Export table data to tree JSON
- `import_tree_json` function - Import tree JSON to SQL INSERT statements
- `validate_tree` function - Validate tree structure integrity

### Scripts
- `scripts/export-tree.js` - Node.js script for exporting tree JSON
- `scripts/import-tree.js` - Node.js script for importing tree JSON to SQL
- `scripts/validate-tree.js` - Node.js script for validating tree structure
- `scripts/describe.py` - Python script using chatdb to describe table
- `scripts/query.py` - Python script using chatdb to query data

### Features
- Support for Adjacency List model (pid, node_level)
- Support for Nested Set model (left_num, right_num)
- Automatic calculation of nested set values
- Tree validation with error and warning reporting
- Multiple output formats (nested, flat, both)
- Multiple import modes (insert, replace, merge)

### Documentation
- README.md with detailed usage instructions
- QUICKSTART.md for quick start guide
- skill.yaml with function definitions
