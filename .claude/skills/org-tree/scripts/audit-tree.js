#!/usr/bin/env node

/**
 * Audit and fix tree structure for t_sys_org
 *
 * Features:
 * 1. Audit mode: Validate left_num, right_num, node_level, pid consistency
 * 2. Fix mode: Recalculate and update fields based on tree structure
 *
 * Usage:
 *   # Audit tree structure
 *   node audit-tree.js audit <input-json>
 *
 *   # Fix and recalculate tree fields
 *   node audit-tree.js fix <input-json> <output-json>
 *
 *   # Check without nested set values
 *   node audit-tree.js audit <input-json> --no-nested
 */

const fs = require('fs');
const path = require('path');

class TreeAuditor {
  constructor() {
    this.records = [];
    this.idMap = new Map();
    this.errors = [];
    this.warnings = [];
    this.info = [];
    this.stats = {};
  }

  /**
   * Load JSON data from file or object
   */
  load(input) {
    let data;

    if (typeof input === 'string') {
      // File path
      const rawData = fs.readFileSync(input, 'utf8');
      data = JSON.parse(rawData);
    } else {
      // Direct object
      data = input;
    }

    // Extract flat list from various formats
    if (Array.isArray(data)) {
      this.records = data;
    } else if (data.flat) {
      this.records = data.flat;
    } else if (data.data) {
      if (Array.isArray(data.data)) {
        this.records = data.data;
      } else if (data.data.flat) {
        this.records = data.data.flat;
      } else if (data.data.nested) {
        this.records = this.flattenTree(data.data.nested);
      }
    } else if (data.nested) {
      this.records = this.flattenTree(data.nested);
    } else if (data.id && data.children) {
      // Single nested node
      this.records = this.flattenTree(data);
    } else {
      throw new Error('Unknown input format. Expected array, {flat}, {nested}, or {data}');
    }

    // Build ID map
    for (const record of this.records) {
      if (record.id !== undefined) {
        this.idMap.set(record.id, record);
      }
    }

    return this;
  }

  /**
   * Flatten nested tree to flat list (preserving existing fields)
   */
  flattenTree(node, result = [], pid = null, level = 0) {
    const { children, ...nodeData } = node;

    // Set pid and level if not provided
    if (pid !== null && nodeData.pid === undefined) {
      nodeData.pid = pid;
    }
    if (nodeData.node_level === undefined) {
      nodeData.node_level = level;
    }

    result.push(nodeData);

    for (const child of (children || [])) {
      this.flattenTree(child, result, node.id, level + 1);
    }

    return result;
  }

  /**
   * Build tree from flat list using pid relationships
   */
  buildTree(flatList) {
    const nodeMap = new Map();
    const roots = [];

    // First pass: create all nodes
    for (const item of flatList) {
      nodeMap.set(item.id, { ...item, children: [] });
    }

    // Second pass: build hierarchy
    for (const item of flatList) {
      const node = nodeMap.get(item.id);

      if (!item.pid || item.pid === null) {
        roots.push(node);
      } else {
        const parent = nodeMap.get(item.pid);
        if (parent) {
          parent.children.push(node);
        } else {
          // Parent not found, treat as root
          this.warnings.push({
            category: 'ORPHAN',
            message: `Node ${item.id} has non-existent parent ${item.pid}, treating as root`,
            node_id: item.id
          });
          roots.push(node);
        }
      }
    }

    return roots;
  }

  /**
   * Calculate nested set values (left_num, right_num) using pre-order traversal
   * Also calculates node_level and verifies pid
   */
  calculateNestedSet(roots) {
    let counter = 1;

    const traverse = (node, level = 0, pid = null) => {
      // Set values
      node.left_num = counter++;
      node.node_level = level;
      if (pid !== null) {
        node.pid = pid;
      }

      // Process children
      for (const child of (node.children || [])) {
        traverse(child, level + 1, node.id);
      }

      node.right_num = counter++;

      return node;
    };

    if (Array.isArray(roots)) {
      return roots.map(root => traverse(root));
    }
    return traverse(roots);
  }

