#!/usr/bin/env node
/**
 * Organization Management CLI Tool
 *
 * For accessing and managing organization API, supports:
 * - Display organization tree structure
 * - List organizations with pagination
 * - Get specific organization details
 */

const fs = require('fs');
const path = require('path');
const http = require('http');
const https = require('https');

// Load .env file if exists
const dotenv = require('dotenv');
const envPaths = [
    path.join(__dirname, '..', '.env'),
    path.join(process.cwd(), '.env'),
    path.join(require('os').homedir(), '.env')
];
for (const envPath of envPaths) {
    if (fs.existsSync(envPath)) {
        dotenv.config({ path: envPath });
        break;
    }
}

const DEFAULT_TIMEOUT = 30000;

// Config file paths
const CONFIG_DIR = path.join(require('os').homedir(), '.config', 'org-cli');
const CONFIG_FILE = path.join(CONFIG_DIR, 'config.json');

/**
 * Organization Item class
 */
class OrganizationItem {
    constructor(data = {}) {
        this.id = data.id;
        this.pid = data.pid || data.parentId || 0;
        this.name = data.name || '';
        this.code = data.code || null;
        this.status = data.status || null;
        this.orderNum = data.orderNum || data.order_num || null;
        this.remark = data.remark || null;
        this.children = (data.children || []).map(child => new OrganizationItem(child));
    }

    toDict() {
        return {
            id: this.id,
            pid: this.pid,
            name: this.name,
            code: this.code,
            status: this.status,
            orderNum: this.orderNum,
            remark: this.remark,
            children: this.children.map(child => child.toDict())
        };
    }
}

/**
 * Organization Item with Tenant information
 */
class OrgTenantItem {
    constructor(data = {}) {
        this.id = data.id;
        this.pid = data.pid || data.parentId || 0;
        this.name = data.name || '';
        this.orgCode = data.orgCode || data.org_code || null;
        this.tenantCode = data.tenantCode || data.tenant_code || null;
        this.tenantName = data.tenantName || data.tenant_name || null;
        this.tenantId = data.tenantId || data.tenant_id || null;
        this.tenantOrgId = data.tenantOrgId || data.tenant_org_id || null;
        this.status = data.status || null;
        this.children = (data.children || []).map(child => new OrgTenantItem(child));
    }

    toDict() {
        return {
            id: this.id,
            pid: this.pid,
            name: this.name,
            orgCode: this.orgCode,
            tenantCode: this.tenantCode,
            tenantName: this.tenantName,
            tenantId: this.tenantId,
            tenantOrgId: this.tenantOrgId,
            status: this.status,
            children: this.children.map(child => child.toDict())
        };
    }
}

/**
 * Tenant Item class
 */
class TenantItem {
    constructor(data = {}) {
        this.id = data.id;
        this.name = data.name || '';
        this.orgId = data.orgId || data.org_id || null;
        this.orgCode = data.orgCode || data.org_code || null;
        this.orgName = data.orgName || data.org_name || null;
        this.domain = data.domain || null;
        this.status = data.status || null;
        this.appId = data.appId || data.app_id || null;
    }

    toDict() {
        return {
            id: this.id,
            name: this.name,
            orgId: this.orgId,
            orgCode: this.orgCode,
            orgName: this.orgName,
            domain: this.domain,
            status: this.status,
            appId: this.appId
        };
    }
}

/**
 * Configuration management
 */
function loadConfig() {
    try {
        if (fs.existsSync(CONFIG_FILE)) {
            const content = fs.readFileSync(CONFIG_FILE, 'utf8');
            return JSON.parse(content);
        }
    } catch (e) {
        // Ignore errors, return empty config
    }
    return {};
}

function saveConfig(config) {
    if (!fs.existsSync(CONFIG_DIR)) {
        fs.mkdirSync(CONFIG_DIR, { recursive: true });
    }
    fs.writeFileSync(CONFIG_FILE, JSON.stringify(config, null, 2));
}

/**
 * HTTP request wrapper
 */
