#!/usr/bin/env node

/**
 * Export t_sys_org table data to hierarchical tree JSON
 * Usage: node export-tree.js <input-json> <output-json> [options]
 */

const fs = require('fs');
const path = require('path');

class OrgTreeExporter {
  constructor() {
    this.flatList = [];
    this.idMap = new Map();
  }

  /**
   * Build hierarchical tree from flat list
   */
  buildTree(flatData, rootId = null) {
    // Create id to node map
    const nodeMap = new Map();
    const roots = [];

    // First pass: create all nodes and initialize children
    for (const item of flatData) {
      const node = { ...item, children: [] };
      nodeMap.set(item.id, node);
    }

    // Second pass: build hierarchy
    for (const item of flatData) {
      const node = nodeMap.get(item.id);

      if (item.pid === null || item.pid === undefined) {
        // Root node
        if (rootId === null || item.id === rootId) {
          roots.push(node);
        }
      } else {
        // Child node - add to parent's children
        const parent = nodeMap.get(item.pid);
        if (parent) {
          parent.children.push(node);
        } else {
          // Parent not found, treat as root
          if (rootId === null || item.id === rootId) {
            roots.push(node);
          }
        }
      }
    }

    return roots.length === 1 ? roots[0] : roots;
  }

  /**
   * Calculate nested set values (left_num, right_num) from tree
   */
  calculateNestedSet(node, counter = { value: 1 }, level = 0) {
    node.left_num = counter.value++;
    node.node_level = level;

    for (const child of (node.children || [])) {
      this.calculateNestedSet(child, counter, level + 1);
    }

    node.right_num = counter.value++;
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
   * Export from database records to JSON
   */
  export(records, options = {}) {
    const {
      format = 'both', // 'nested', 'flat', 'both'
      rootId = null,
      calculateNested = false
    } = options;

    this.flatList = records;

    // Build tree
    let tree = this.buildTree(records, rootId);

    // Calculate nested set if requested
    if (calculateNested && tree) {
      if (Array.isArray(tree)) {
        tree = tree.map(t => this.calculateNestedSet(t));
      } else {
        tree = this.calculateNestedSet(tree);
      }
    }

    // Prepare output
    const output = {};

    if (format === 'nested' || format === 'both') {
      output.nested = tree;
    }

    if (format === 'flat' || format === 'both') {
      output.flat = this.flattenTree(Array.isArray(tree) ? tree[0] : tree, []);
    }

    return {
      success: true,
      data: output,
      meta: {
        record_count: records.length,
        format: format,
        nested_set_calculated: calculateNested
      }
    };
  }

  /**
   * Load and export from JSON file
   */
  exportFromFile(inputFile, outputFile, options = {}) {
    // Read input file
    const rawData = fs.readFileSync(inputFile, 'utf8');
    const records = JSON.parse(rawData);

    // Handle different input formats
    let flatRecords;
    if (Array.isArray(records)) {
      flatRecords = records;
    } else if (records.data) {
      flatRecords = Array.isArray(records.data) ? records.data : records.data.flat || records.data.nested;
    } else if (records.flat) {
      flatRecords = records.flat;
    } else if (records.nested) {
      // Need to flatten
      flatRecords = this.flattenTree(records.nested, []);
    } else {
      throw new Error('Unknown input format');
    }

    // Export
    const result = this.export(flatRecords, options);

    // Write output
    if (outputFile) {
      fs.writeFileSync(outputFile, JSON.stringify(result.data, null, 2), 'utf8');
      console.log(`Exported to ${outputFile}`);
    }

    return result;
  }
}

// CLI interface
if (require.main === module) {
  const args = process.argv.slice(2);

  if (args.length < 2) {
    console.log('Usage: node export-tree.js <input-json> <output-json> [options]');
    console.log('');
    console.log('Options:');
    console.log('  --format=<nested|flat|both>  Output format (default: both)');
    console.log('  --root=<id>                  Root organization id');
    console.log('  --calculate-nested           Calculate left_num/right_num');
    console.log('');
    console.log('Example:');
    console.log('  node export-tree.js db-records.json org-tree.json --format=both --calculate-nested');
    process.exit(1);
  }

  const [inputFile, outputFile, ...options] = args;

  const opts = {
    format: 'both',
    rootId: null,
    calculateNested: false
  };

  // Parse options
  for (const opt of options) {
    if (opt.startsWith('--format=')) {
      opts.format = opt.split('=')[1];
    } else if (opt.startsWith('--root=')) {
      opts.rootId = parseInt(opt.split('=')[1]);
    } else if (opt === '--calculate-nested') {
      opts.calculateNested = true;
    }
  }

  try {
    const exporter = new OrgTreeExporter();
    const result = exporter.exportFromFile(inputFile, outputFile, opts);

    console.log(JSON.stringify(result, null, 2));
  } catch (error) {
    console.error(JSON.stringify({
      success: false,
      error: error.message
    }, null, 2));
    process.exit(1);
  }
}

module.exports = OrgTreeExporter;
