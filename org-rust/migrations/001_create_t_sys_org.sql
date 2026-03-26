-- Create t_sys_org table for organization management
CREATE TABLE IF NOT EXISTS t_sys_org (
    id BIGINT PRIMARY KEY,
    pid BIGINT,
    name VARCHAR(255) NOT NULL,
    full_name VARCHAR(512),
    org_code VARCHAR(128),
    node_level INTEGER,
    left_num INTEGER,
    right_num INTEGER,
    note TEXT,
    status VARCHAR(64),
    org_type INTEGER COMMENT '组织类型0-平台/应用,1-租户,2-分公司/学院/局,3-部门/科/村镇,4-用户组织',
    appid VARCHAR(128),
    icon VARCHAR(512),
    is_visible BOOLEAN DEFAULT TRUE,
    need_validate BOOLEAN DEFAULT FALSE,
    delete_flag INTEGER NOT NULL DEFAULT 0,
    tenant_id BIGINT,
    tenant_org_id BIGINT,
    create_time TIMESTAMP,
    update_time TIMESTAMP
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_t_sys_org_pid ON t_sys_org(pid);
CREATE INDEX IF NOT EXISTS idx_t_sys_org_appid ON t_sys_org(appid);
CREATE INDEX IF NOT EXISTS idx_t_sys_org_tenant_org_id ON t_sys_org(tenant_org_id);
CREATE INDEX IF NOT EXISTS idx_t_sys_org_delete_flag ON t_sys_org(delete_flag);

-- Add comments
COMMENT ON TABLE t_sys_org IS 'Organization tree structure table using Nested Set Model';
COMMENT ON COLUMN t_sys_org.id IS 'Organization ID';
COMMENT ON COLUMN t_sys_org.pid IS 'Parent organization ID';
COMMENT ON COLUMN t_sys_org.name IS 'Organization name';
COMMENT ON COLUMN t_sys_org.full_name IS 'Full organization name';
COMMENT ON COLUMN t_sys_org.org_code IS 'Organization code';
COMMENT ON COLUMN t_sys_org.node_level IS 'Node level in tree';
COMMENT ON COLUMN t_sys_org.left_num IS 'Left value for Nested Set Model';
COMMENT ON COLUMN t_sys_org.right_num IS 'Right value for Nested Set Model';
COMMENT ON COLUMN t_sys_org.appid IS 'Application ID';
COMMENT ON COLUMN t_sys_org.tenant_id IS 'Tenant ID';
COMMENT ON COLUMN t_sys_org.tenant_org_id IS 'Tenant organization ID';
