SET FOREIGN_KEY_CHECKS=0;

DELETE FROM `t_sys_org` WHERE `id` = '1';
INSERT INTO `t_sys_org` (`id`, `pid`, `org_type`,`org_code`, `name`, `full_name`, `node_level`, `left_num`, `right_num`, `note`, `status`)
                 VALUES ('1', null, 2, 'PLATFORM', 'PLATFORM', 'Default Platform Tech.', '1', '1', '2', '平台', 'NORMAL');