  /**
   * Fix tree: recalculate all fields based on tree structure
   */
  fix() {
    if (this.records.length === 0) {
      throw new Error('No records to fix');
    }

    // Build tree from current structure
    const roots = this.buildTree(this.records);

    // Recalculate nested set values
    const fixedTrees = this.calculateNestedSet(roots);

    // Flatten back to list
    const fixedRecords = [];
    const flatten = (node) => {
      const { children, ...nodeData } = node;
      fixedRecords.push(nodeData);
      for (const child of (children || [])) {
        flatten(child);
      }
    };

    if (Array.isArray(fixedTrees)) {
      for (const tree of fixedTrees) {
        flatten(tree);
      }
    } else {
      flatten(fixedTrees);
    }

    this.records = fixedRecords;

    // Rebuild ID map
    this.idMap.clear();
    for (const record of this.records) {
      this.idMap.set(record.id, record);
    }

    return this.records;
  }

  /**
   * Audit: validate all tree structure rules
   */
  audit(checkNestedSet = true) {
    this.errors = [];
    this.warnings = [];
    this.info = [];
    this.stats = {
      total_nodes: this.records.length,
      root_nodes: 0,
      leaf_nodes: 0,
      max_depth: 0,
      nodes_with_children: 0,
      nodes_per_level: {}
    };

    if (this.records.length === 0) {
      this.errors.push({
        category: 'DATA',
        message: 'No records to audit'
      });
      return this.getResult();
    }

    // Collect basic stats
    for (const record of this.records) {
      if (!record.pid) {
        this.stats.root_nodes++;
      }
      const level = record.node_level || 0;
      this.stats.nodes_per_level[level] = (this.stats.nodes_per_level[level] || 0) + 1;
      if (record.node_level > this.stats.max_depth) {
        this.stats.max_depth = record.node_level;
      }
    }

    // Run validation checks
    this._validateRequiredFields();
    this._validateParentReferences();
    this._validateNoCycles();
    this._validateSingleRoot();
    this._validateConnectivity();

    if (checkNestedSet) {
      this._validateNestedSetBasicRules();
      this._validateNestedSetParentChildConsistency();
      this._validateNestedSetNoDuplicates();
      this._validateLeafNodes();
    }

    this._validateNodeLevelConsistency();

    // Count leaf nodes and nodes with children
    for (const record of this.records) {
      const hasChildren = this.records.some(r => r.pid === record.id);
      if (hasChildren) {
        this.stats.nodes_with_children++;
      } else {
        this.stats.leaf_nodes++;
      }
    }

    this.info.push({
      category: 'STATS',
      message: `Audit complete: ${this.stats.total_nodes} nodes, ${this.stats.root_nodes} root(s), ${this.stats.leaf_nodes} leaves, max depth: ${this.stats.max_depth}`
    });

    return this.getResult();
  }

  /**
   * Validate required fields exist
   */
  _validateRequiredFields() {
    const requiredFields = ['id', 'name'];
    const recommendedFields = ['pid', 'node_level'];

    for (const record of this.records) {
      for (const field of requiredFields) {
        if (record[field] === undefined || record[field] === null) {
          this.errors.push({
            category: 'REQUIRED_FIELD',
            message: `Missing required field: ${field}`,
            node_id: record.id
          });
        }
      }
    }

    // Check for recommended fields
    for (const field of recommendedFields) {
      const missing = this.records.filter(r => r[field] === undefined);
      if (missing.length > 0) {
        this.warnings.push({
          category: 'RECOMMENDED_FIELD',
          message: `Field '${field}' missing in ${missing.length} nodes`,
          details: { sample_ids: missing.slice(0, 5).map(r => r.id) }
        });
      }
    }
  }