function httpRequest(options) {
    return new Promise((resolve, reject) => {
        const url = new URL(options.url);
        const isHttps = url.protocol === 'https:';
        const client = isHttps ? https : http;

        const reqOptions = {
            hostname: url.hostname,
            port: url.port || (isHttps ? 443 : 80),
            path: url.pathname + url.search,
            method: options.method || 'GET',
            headers: options.headers || {},
            timeout: options.timeout || DEFAULT_TIMEOUT
        };

        if (options.body) {
            reqOptions.headers['Content-Length'] = Buffer.byteLength(options.body);
        }

        if (options.verbose) {
            console.error(`[DEBUG] ${reqOptions.method} ${options.url}`);
            if (options.params) {
                console.error(`[DEBUG] Params: ${JSON.stringify(options.params)}`);
            }
        }

        const req = client.request(reqOptions, (res) => {
            let data = '';

            res.on('data', chunk => {
                data += chunk;
            });

            res.on('end', () => {
                if (res.statusCode >= 200 && res.statusCode < 300) {
                    try {
                        resolve(JSON.parse(data));
                    } catch (e) {
                        resolve(data);
                    }
                } else {
                    const error = new Error(`HTTP ${res.statusCode}`);
                    error.statusCode = res.statusCode;
                    error.response = data;
                    try {
                        error.responseJson = JSON.parse(data);
                    } catch (e) {
                        // Ignore parse error
                    }
                    reject(error);
                }
            });
        });

        req.on('timeout', () => {
            req.destroy();
            reject(new Error(`Request timeout (${reqOptions.timeout}ms)`));
        });

        req.on('error', (err) => {
            if (err.code === 'ECONNREFUSED') {
                reject(new Error(`Cannot connect to server: ${options.url}`));
            } else {
                reject(err);
            }
        });

        if (options.body) {
            req.write(options.body);
        }

        req.end();
    });
}

/**
 * Organization API client
 */
class OrganizationAPIClient {
    constructor(options = {}) {
        const config = loadConfig();

        // Priority: CLI args > config file > env vars
        const rawBaseUrl = options.baseUrl ||
                          config.base_url ||
                          process.env.ORG_BASE_URL ||
                          process.env.ORG_CLI_BASE_URL;
        this.baseUrl = rawBaseUrl ? rawBaseUrl.replace(/\/$/, '') : null;

        this.token = options.token || config.token || process.env.ORG_TOKEN;
        this.timeout = options.timeout || DEFAULT_TIMEOUT;
        this.verbose = options.verbose || false;

        if (!this.baseUrl) {
            throw new Error('API endpoint not configured. Set it via:\n' +
                '  1. Command line: --url <URL>\n' +
                '  2. Config file: ~/.config/org-cli/config.json (base_url)\n' +
                '  3. Environment: ORG_BASE_URL or ORG_CLI_BASE_URL\n' +
                '  4. .env file: ORG_BASE_URL=http://your-server/api');
        }

        this.headers = {
            'Content-Type': 'application/json',
            'Accept': 'application/json'
        };

        if (this.token) {
            this.headers['Authorization'] = `Bearer ${this.token}`;
        }
    }

    async _request(method, endpoint, options = {}) {
        const url = `${this.baseUrl}${endpoint}`;
        const params = new URLSearchParams(options.params || {});
        const queryString = params.toString();
        const fullUrl = queryString ? `${url}?${queryString}` : url;

        try {
            return await httpRequest({
                url: fullUrl,
                method: method,
                headers: this.headers,
                timeout: this.timeout,
                verbose: this.verbose,
                body: options.body
            });
        } catch (error) {
            if (error.message.includes('timeout')) {
                console.error(`Error: Request timeout (${this.timeout / 1000}s)`);
                console.error('Hint: Use --timeout option to increase timeout');
            } else if (error.message.includes('Cannot connect')) {
                console.error(`Error: ${error.message}`);
                console.error(`URL: ${url}`);
                console.error('Hint: Check if URL is correct and server is running');
            } else if (error.statusCode) {
                console.error(`Error: HTTP ${error.statusCode}`);
                if (error.responseJson) {
                    console.error(`Details: ${JSON.stringify(error.responseJson)}`);
                } else {
                    console.error(`Details: ${error.response}`);
                }
            } else {
                console.error(`Request failed: ${error.message}`);
            }
            process.exit(1);
        }
    }

    async getOrgTree(appid = null) {
        const params = {};
        if (appid !== null) {
            params.appid = appid;
        }
        const result = await this._request('GET', '/adm/org/tree', { params });
        const data = result.data || result;
        // API returns { children: [...] } - extract children array
        const children = data.children || (Array.isArray(data) ? data : []);
        return children.map(item => new OrganizationItem(item));
    }

    async getOrgList(page = 1, pageSize = 10, search = null) {
        const params = { pageNum: page, pageSize: pageSize };
        if (search) params.name = search;
        return this._request('GET', '/adm/org', { params });
    }

    async getOrg(id) {
        return this._request('GET', `/adm/org/${id}`);
    }

    async getTenantTree(appid = null) {
        const params = {};
        if (appid !== null) {
            params.appid = appid;
        }
        const result = await this._request('GET', '/adm/org/tenant/tree', { params });
        const data = result.data || result;
        const children = data.children || (Array.isArray(data) ? data : []);
        return children.map(item => new OrgTenantItem(item));
    }

