SET FOREIGN_KEY_CHECKS=0;

DROP TABLE IF EXISTS `t_sys_org`;
CREATE TABLE `t_sys_org` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键id',
  `pid` bigint(20) DEFAULT NULL COMMENT '父组织id (NULL表示根节点)',
  `name` varchar(60) NOT NULL COMMENT '组织名称',
  `full_name` varchar(128) DEFAULT NULL COMMENT '组织全称',
  `org_code` varchar(50) DEFAULT NULL COMMENT '组织编号 (唯一)',
  `org_type` smallint NOT NULL DEFAULT 0 COMMENT '默认组织0, 租户1, 平台2',
  `node_level` int(11) NOT NULL DEFAULT 0 COMMENT '所在层级 (0,1,2,...)',
  `left_num` int(11) NOT NULL DEFAULT 1 COMMENT '左边界 (预排序)',
  `right_num` int(11) NOT NULL DEFAULT 2 COMMENT '右边界 (预排序)',
  `tenant_id` bigint(20) DEFAULT NULL COMMENT '租户组织id',
  `tenant_org_id` bigint(20) DEFAULT NULL COMMENT '顶级租户组织id',
  `status` varchar(26) NOT NULL DEFAULT 'NORMAL' COMMENT '状态',
  `type` tinyint DEFAULT 0 COMMENT '组织类型0-平台/应用,1-租户,2-分公司/学院/局,3-部门/科/村镇,4-用户组织',
  `is_visible` tinyint(1) DEFAULT 0 COMMENT '可见标志',
  `need_validate` tinyint(1) DEFAULT 0 COMMENT '需要重设密码 (对该组织的所有用户生效)',
  `delete_flag` tinyint(1) DEFAULT 0 COMMENT '软删除标志 (0=正常, 1=已删除)',
  `note` text DEFAULT NULL COMMENT '组织描述',
  `icon` varchar(255) DEFAULT NULL COMMENT '图标地址',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
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

