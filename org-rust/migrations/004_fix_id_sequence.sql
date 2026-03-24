-- Create a sequence for auto-incrementing IDs
CREATE SEQUENCE IF NOT EXISTS t_sys_org_id_seq;

-- Set the default value for the id column
ALTER TABLE t_sys_org ALTER COLUMN id SET DEFAULT nextval('t_sys_org_id_seq');

-- Set the sequence ownership
ALTER SEQUENCE t_sys_org_id_seq OWNED BY t_sys_org.id;

-- Set the current value to be higher than existing IDs
SELECT setval('t_sys_org_id_seq', (SELECT COALESCE(MAX(id), 0) + 1 FROM t_sys_org));
