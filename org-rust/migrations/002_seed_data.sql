-- Insert sample organization data for testing
-- Root organization
INSERT INTO t_sys_org (id, pid, name, full_name, org_code, node_level, left_num, right_num,
                       note, status, org_type, appid, is_visible, delete_flag, tenant_id, tenant_org_id, create_time, update_time)
VALUES (1, NULL, 'Root Organization', 'Root Organization', 'ROOT', 1, 1, 10,
        'Root organization', 'active', 1, '1', true, 0, 1, 1, NOW(), NOW());

-- Child organizations
INSERT INTO t_sys_org (id, pid, name, full_name, org_code, node_level, left_num, right_num,
                       note, status, org_type, appid, is_visible, delete_flag, tenant_id, tenant_org_id, create_time, update_time)
VALUES
    (2, 1, 'Engineering', 'Engineering Department', 'ENG', 2, 2, 5,
     'Engineering department', 'active', 2, '1', true, 0, 1, 1, NOW(), NOW()),

    (3, 1, 'Sales', 'Sales Department', 'SALES', 2, 6, 9,
     'Sales department', 'active', 2, '1', true, 0, 1, 1, NOW(), NOW()),

    (4, 2, 'Backend Team', 'Backend Development Team', 'BACKEND', 3, 3, 4,
     'Backend development team', 'active', 3, '1', true, 0, 1, 1, NOW(), NOW()),

    (5, 3, 'Enterprise Sales', 'Enterprise Sales Team', 'ENT-SALES', 3, 7, 8,
     'Enterprise sales team', 'active', 3, '1', true, 0, 1, 1, NOW(), NOW());

-- Create sequence for auto-increment IDs (for future inserts)
CREATE SEQUENCE IF NOT EXISTS t_sys_org_id_seq START WITH 6;
