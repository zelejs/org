# Organization CLI

组织管理 CLI 工具 - 用于访问和管理组织 API

## Installation

```bash
cd /home/ubuntu/workspace/clis/org-cli
npm install
npm link
```

## Configuration

Configuration priority: CLI args → config file → environment variables

### Config file location
`~/.config/org-cli/config.json`

### Environment variables
- `ORG_BASE_URL` - API base URL
- `ORG_TOKEN` - Auth token

### .env file
```bash
ORG_BASE_URL=http://your-server/api/adm
ORG_TOKEN=your-token
```

## Commands

### tree - Display organization tree
```bash
org-cli tree [--appid <id>] [-d|--details]
```

### list - List organizations with pagination
```bash
org-cli list [--page <n>] [--page-size <n>] [--search <query>]
```

### get - Get specific organization details
```bash
org-cli get <id>
```

### config - Manage configuration
```bash
org-cli config --show           # Show current config
org-cli config --set <k> <v>    # Set config value
org-cli config --clear          # Clear config
```

## Options

- `--url <url>` - API base URL
- `--token <token>` - Auth token
- `--json` - Output in JSON format
- `-v, --verbose` - Enable verbose output
- `--timeout <sec>` - Request timeout in seconds
- `--save-config` - Save CLI args to config file

## Examples

```bash
# Show organization tree
org-cli tree --appid 1

# List organizations
org-cli list --page 1 --page-size 10

# Get specific organization
org-cli get 1

# Use custom URL
org-cli --url http://example.com/api tree

# JSON output
org-cli tree --json
```
