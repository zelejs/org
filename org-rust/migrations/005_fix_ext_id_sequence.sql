-- Create a sequence for t_sys_org_ext
CREATE SEQUENCE IF NOT EXISTS t_sys_org_ext_id_seq;

-- Set the default value for the id column
ALTER TABLE t_sys_org_ext ALTER COLUMN id SET DEFAULT nextval('t_sys_org_ext_id_seq');

-- Set the sequence ownership
ALTER SEQUENCE t_sys_org_ext_id_seq OWNED BY t_sys_org_ext.id;
