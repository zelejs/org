#!/usr/bin/env node

/**
 * Import tree JSON and generate SQL INSERT statements for t_sys_org
 * Usage: node import-tree.js <input-json> <output-sql> [options]
 */

const fs = require('fs');
const path = require('path');

class OrgTreeImporter {
  constructor() {
    this.tableName = 't_sys_org';
  }

  /**
   * Calculate nested set values (left_num, right_num) from tree
   */
  calculateNestedSet(node, counter = { value: 1 }, level = 0) {
    if (!node.left_num) node.left_num = counter.value++;
    if (!node.node_level) node.node_level = level;

    for (const child of (node.children || [])) {
      this.calculateNestedSet(child, counter, level + 1);
    }

    if (!node.right_num) node.right_num = counter.value++;
    return node;
  }

  /**
   * Flatten tree to list (DFS traversal)
   */
  flattenTree(node, result = []) {
    const { children, ...nodeData } = node;
    result.push(nodeData);

    for (const child of (children || [])) {
      this.flattenTree(child, result);
    }

    return result;
  }

  /**
   * Escape SQL value
   */
  escapeValue(value) {
    if (value === null || value === undefined) {
      return 'NULL';
    }
    if (typeof value === 'string') {
      return `'${value.replace(/'/g, "''").replace(/\\/g, '\\\\')}'`;
    }
    if (typeof value === 'boolean') {
      return value ? '1' : '0';
    }
    return String(value);
  }

  /**
   * Generate INSERT statement for a single record
   */
  generateInsert(record) {
    const fields = [
      'id', 'pid', 'org_code', 'name', 'full_name',
      'node_level', 'left_num', 'right_num', 'org_type',
      'status', 'b_type', 'note', 'icon'
    ];

    const values = fields.map(field => {
      const value = record[field];
      return this.escapeValue(value);
    });

    const fieldList = fields.join(', ');
    const valueList = values.join(', ');

    return `INSERT INTO \`${this.tableName}\` (${fieldList})\n  VALUES (${valueList});`;
  }

  /**
   * Generate DELETE statement for a record
   */
  generateDelete(id) {
    return `DELETE FROM \`${this.tableName}\` WHERE \`id\` = ${id};`;
  }

  /**
   * Import tree JSON and generate SQL
   */
  import(treeJson, options = {}) {
    const {
      mode = 'insert', // 'insert', 'replace', 'merge'
      calculateNested = true,
      tableName = 't_sys_org'
    } = options;

    this.tableName = tableName;

    // Parse input
    let flatList;
    let tree;

    if (Array.isArray(treeJson)) {
      flatList = treeJson;
      tree = this.buildTree(flatList);
    } else if (treeJson.flat) {
      flatList = treeJson.flat;
      tree = treeJson.nested;
    } else if (treeJson.nested) {
      tree = treeJson.nested;
      flatList = this.flattenTree(tree, []);
    } else {
      // Assume nested tree
      tree = treeJson;
      flatList = this.flattenTree(tree, []);
    }

    // Calculate nested set if requested
    if (calculateNested && tree) {
      const counter = { value: 1 };
      if (Array.isArray(tree)) {
        tree.forEach(t => this.calculateNestedSet(t, counter, 0));
      } else {
        this.calculateNestedSet(tree, counter, 0);
      }
      // Re-flatten to get updated values
      flatList = Array.isArray(tree)
        ? tree.flatMap(t => this.flattenTree(t, []))
        : this.flattenTree(tree, []);
    }

    // Generate SQL
    const statements = [];

    // Header
    statements.push('-- Auto-generated SQL from org-tree skill');
    statements.push(`-- Mode: ${mode}`);
    statements.push('-- Records: ' + flatList.length);
    statements.push('');
    statements.push('SET FOREIGN_KEY_CHECKS=0;');
    statements.push('');

    if (mode === 'replace') {
      // Delete existing records
      for (const record of flatList) {
        statements.push(this.generateDelete(record.id));
      }
      statements.push('');
    }

    // Insert records (sort by id to avoid FK issues)
    const sortedList = [...flatList].sort((a, b) => (a.id || 0) - (b.id || 0));

    for (const record of sortedList) {
      statements.push(this.generateInsert(record));
      statements.push('');
    }

    // Footer
    statements.push('SET FOREIGN_KEY_CHECKS=1;');

    return {
      success: true,
      data: {
        sql: statements.join('\n'),
        statements: statements.length,
        record_count: flatList.length,
        mode: mode,
        nested_set_calculated: calculateNested
      }
    };
  }

  /**
   * Build tree from flat list
   */
  buildTree(flatData) {
    const nodeMap = new Map();
    const roots = [];

    // Create all nodes
    for (const item of flatData) {
      const node = { ...item, children: [] };
      nodeMap.set(item.id, node);
    }

    // Build hierarchy
    for (const item of flatData) {
      const node = nodeMap.get(item.id);

      if (item.pid === null || item.pid === undefined) {
        roots.push(node);
      } else {
        const parent = nodeMap.get(item.pid);
        if (parent) {
          parent.children.push(node);
        } else {
          roots.push(node);
        }
      }
    }

    return roots.length === 1 ? roots[0] : roots;
  }

  /**
   * Import from file and save SQL
   */
  importFromFile(inputFile, outputFile, options = {}) {
    // Read input
    const rawData = fs.readFileSync(inputFile, 'utf8');
    const treeJson = JSON.parse(rawData);

    // Generate SQL
    const result = this.import(treeJson, options);

    // Write output
    if (outputFile) {
      fs.writeFileSync(outputFile, result.data.sql, 'utf8');
      console.log(`Generated SQL: ${outputFile}`);
    }

    return result;
  }
}

// CLI interface
if (require.main === module) {
  const args = process.argv.slice(2);

  if (args.length < 2) {
    console.log('Usage: node import-tree.js <input-json> <output-sql> [options]');
    console.log('');
    console.log('Options:');
    console.log('  --mode=<insert|replace|merge>  SQL generation mode (default: insert)');
    console.log('  --no-calculate-nested          Don\'t calculate left_num/right_num');
    console.log('  --table=<name>                 Table name (default: t_sys_org)');
    console.log('');
    console.log('Example:');
    console.log('  node import-tree.js org-tree.json insert-org.sql --mode=insert');
    process.exit(1);
  }

  const [inputFile, outputFile, ...options] = args;

  const opts = {
    mode: 'insert',
    calculateNested: true,
    tableName: 't_sys_org'
  };

  // Parse options
  for (const opt of options) {
    if (opt.startsWith('--mode=')) {
      opts.mode = opt.split('=')[1];
    } else if (opt === '--no-calculate-nested') {
      opts.calculateNested = false;
    } else if (opt.startsWith('--table=')) {
      opts.tableName = opt.split('=')[1];
    }
  }

  try {
    const importer = new OrgTreeImporter();
    const result = importer.importFromFile(inputFile, outputFile, opts);

    console.log(JSON.stringify({
      success: true,
      data: {
        output_sql: outputFile,
        statements: result.data.statements,
        record_count: result.data.record_count,
        nested_set_calculated: result.data.nested_set_calculated
      }
    }, null, 2));
  } catch (error) {
    console.error(JSON.stringify({
      success: false,
      error: error.message,
      stack: error.stack
    }, null, 2));
    process.exit(1);
  }
}

module.exports = OrgTreeImporter;