  /**
   * Validate parent references exist
   */
  _validateParentReferences() {
    for (const record of this.records) {
      if (record.pid !== null && record.pid !== undefined) {
        const parent = this.idMap.get(record.pid);
        if (!parent) {
          this.errors.push({
            category: 'PARENT_REF',
            message: `Parent node ${record.pid} not found`,
            node_id: record.id,
            details: { pid: record.pid }
          });
        }
      }
    }
  }

  /**
   * Validate no cycles in parent references
   */
  _validateNoCycles() {
    const visited = new Set();
    const recursionStack = new Set();

    const dfs = (id) => {
      if (recursionStack.has(id)) {
        this.errors.push({
          category: 'CYCLE',
          message: `Cycle detected in tree involving node ${id}`,
          node_id: id
        });
        return true;
      }
      if (visited.has(id)) {
        return false;
      }

      visited.add(id);
      recursionStack.add(id);

      const record = this.idMap.get(id);
      if (record && record.pid !== null && record.pid !== undefined) {
        const parent = this.idMap.get(record.pid);
        if (parent && dfs(record.pid)) {
          return true;
        }
      }

      recursionStack.delete(id);
      return false;
    };

    for (const record of this.records) {
      if (!visited.has(record.id)) {
        dfs(record.id);
      }
    }
  }

  /**
   * Validate single root (or warn if multiple)
   */
  _validateSingleRoot() {
    const roots = this.records.filter(r => !r.pid || r.pid === null);

    if (roots.length === 0) {
      this.errors.push({
        category: 'TREE_STRUCTURE',
        message: 'No root node found (node with pid=null)'
      });
    } else if (roots.length > 1) {
      this.warnings.push({
        category: 'TREE_STRUCTURE',
        message: `Multiple root nodes found: ${roots.length}`,
        details: { root_ids: roots.map(r => r.id) }
      });
    }
  }

  /**
   * Validate all nodes are connected (reachable from roots)
   */
  _validateConnectivity() {
    const roots = this.records.filter(r => !r.pid || r.pid === null);
    if (roots.length === 0) return;

    const visited = new Set();
    const queue = [...roots.map(r => r.id)];

    while (queue.length > 0) {
      const id = queue.shift();
      if (visited.has(id)) continue;
      visited.add(id);

      // Find children
      for (const record of this.records) {
        if (record.pid === id && !visited.has(record.id)) {
          queue.push(record.id);
        }
      }
    }

    const unvisited = this.records.filter(r => !visited.has(r.id));
    if (unvisited.length > 0) {
      this.errors.push({
        category: 'CONNECTIVITY',
        message: `${unvisited.length} nodes not reachable from roots (disconnected tree or cycle)`,
        details: { unvisited_ids: unvisited.map(r => r.id) }
      });
    }
  }

  /**
   * Validate basic nested set rules
   */
  _validateNestedSetBasicRules() {
    let hasNestedSet = false;

    for (const record of this.records) {
      if (record.left_num !== undefined && record.right_num !== undefined) {
        hasNestedSet = true;

        if (record.left_num >= record.right_num) {
          this.errors.push({
            category: 'NESTED_SET',
            message: `left_num (${record.left_num}) must be less than right_num (${record.right_num})`,
            node_id: record.id,
            details: { left_num: record.left_num, right_num: record.right_num }
          });
        }
      }
    }

    if (!hasNestedSet) {
      this.warnings.push({
        category: 'NESTED_SET',
        message: 'No nodes have nested set values (left_num, right_num)'
      });
    }
  }

  /**
   * Validate parent-child nested set consistency
   */
  _validateNestedSetParentChildConsistency() {
    for (const record of this.records) {
      const { left_num, right_num, pid } = record;

      if (left_num === undefined || right_num === undefined) continue;
      if (!pid) continue;

      const parent = this.idMap.get(pid);
      if (!parent) continue;

      const parentLeft = parent.left_num;
      const parentRight = parent.right_num;

      if (parentLeft === undefined || parentRight === undefined) continue;

      // Child must be within parent's range
      if (!(parentLeft < left_num && right_num < parentRight)) {
        this.errors.push({
          category: 'NESTED_SET',
          message: 'Node not within parent nested set range',
          node_id: record.id,
          details: {
            node: `(${left_num}, ${right_num})`,
            parent: `(${parentLeft}, ${parentRight})`,
            parent_id: pid
          }
        });
      }
    }
  }

