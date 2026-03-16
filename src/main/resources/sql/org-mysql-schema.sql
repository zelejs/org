SET FOREIGN_KEY_CHECKS=0;

DROP TABLE IF EXISTS `t_sys_org`;
CREATE TABLE `t_sys_org` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键id',
  `pid` bigint DEFAULT NULL COMMENT '父部门id',
  `name` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '简称',
  `full_name` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '全称',
  `org_code` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '组织代码',
  `appid` varchar(50) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '应用ID，代表最高层级组织，appid=null为默认appid',
  `node_level` int DEFAULT NULL COMMENT '所在层级',
  `left_num` int DEFAULT NULL COMMENT '左下标',
  `right_num` int DEFAULT NULL COMMENT '右下标',
  `note` varchar(500) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '部门描述',
  `status` varchar(50) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '状态',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `org_type` int DEFAULT NULL COMMENT '组织类型',
  `b_type` varchar(50) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT 'B类型',
  `icon` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT '图标',
  `is_visible` tinyint(1) DEFAULT '1' COMMENT '是否可见',
  `need_validate` tinyint(1) DEFAULT '0' COMMENT '是否需要验证',
  `delete_flag` tinyint(1) DEFAULT '0' COMMENT '删除标志(0-正常 1-已删除)',
  `tenant_id` bigint DEFAULT NULL COMMENT '组织所属租户',
  `tenant_org_id` bigint DEFAULT NULL COMMENT '组织所属租户,租户为项级组织(org_id)',
  PRIMARY KEY (`id`),
  KEY `idx_pid` (`pid`),
  KEY `idx_tenant_id` (`tenant_id`),
  KEY `idx_tenant_org_id` (`tenant_org_id`),
  KEY `idx_org_code` (`org_code`)
) ENGINE=InnoDB AUTO_INCREMENT=1015 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='系统组织表';

-- ----------------------------
-- Table structure for t_sys_position
-- ----------------------------
DROP TABLE IF EXISTS `t_sys_position`;
CREATE TABLE `t_sys_position` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL COMMENT '职位名称',
  `pos_code` varchar(20) NOT NULL COMMENT '职位代码',
  `org_id` bigint(20) NOT NULL COMMENT '职位所属部门id',
  `note` text COMMENT '职位描述',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8;


-- ----------------------------
-- Table structure for t_sys_org_user_relation
-- ----------------------------
DROP TABLE IF EXISTS `t_sys_org_user_relation`;
CREATE TABLE `t_sys_org_user_relation` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键id',
  `user_id` bigint(20) NOT NULL COMMENT '员工ID',
  `org_position` varchar(100) NOT NULL COMMENT '职位(职称)',
  `org_id` bigint(20) NOT NULL COMMENT '组织id',
  `position_id` bigint(20) NOT NULL COMMENT '职位id',
  `is_leader` smallint(6) NOT NULL DEFAULT '0' COMMENT '是否是管理员',
  `is_primary` smallint(6) NOT NULL DEFAULT '0' COMMENT '是否为主要的',
  PRIMARY KEY (`id`),
  KEY `t_sys_org_user_relation_org_id` (`org_id`)
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8;


SET FOREIGN_KEY_CHECKS=1;

