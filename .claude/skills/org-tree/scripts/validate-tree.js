#!/usr/bin/env node

/**
 * Validate tree structure integrity
 * Usage: node validate-tree.js <input-json>
 */

const fs = require('fs');

class TreeValidator {
  constructor() {
    this.errors = [];
    this.warnings = [];
    this.records = [];
    this.idMap = new Map();
  }

  /**
   * Load and parse input file
   */
  load(inputFile) {
    const rawData = fs.readFileSync(inputFile, 'utf8');
    const data = JSON.parse(rawData);

    // Extract flat list
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
        this.records = this.flattenTree(data.data.nested, []);
      }
    } else if (data.nested) {
      this.records = this.flattenTree(data.nested, []);
    }

    // Build id map
    for (const record of this.records) {
      if (record.id) {
        this.idMap.set(record.id, record);
      }
    }

    return this;
  }

  /**
   * Flatten tree to list
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
   * Validate required fields
   */
  validateFields() {
    const requiredFields = ['id', 'name'];
    const optionalFields = ['pid', 'org_code', 'org_type', 'node_level', 'left_num', 'right_num', 'status'];

    for (const record of this.records) {
      for (const field of requiredFields) {
        if (record[field] === undefined || record[field] === null) {
          this.errors.push(`Record missing required field '${field}': ${JSON.stringify(record)}`);
        }
      }
    }

    return this;
  }

  /**
   * Validate parent references
   */
  validateParentReferences() {
    const rootCount = this.records.filter(r => r.pid === null || r.pid === undefined).length;

    for (const record of this.records) {
      if (record.pid !== null && record.pid !== undefined) {
        const parent = this.idMap.get(record.pid);
        if (!parent) {
          this.errors.push(`Record ${record.id} has invalid parent reference: pid=${record.pid} not found`);
        }
      }
    }

    // Check for multiple roots
    if (rootCount > 1) {
      this.warnings.push(`Found ${rootCount} root nodes (pid=null). Expected 1 for single tree.`);
    }

    return this;
  }

  /**
   * Validate nested set values
   */
  validateNestedSet() {
    for (const record of this.records) {
      const { id, left_num, right_num, pid } = record;

      // Check if left_num and right_num exist
      if (left_num === undefined || right_num === undefined) {
        this.warnings.push(`Record ${id} missing nested set values (left_num, right_num)`);
        continue;
      }

      // Check basic constraints
      if (left_num >= right_num) {
        this.errors.push(`Record ${id} has invalid nested set: left_num (${left_num}) must be < right_num (${right_num})`);
      }

      // Check parent relationship
      if (pid !== null && pid !== undefined) {
        const parent = this.idMap.get(pid);
        if (parent && parent.left_num !== undefined && parent.right_num !== undefined) {
          if (left_num <= parent.left_num || right_num >= parent.right_num) {
            this.errors.push(`Record ${id} nested set (${left_num}, ${right_num}) not within parent ${pid} (${parent.left_num}, ${parent.right_num})`);
          }
        }
      }

      // Check for overlaps (simplified check)
      for (const other of this.records) {
        if (other.id === id) continue;
        if (other.left_num === undefined || other.right_num === undefined) continue;

        // Check if this node is a descendant of other
        if (left_num > other.left_num && right_num < other.right_num) {
          // This node is inside other's range - should be descendant
          if (record.pid !== other.id) {
            // Check if it's a descendant through chain
            let current = this.idMap.get(record.pid);
            let isDescendant = false;
            while (current) {
              if (current.id === other.id) {
                isDescendant = true;
                break;
              }
              current = this.idMap.get(current.pid);
            }
            if (!isDescendant) {
              this.warnings.push(`Record ${id} is within nested set range of ${other.id} but not a descendant`);
            }
          }
        }
      }
    }

    return this;
  }

  /**
   * Validate node levels
   */
  validateNodeLevels() {
    for (const record of this.records) {
      const { id, node_level, pid } = record;

      if (node_level === undefined) {
        this.warnings.push(`Record ${id} missing node_level`);
        continue;
      }

      if (pid === null || pid === undefined) {
        // Root should have level 0
        if (node_level !== 0) {
          this.warnings.push(`Root record ${id} has node_level=${node_level}, expected 0`);
        }
      } else {
        const parent = this.idMap.get(pid);
        if (parent && parent.node_level !== undefined) {
          if (node_level !== parent.node_level + 1) {
            this.errors.push(`Record ${id} has node_level=${node_level}, expected ${parent.node_level + 1} based on parent ${pid}`);
          }
        }
      }
    }

    return this;
  }

  /**
   * Check for cycles
   */
  validateNoCycles() {
    const visited = new Set();
    const recursionStack = new Set();

    const dfs = (id) => {
      if (recursionStack.has(id)) {
        this.errors.push(`Cycle detected involving record ${id}`);
        return true;
      }
      if (visited.has(id)) {
        return false;
      }

      visited.add(id);
      recursionStack.add(id);

      const record = this.idMap.get(id);
      if (record && record.pid !== null && record.pid !== undefined) {
        if (dfs(record.pid)) {
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

    return this;
  }

  /**
   * Run all validations
   */
  validate() {
    this.validateFields()
      .validateParentReferences()
      .validateNestedSet()
      .validateNodeLevels()
      .validateNoCycles();

    return {
      is_valid: this.errors.length === 0,
      errors: this.errors,
      warnings: this.warnings,
      record_count: this.records.length
    };
  }
}

// CLI interface
if (require.main === module) {
  const args = process.argv.slice(2);

  if (args.length < 1) {
    console.log('Usage: node validate-tree.js <input-json>');
    process.exit(1);
  }

  const [inputFile] = args;

  try {
    const validator = new TreeValidator();
    validator.load(inputFile);
    const result = validator.validate();

    console.log(JSON.stringify(result, null, 2));

    // Exit with error code if validation failed
    if (!result.is_valid) {
      process.exit(1);
    }
  } catch (error) {
    console.error(JSON.stringify({
      is_valid: false,
      errors: [error.message],
      warnings: []
    }, null, 2));
    process.exit(1);
  }
}

module.exports = TreeValidator;
