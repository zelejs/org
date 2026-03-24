-- Extension table for extended organization relationships
CREATE TABLE IF NOT EXISTS t_sys_org_ext (
    id BIGINT PRIMARY KEY,
    ext_org_id BIGINT NOT NULL,
    ext_org_type INTEGER,
    delete_flag INTEGER NOT NULL DEFAULT 0,
    create_time TIMESTAMP NOT NULL DEFAULT NOW(),
    update_time TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_t_sys_org_ext_ext_org_id ON t_sys_org_ext(ext_org_id);
CREATE INDEX IF NOT EXISTS idx_t_sys_org_ext_delete_flag ON t_sys_org_ext(delete_flag);