    async getTenantList(page = 1, pageSize = 10, search = null) {
        const params = { pageNum: page, pageSize: pageSize };
        if (search) params.search = search;
        return this._request('GET', '/adm/tenant', { params });
    }
}

/**
 * Organization tree printer
 */
class OrgTreePrinter {
    constructor(options = {}) {
        this.showDetails = options.showDetails || false;
    }

    printTree(items, prefix = '', isLast = true) {
        items.forEach((item, i) => {
            const isLastItem = i === items.length - 1;
            const connector = isLastItem ? '└── ' : '├── ';
            console.log(`${prefix}${connector}${this._formatItem(item)}`);

            if (item.children && item.children.length > 0) {
                const extension = isLastItem ? '    ' : '│   ';
                this.printTree(item.children, prefix + extension, isLastItem);
            }
        });
    }

    _formatItem(item) {
        const parts = [this._colorize(`[${item.id}] ${item.name}`, item.status)];

        if (this.showDetails) {
            if (item.code) {
                parts.push(`code: ${item.code}`);
            }
            if (item.status !== null && item.status !== undefined) {
                const statusStr = item.status === '0' ? '✓' : '✗';
                parts.push(`status: ${statusStr}`);
            }
            if (item.orderNum !== null) {
                parts.push(`order: ${item.orderNum}`);
            }
        }

        return this.showDetails ? parts.join(' ') : parts[0];
    }

    _colorize(text, status) {
        if (status === '1') {
            return `${text} [disabled]`;
        }
        return text;
    }
}

/**
 * Print tenant tree
 */
function printTenantTree(items, showDetails = false, prefix = '', isLast = true) {
    items.forEach((item, i) => {
        const isLastItem = i === items.length - 1;
        const connector = isLastItem ? '└── ' : '├── ';
        console.log(`${prefix}${connector}${formatTenantItem(item, showDetails)}`);

        if (item.children && item.children.length > 0) {
            const extension = isLastItem ? '    ' : '│   ';
            printTenantTree(item.children, showDetails, prefix + extension, isLastItem);
        }
    });
}

function formatTenantItem(item, showDetails) {
    const parts = [`[${item.id}] ${item.name}`];

    if (showDetails) {
        if (item.orgCode) {
            parts.push(`orgCode: ${item.orgCode}`);
        }
        if (item.tenantCode) {
            parts.push(`tenantCode: ${item.tenantCode}`);
        }
        if (item.tenantName) {
            parts.push(`tenant: ${item.tenantName}`);
        }
        if (item.tenantId) {
            parts.push(`tenantId: ${item.tenantId}`);
        }
    }

    return parts.join(' ');
}

/**
 * Print JSON output
 */
function printJson(data, pretty = true) {
    let output;

    if (Array.isArray(data)) {
        output = data.map(item => (item.toDict ? item.toDict() : item));
    } else if (data.toDict) {
        output = data.toDict();
    } else {
        output = data;
    }

    if (pretty) {
        console.log(JSON.stringify(output, null, 2));
    } else {
        console.log(JSON.stringify(output));
    }
}

/**
 * Command line argument parser
 */
function parseArgs() {
    const args = process.argv.slice(2);
    const options = {
        command: null,
        baseUrl: null,
        token: null,
        timeout: null,
        json: false,
        verbose: false,
        saveConfig: false,
        showHelp: false
    };

    for (let i = 0; i < args.length; i++) {
        const arg = args[i];

        switch (arg) {
            case '--url':
                options.baseUrl = args[++i];
                break;
            case '--token':
                options.token = args[++i];
                break;
            case '--timeout':
                options.timeout = parseInt(args[++i]) * 1000;
                break;
            case '--json':
                options.json = true;
                break;
            case '-v':
            case '--verbose':
                options.verbose = true;
                break;
            case '--save-config':
                options.saveConfig = true;
                break;
            case '-h':
            case '--help':
                options.showHelp = true;
                break;
            default:
                if (!arg.startsWith('-')) {
                    options.command = arg;
                    options.commandArgs = args.slice(i + 1);
                    return options;
                }
        }
    }

    return options;
}

/**
 * Parse sub-command arguments
 */
function parseCommandArgs(args, spec) {
    const result = {};
    const flags = spec.flags || [];

    for (let i = 0; i < args.length; i++) {
        const arg = args[i];

        const flag = flags.find(f => f.names.includes(arg));
        if (flag) {
            if (flag.hasValue) {
                result[flag.key] = args[++i];
            } else {
                result[flag.key] = true;
            }
        }
    }

    return result;
}