  /**
   * Validate no duplicate left_num or right_num values
   */
  _validateNestedSetNoDuplicates() {
    const leftMap = new Map();
    const rightMap = new Map();

    for (const record of this.records) {
      if (record.left_num !== undefined) {
        if (!leftMap.has(record.left_num)) {
          leftMap.set(record.left_num, []);
        }
        leftMap.get(record.left_num).push(record.id);
      }

      if (record.right_num !== undefined) {
        if (!rightMap.has(record.right_num)) {
          rightMap.set(record.right_num, []);
        }
        rightMap.get(record.right_num).push(record.id);
      }
    }

    // Check duplicates
    for (const [value, ids] of leftMap) {
      if (ids.length > 1) {
        this.errors.push({
          category: 'DUPLICATE',
          message: `Duplicate left_num value ${value} in ${ids.length} nodes`,
          details: { node_ids: ids }
        });
      }
    }

    for (const [value, ids] of rightMap) {
      if (ids.length > 1) {
        this.errors.push({
          category: 'DUPLICATE',
          message: `Duplicate right_num value ${value} in ${ids.length} nodes`,
          details: { node_ids: ids }
        });
      }
    }

    // Check for left_num = another node's right_num
    for (const record of this.records) {
      if (record.left_num !== undefined && rightMap.has(record.left_num)) {
        const conflictingIds = rightMap.get(record.left_num).filter(id => id !== record.id);
        if (conflictingIds.length > 0) {
          this.errors.push({
            category: 'NESTED_SET',
            message: `Node's left_num equals another node's right_num`,
            node_id: record.id,
            details: {
              left_num: record.left_num,
              conflicting_node_ids: conflictingIds
            }
          });
        }
      }
    }
  }

  /**
   * Validate leaf nodes have correct nested set values
   */
  _validateLeafNodes() {
    for (const record of this.records) {
      const hasChildren = this.records.some(r => r.pid === record.id);

      if (!hasChildren) {
        // Leaf node should have right_num = left_num + 1
        if (record.left_num !== undefined && record.right_num !== undefined) {
          if (record.right_num !== record.left_num + 1) {
            this.errors.push({
              category: 'LEAF_NODE',
              message: `Leaf node should have right_num = left_num + 1`,
              node_id: record.id,
              details: {
                left_num: record.left_num,
                right_num: record.right_num,
                expected_right: record.left_num + 1
              }
            });
          }
        }
      }
    }
  }

  /**
   * Validate node_level consistency
   */
  _validateNodeLevelConsistency() {
    // BFS from roots to check levels
    const roots = this.records.filter(r => !r.pid || r.pid === null);
    if (roots.length === 0) return;

    for (const root of roots) {
      if (root.node_level !== undefined && root.node_level !== 0 && root.node_level !== 1) {
        this.warnings.push({
          category: 'NODE_LEVEL',
          message: `Root node has unexpected level: ${root.node_level}`,
          node_id: root.id,
          details: { node_level: root.node_level, expected: 0 }
        });
      }
    }

    // Check each node's level against parent
    for (const record of this.records) {
      if (record.node_level === undefined) continue;
      if (!record.pid) continue;

      const parent = this.idMap.get(record.pid);
      if (!parent || parent.node_level === undefined) continue;

      const expectedLevel = parent.node_level + 1;
      if (record.node_level !== expectedLevel) {
        this.errors.push({
          category: 'NODE_LEVEL',
          message: `node_level inconsistent with parent`,
          node_id: record.id,
          details: {
            actual: record.node_level,
            expected: expectedLevel,
            parent_id: record.pid,
            parent_level: parent.node_level
          }
        });
      }
    }
  }

