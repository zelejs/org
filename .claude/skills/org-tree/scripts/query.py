#!/usr/bin/env python3
"""
Query t_sys_org table data using chatdb skill
Usage: python query.py [--output=<file>] [--filter=<where>] [--limit=<n>]
"""

import sys
import os
import json
import argparse

# Add chatdb scripts to path
chatdb_path = os.path.expanduser('~/.claude/skills/chatdb/scripts')
sys.path.insert(0, chatdb_path)

try:
    from database.client import DatabaseClient
    from config import SkillConfig
except ImportError as e:
    print(json.dumps({
        "success": False,
        "error": f"Failed to import chatdb modules: {e}",
        "suggestion": "Ensure chatdb skill is properly installed and configured"
    }, indent=2))
    sys.exit(1)


def query_org_table(filter_clause=None, limit=1000, order_by='id ASC'):
    """Query t_sys_org table data"""

    try:
        # Load chatdb config
        config = SkillConfig.load()
        client = DatabaseClient(config)

        # Query all data
        result = client.query_table(
            table='t_sys_org',
            filter=filter_clause,
            limit=limit,
            order_by=order_by
        )

        if not result.get('success'):
            return {
                "success": False,
                "error": "Failed to query table",
                "details": result
            }

        data = result.get('data', [])
        meta = result.get('meta', {})

        return {
            "success": True,
            "data": data,
            "meta": {
                "count": len(data),
                "total": meta.get('total', len(data)),
                "table": "t_sys_org"
            }
        }

    except Exception as e:
        return {
            "success": False,
            "error": str(e),
            "error_type": type(e).__name__
        }


def query_tree_structure(root_id=None):
    """Query and build tree structure"""

    try:
        # Load chatdb config
        config = SkillConfig.load()
        client = DatabaseClient(config)

        # Get all data
        result = client.query_table(
            table='t_sys_org',
            limit=10000,
            order_by='node_level ASC, left_num ASC'
        )

        if not result.get('success'):
            return {
                "success": False,
                "error": "Failed to query table",
                "details": result
            }

        flat_data = result.get('data', [])

        # Build tree
        node_map = {}
        roots = []

        # Create nodes
        for item in flat_data:
            node = {**item, 'children': []}
            node_map[item['id']] = node

        # Build hierarchy
        for item in flat_data:
            node = node_map[item['id']]

            if item.get('pid') is None:
                if root_id is None or item['id'] == root_id:
                    roots.append(node)
            else:
                parent = node_map.get(item['pid'])
                if parent:
                    parent['children'].append(node)
                elif root_id is None or item['id'] == root_id:
                    roots.append(node)

        tree = roots[0] if len(roots) == 1 else roots

        return {
            "success": True,
            "data": {
                "nested": tree,
                "flat": flat_data
            },
            "meta": {
                "count": len(flat_data),
                "table": "t_sys_org",
                "root_id": root_id
            }
        }

    except Exception as e:
        return {
            "success": False,
            "error": str(e),
            "error_type": type(e).__name__
        }


def main():
    parser = argparse.ArgumentParser(description='Query t_sys_org table data')
    parser.add_argument('--output', '-o', help='Output JSON file')
    parser.add_argument('--filter', '-f', help='WHERE clause filter')
    parser.add_argument('--limit', '-l', type=int, default=1000, help='Limit records')
    parser.add_argument('--tree', '-t', action='store_true', help='Export as tree structure')
    parser.add_argument('--root', '-r', type=int, help='Root organization id (for tree export)')

    args = parser.parse_args()

    if args.tree:
        result = query_tree_structure(args.root)
    else:
        result = query_org_table(args.filter, args.limit)

    # Output to file or stdout
    output = json.dumps(result, indent=2, ensure_ascii=False)

    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            f.write(output)
        print(f"Exported to {args.output}", file=sys.stderr)
    else:
        print(output)

    # Exit with error code if failed
    if not result.get('success'):
        sys.exit(1)


if __name__ == '__main__':
    main()
