SET FOREIGN_KEY_CHECKS=0;

DROP TABLE IF EXISTS `t_sys_org`;
CREATE TABLE `t_sys_org` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键id',
  `pid` bigint(20) DEFAULT NULL COMMENT '父部门id',
  `name` varchar(60) NOT NULL COMMENT '部门名称',
  `org_code` varchar(50) DEFAULT NULL COMMENT '部门编号',
  `org_type` smallint NOT NULL DEFAULT 0 COMMENT '默认组织 org_type = 0, 租户 org_type = 1, 平台 org_type = 2',
  `tenant_id` bigint(20) DEFAULT NULL COMMENT '所属类型为租户的父组织',
  `full_name` varchar(128) DEFAULT NULL COMMENT '部门全称',
  `node_level` int(11) NOT NULL DEFAULT 0 COMMENT '所在层级 (0,1,2)',
  `left_num` int(11) NOT NULL DEFAULT 1 COMMENT '左下标',
  `right_num` int(11) NOT NULL DEFAULT 2 COMMENT '右下标',
  `note` text DEFAULT NULL COMMENT '部门描述',
  `status` varchar(26) NOT NULL DEFAULT 'NORMAL' COMMENT '状态',
  `b_type` varchar(30) default 'SYSTEM' COMMENT '默认为系统平台为 SYSTEM    用户组织为 USER ',
  `need_validate` tinyint(1) DEFAULT  0 COMMENT '需要重设密码 针对租户id为本组织的所有用户起效',
  `is_visible` tinyint(1) DEFAULT  0 COMMENT '可见的',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` datetime DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
  `icon` varchar(255) DEFAULT NULL COMMENT '图标地址',
  UNIQUE(`org_code`),
  UNIQUE(`tenant_id`,`name`),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8;

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
-- Table structure for t_org_user_relation
-- ----------------------------
DROP TABLE IF EXISTS `t_org_user_relation`;
CREATE TABLE `t_org_user_relation` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键id',
  `user_id` bigint(20) NOT NULL COMMENT '员工ID',
  `org_position` varchar(100) NOT NULL COMMENT '职位(职称)',
  `org_id` bigint(20) NOT NULL COMMENT '组织id',
  `position_id` bigint(20) NOT NULL COMMENT '职位id',
  `is_leader` smallint(6) NOT NULL DEFAULT '0' COMMENT '是否是管理员',
  `is_primary` smallint(6) NOT NULL DEFAULT '0' COMMENT '是否为主要的',
  PRIMARY KEY (`id`),
  KEY `t_org_user_relation_org_id` (`org_id`)
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8;


SET FOREIGN_KEY_CHECKS=1;

