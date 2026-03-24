-- Migration script: Rename party_org columns to ext_org
-- This aligns the Java org service with the org-rust service

-- Check if table exists, if not create it
CREATE TABLE IF NOT EXISTS `t_sys_org_ext` (
  `id` bigint NOT NULL PRIMARY KEY COMMENT '组织ID',
  `ext_org_id` bigint NOT NULL COMMENT '扩展组织ID',
  `ext_org_type` int DEFAULT NULL COMMENT '扩展组织类型',
  `delete_flag` tinyint(1) NOT NULL DEFAULT 0 COMMENT '删除标志(0-正常 1-已删除)',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  KEY `idx_ext_org_id` (`ext_org_id`),
  KEY `idx_delete_flag` (`delete_flag`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='扩展组织表';

-- If table exists with old column names, rename them
-- Note: This may fail if columns don't exist, which is fine
ALTER TABLE `t_sys_org_ext`
  CHANGE COLUMN `party_org_id` `ext_org_id` BIGINT NOT NULL COMMENT '扩展组织ID',
  CHANGE COLUMN `party_org_type` `ext_org_type` INT DEFAULT NULL COMMENT '扩展组织类型';