/**
 * Show help information
 */
function showHelp() {
    console.log(`
Organization CLI v1.1.0

USAGE:
  org-cli [options] <command> [args]

COMMANDS:
  tree           Show organization tree structure
  tree tenant    Show organization tree with tenant information
  list           List organizations with pagination
  tenant list    List tenants with pagination
  get <id>       Get specific organization details
  config         Manage configuration
  help           Show this help

OPTIONS:
  --url <url>        API base URL
  --token <token>    Auth token
  --json             JSON output
  -v, --verbose      Debug mode
  --timeout <sec>    Request timeout in seconds

EXAMPLES:
  org-cli tree --appid 1
  org-cli tree tenant --appid school-app
  org-cli list --page 1 --page-size 10
  org-cli tenant list --page 1
  org-cli get 1

CONFIG:
  Config: ~/.config/org-cli/config.json
  Env: ORG_BASE_URL, ORG_TOKEN
`);
}

/**
 * Main function
 */
async function main() {
    const options = parseArgs();

    if (options.showHelp || !options.command || options.command === 'help') {
        showHelp();
        process.exit(0);
    }

    // Handle config command
    if (options.command === 'config') {
        const config = loadConfig();
        const args = options.commandArgs || [];

        if (args.includes('--show')) {
            console.log('Current configuration:');
            console.log(JSON.stringify(config, null, 2));
        } else if (args.includes('--clear')) {
            if (fs.existsSync(CONFIG_FILE)) {
                fs.unlinkSync(CONFIG_FILE);
                console.log('Configuration cleared');
            } else {
                console.log('No configuration file found');
            }
        } else if (args.includes('--set')) {
            const idx = args.indexOf('--set');
            const key = args[idx + 1];
            const value = args[idx + 2];
            if (key && value) {
                config[key] = value;
                saveConfig(config);
                console.log(`Set: ${key} = ${value}`);
            } else {
                console.error('--set requires KEY and VALUE');
            }
        } else {
            console.log(`
CONFIG COMMANDS:
  --show              Show config
  --set <k> <v>       Set value
  --clear             Clear config
`);
        }
        return;
    }

    // Save configuration
    if (options.saveConfig) {
        const config = loadConfig();
        if (options.baseUrl) {
            config.base_url = options.baseUrl;
        }
        if (options.token) {
            config.token = options.token;
        }
        saveConfig(config);
        console.log(`Configuration saved to: ${CONFIG_FILE}`);
    }

    // Create API client
    let client;
    try {
        client = new OrganizationAPIClient({
            baseUrl: options.baseUrl,
            token: options.token,
            timeout: options.timeout,
            verbose: options.verbose
        });
    } catch (err) {
        console.error(`Error: ${err.message}`);
        showHelp();
        process.exit(1);
    }

    const commandArgs = options.commandArgs || [];

    // Handle sub-commands with spaces (e.g., "tree tenant")
    if (options.command === 'tree' && commandArgs.length > 0 && commandArgs[0] === 'tenant') {
        options.command = 'tree-tenant';
        options.commandArgs = commandArgs.slice(1);
    } else if (options.command === 'tenant' && commandArgs.length > 0 && commandArgs[0] === 'list') {
        options.command = 'tenant-list';
        options.commandArgs = commandArgs.slice(1);
    }

    if (options.command === 'tree') {
        const treeSpec = {
            flags: [
                { names: ['-d', '--details'], key: 'details', hasValue: false },
                { names: ['--appid'], key: 'appid', hasValue: true }
            ]
        };
        const cmdOptions = parseCommandArgs(commandArgs, treeSpec);
        const appid = cmdOptions.appid || null;

        const orgTree = await client.getOrgTree(appid);

        if (options.json) {
            printJson(orgTree);
        } else {
            const displayAppid = appid !== null ? appid : 'default';
            console.log(`Organization Tree (appid: ${displayAppid})`);
            console.log('-'.repeat(80));
            const printer = new OrgTreePrinter({ showDetails: cmdOptions.details });
            printer.printTree(orgTree);
        }

    } else if (options.command === 'list') {
        const listSpec = {
            flags: [
                { names: ['-p', '--page'], key: 'page', hasValue: true },
                { names: ['-s', '--page-size'], key: 'pageSize', hasValue: true },
                { names: ['--search'], key: 'search', hasValue: true }
            ]
        };
        const cmdOptions = parseCommandArgs(commandArgs, listSpec);
        const page = parseInt(cmdOptions.page) || 1;
        const pageSize = parseInt(cmdOptions.pageSize) || 10;

        const result = await client.getOrgList(page, pageSize, cmdOptions.search || null);

        if (options.json) {
            printJson(result);
        } else {
            const data = result.data || result;
            let records, total, pages, current;

            if (typeof data === 'object' && !Array.isArray(data)) {
                records = data.records || [];
                total = data.total || records.length;
                pages = data.pages || 1;
                current = data.current || page;
                console.log(`Organization List (Page ${current} of ${pages}, ${total} records)`);
            } else {
                records = data || [];
                total = records.length;
                console.log(`Organization List (${total} records)`);
            }

            console.log('-'.repeat(80));

            records.forEach(item => {
                const statusStr = item.status === '0' ? '✓' : '✗';
                const code = (item.code || 'N/A').padEnd(15);
                console.log(`[${item.id}] ${item.name} - code: ${code} status: ${statusStr}`);
            });
        }

    } else if (options.command === 'get') {
        const orgId = commandArgs[0];
        if (!orgId) {
            console.error('Error: Organization ID is required');
            console.error('Usage: org-cli get <id>');
            process.exit(1);
        }

        const id = parseInt(orgId);
        if (isNaN(id)) {
            console.error(`Error: Invalid organization ID: ${orgId}`);
            process.exit(1);
        }

        const result = await client.getOrg(id);

        if (options.json) {
            printJson(result);
        } else {
            const org = result.data || result;
            console.log(`Organization Details:`);
            console.log('-'.repeat(40));
            console.log(`ID:       ${org.id}`);
            console.log(`Name:     ${org.name}`);
            console.log(`Code:     ${org.code || 'N/A'}`);
            console.log(`Parent:   ${org.pid || org.parentId || 'N/A'}`);
            console.log(`Status:   ${org.status === '0' ? 'Active' : 'Disabled'}`);
            console.log(`Order:    ${org.orderNum || org.order_num || 'N/A'}`);
            if (org.remark) {
                console.log(`Remark:   ${org.remark}`);
            }
        }

    } else if (options.command === 'tree-tenant') {
        const treeSpec = {
            flags: [
                { names: ['-d', '--details'], key: 'details', hasValue: false },
                { names: ['--appid'], key: 'appid', hasValue: true }
            ]
        };
        const cmdOptions = parseCommandArgs(options.commandArgs, treeSpec);
        const appid = cmdOptions.appid || null;

        const tenantTree = await client.getTenantTree(appid);

        if (options.json) {
            printJson(tenantTree);
        } else {
            const displayAppid = appid !== null ? appid : 'default';
            console.log(`Organization Tree with Tenant Info (appid: ${displayAppid})`);
            console.log('-'.repeat(80));
            printTenantTree(tenantTree, cmdOptions.details);
        }

    } else if (options.command === 'tenant-list') {
        const listSpec = {
            flags: [
                { names: ['-p', '--page'], key: 'page', hasValue: true },
                { names: ['-s', '--page-size'], key: 'pageSize', hasValue: true },
                { names: ['--search'], key: 'search', hasValue: true }
            ]
        };
        const cmdOptions = parseCommandArgs(options.commandArgs, listSpec);
        const page = parseInt(cmdOptions.page) || 1;
        const pageSize = parseInt(cmdOptions.pageSize) || 10;

        const result = await client.getTenantList(page, pageSize, cmdOptions.search || null);

        if (options.json) {
            printJson(result);
        } else {
            const data = result.data || result;
            let records, total, pages, current;

            if (typeof data === 'object' && !Array.isArray(data)) {
                records = data.records || [];
                total = data.total || records.length;
                pages = data.pages || 1;
                current = data.current || page;
                console.log(`Tenant List (Page ${current} of ${pages}, ${total} records)`);
            } else {
                records = data || [];
                total = records.length;
                console.log(`Tenant List (${total} records)`);
            }

            console.log('-'.repeat(80));

            records.forEach(item => {
                const statusStr = item.status === 1 ? '✓' : '✗';
                const orgCode = (item.orgCode || 'N/A').padEnd(15);
                console.log(`[${item.id}] ${item.name} - orgCode: ${orgCode} status: ${statusStr}`);
                if (cmdOptions.details) {
                    console.log(`    Org ID: ${item.orgId || 'N/A'}`);
                    console.log(`    Org Name: ${item.orgName || 'N/A'}`);
                    console.log(`    Domain: ${item.domain || 'N/A'}`);
                }
            });
        }

    } else {
        console.error(`Unknown command: ${options.command}`);
        showHelp();
        process.exit(1);
    }
}

// Run main function
main().catch(err => {
    console.error(`Error: ${err.message}`);
    process.exit(1);
});
