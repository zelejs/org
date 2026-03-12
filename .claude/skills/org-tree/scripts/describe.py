#!/usr/bin/env python3
"""
Describe t_sys_org table structure using direct MySQL connection
Usage: python describe.py
"""

import sys
import os
import json
import pymysql
from pathlib import Path

# Add parent directory to path for config import
sys.path.insert(0, str(Path(__file__).parent))

try:
    from config import OrgTreeConfig
except ImportError as e:
    print(json.dumps({
        "success": False,
        "error": f"Failed to import config: {e}"
    }, indent=2))
    sys.exit(1)


class OrgTableDescriber:
    """Describe t_sys_org table structure"""

    def __init__(self):
        config = OrgTreeConfig()
        self.params = config.get_connection_params()
        self.conn = None
        self.cursor = None

    def connect(self):
        """Establish database connection"""
        try:
            self.conn = pymysql.connect(**self.params)
            self.cursor = self.conn.cursor(pymysql.cursors.DictCursor)
            return True
        except Exception as e:
            raise Exception(f"Database connection failed: {e}")

    def close(self):
        """Close database connection"""
        if self.cursor:
            self.cursor.close()
        if self.conn:
            self.conn.close()

    def get_table_schema(self):
        """Get table schema information"""
        self.cursor.execute(f"SHOW FULL COLUMNS FROM t_sys_org")
        return self.cursor.fetchall()

    def get_table_ddl(self):
        """Get CREATE TABLE statement"""
        self.cursor.execute(f"SHOW CREATE TABLE t_sys_org")
        result = self.cursor.fetchone()
        return result.get('Create Table', '') if result else ''

    def get_sample_data(self, limit=5):
        """Get sample data from table"""
        self.cursor.execute(f"SELECT * FROM t_sys_org LIMIT {limit}")
        return self.cursor.fetchall()

    def get_record_count(self):
        """Get total record count"""
        self.cursor.execute("SELECT COUNT(*) as count FROM t_sys_org")
        result = self.cursor.fetchone()
        return result.get('count', 0) if result else 0

    def describe(self):
        """Describe complete table structure"""
        try:
            self.connect()

            # Get schema info
            columns = self.get_table_schema()
            ddl = self.get_table_ddl()
            sample_data = self.get_sample_data()
            count = self.get_record_count()

            # Identify organization fields
            org_fields = {
                'id': None,
                'parent_field': None,
                'name_field': None,
                'code_field': None,
                'level_field': None,
                'left_field': None,
                'right_field': None
            }

            for col in columns:
                field = col.get('Field')
                if field == 'id':
                    org_fields['id'] = field
                elif field == 'pid':
                    org_fields['parent_field'] = field
                elif field == 'name':
                    org_fields['name_field'] = field
                elif field == 'org_code':
                    org_fields['code_field'] = field
                elif field == 'node_level':
                    org_fields['level_field'] = field
                elif field == 'left_num':
                    org_fields['left_field'] = field
                elif field == 'right_num':
                    org_fields['right_field'] = field

            return {
                "success": True,
                "data": {
                    "table_name": "t_sys_org",
                    "description": "System organization table with hierarchical structure using both Adjacency List and Nested Set models",
                    "fields": columns,
                    "organization_logic": {
                        "model": "Hybrid (Adjacency List + Nested Set)",
                        "parent_field": org_fields['parent_field'],
                        "name_field": org_fields['name_field'],
                        "code_field": org_fields['code_field'],
                        "level_field": org_fields['level_field'],
                        "left_field": org_fields['left_field'],
                        "right_field": org_fields['right_field'],
                        "explanation": {
                            "adjacency_list": f"Uses '{org_fields['parent_field']}' to establish parent-child relationships (NULL = root)",
                            "nested_set": f"Uses '{org_fields['left_field']}' and '{org_fields['right_field']}' for efficient tree traversal queries",
                            "level_tracking": f"Uses '{org_fields['level_field']}' to track depth level in the tree"
                        }
                    },
                    "ddl": ddl,
                    "record_count": count,
                    "sample_data": sample_data[:3]
                }
            }

        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "error_type": type(e).__name__
            }
        finally:
            self.close()


def describe_org_table():
    """Describe t_sys_org table structure and organization logic"""
    describer = OrgTableDescriber()
    return describer.describe()


def main():
    result = describe_org_table()
    print(json.dumps(result, indent=2, ensure_ascii=False))

    # Exit with error code if failed
    if not result.get('success'):
        sys.exit(1)


if __name__ == '__main__':
    main()