  /**
   * Get audit result
   */
  getResult() {
    return {
      is_valid: this.errors.length === 0,
      error_count: this.errors.length,
      warning_count: this.warnings.length,
      info_count: this.info.length,
      errors: this.errors,
      warnings: this.warnings,
      info: this.info,
      stats: this.stats
    };
  }

  /**
   * Export records to various formats
   */
  export(format = 'both') {
    // Build tree
    const roots = this.buildTree(this.records);

    const output = {};

    if (format === 'nested' || format === 'both') {
      output.nested = roots.length === 1 ? roots[0] : roots;
    }

    if (format === 'flat' || format === 'both') {
      output.flat = this.records;
    }

    return {
      success: true,
      data: output,
      meta: {
        record_count: this.records.length,
        format: format
      }
    };
  }

  /**
   * Save to file
   */
  save(outputFile, format = 'both') {
    const result = this.export(format);
    fs.writeFileSync(outputFile, JSON.stringify(result.data, null, 2), 'utf8');
    return result;
  }
}

// CLI interface
if (require.main === module) {
  const args = process.argv.slice(2);

  if (args.length < 2) {
    console.log('Usage:');
    console.log('  node audit-tree.js audit <input-json> [options]');
    console.log('  node audit-tree.js fix <input-json> <output-json> [options]');
    console.log('');
    console.log('Commands:');
    console.log('  audit  - Validate tree structure');
    console.log('  fix    - Recalculate and update tree fields');
    console.log('');
    console.log('Options:');
    console.log('  --no-nested    Skip nested set validation (left_num, right_num)');
    console.log('  --format=<nested|flat|both>  Output format (default: both)');
    console.log('');
    console.log('Examples:');
    console.log('  node audit-tree.js audit org-tree.json');
    console.log('  node audit-tree.js audit org-tree.json --no-nested');
    console.log('  node audit-tree.js fix org-tree.json org-fixed.json');
    console.log('  node audit-tree.js fix org-tree.json org-fixed.json --format=flat');
    process.exit(1);
  }

  const [command, inputFile, ...rest] = args;

  try {
    const auditor = new TreeAuditor();
    auditor.load(inputFile);

    let options = { format: 'both', checkNestedSet: true };

    // Parse options
    for (let i = 0; i < rest.length; i++) {
      const arg = rest[i];
      if (arg === '--no-nested') {
        options.checkNestedSet = false;
      } else if (arg.startsWith('--format=')) {
        options.format = arg.split('=')[1];
      }
    }

    if (command === 'audit') {
      const result = auditor.audit(options.checkNestedSet);
      console.log(JSON.stringify(result, null, 2));
      process.exit(result.is_valid ? 0 : 1);

    } else if (command === 'fix') {
      const outputFile = rest.find(a => !a.startsWith('--'));
      if (!outputFile) {
        console.error('Error: Output file required for fix command');
        process.exit(1);
      }

      auditor.fix();

      // Verify the fix
      const auditResult = auditor.audit(true);

      if (auditResult.is_valid) {
        auditor.save(outputFile, options.format);
        console.log(JSON.stringify({
          success: true,
          message: 'Tree fixed successfully',
          output: outputFile,
          stats: auditResult.stats
        }, null, 2));
        process.exit(0);
      } else {
        console.log(JSON.stringify({
          success: false,
          message: 'Fix completed but validation still has errors',
          audit: auditResult
        }, null, 2));
        process.exit(1);
      }

    } else {
      console.error(`Error: Unknown command '${command}'. Use 'audit' or 'fix'`);
      process.exit(1);
    }

  } catch (error) {
    console.error(JSON.stringify({
      success: false,
      error: error.message,
      stack: error.stack
    }, null, 2));
    process.exit(1);
  }
}

module.exports = TreeAuditor;
