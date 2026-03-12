#!/usr/bin/env python3
"""
Configuration loader for org-tree skill
Loads database config from skill directory or parent directory
"""

import os
import yaml
from pathlib import Path


class OrgTreeConfig:
    """Load and manage database configuration for org-tree skill"""

    # Config file search paths (priority: high to low)
    CONFIG_PATHS = [
        # Project root (highest priority)
        lambda: Path.cwd() / 'config.yaml',
        # Parent of scripts directory
        lambda: Path(__file__).parent.parent / 'config.yaml',
        # User-level chatdb config (fallback)
        lambda: Path.home() / '.claude' / 'skills' / 'chatdb' / 'scripts' / 'config.yaml',
    ]

    DEFAULT_CONFIG = {
        'current': {'active': 'primary'},
        'primary': {
            'host': 'localhost',
            'port': 3306,
            'user': 'root',
            'password': '',
            'database': 'study_power',
            'charset': 'utf8mb4',
        }
    }

    def __init__(self, config_file=None):
        self.config_file = config_file
        self.config_data = None

    @classmethod
    def load(cls, config_file=None):
        """Load configuration from file with automatic path discovery"""
        if config_file:
            return cls._load_from_file(Path(config_file))

        # Try each path in priority order
        for path_func in cls.CONFIG_PATHS:
            try:
                path = path_func()
                if path.exists():
                    return cls._load_from_file(path)
            except Exception:
                continue

        # No config found, use defaults
        return cls.DEFAULT_CONFIG

    @classmethod
    def _load_from_file(cls, path):
        """Load configuration from specific file"""
        with open(path, 'r', encoding='utf-8') as f:
            return yaml.safe_load(f)

    def get_active_section(self):
        """Get the active database connection section"""
        if not self.config_data:
            self.config_data = self.load(self.config_file)

        active = self.config_data.get('current', {}).get('active', 'primary')
        return self.config_data.get(active, self.config_data.get('primary', {}))

    def get_connection_params(self):
        """Get database connection parameters"""
        config = self.get_active_section()
        return {
            'host': config.get('host', 'localhost'),
            'port': config.get('port', 3306),
            'user': config.get('user', 'root'),
            'password': config.get('password', ''),
            'database': config.get('database', 'study_power'),
            'charset': config.get('charset', 'utf8mb4'),
        }


if __name__ == '__main__':
    # Test config loading
    config = OrgTreeConfig.load()
    print("Active config:", config.get_active_section())
    print("Connection params:", OrgTreeConfig().get_connection_params())
