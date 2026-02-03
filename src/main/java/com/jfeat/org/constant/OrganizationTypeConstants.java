package com.jfeat.org.constant;

/**
 * @description: 组织类型常量
 * @project: uaas
 * @date: 2024/1/16 16:14
 * @author: hhhhhtao
 */
public class OrganizationTypeConstants {

    /**
     * 平台 顶层节点
     */
    public static final Integer PLATFORM = 0;

    /**
     * 租户 租户节点
     */
    public static final Integer TENANT = 1;

    /**
     * 公司 顶层跟租户都可建
     */
    public static final Integer COMPANY = 2;

    /**
     * 分公司 顶层跟租户都可建
     */
    public static final Integer SUBSIDIARIES = 3;

    /**
     * 部门 顶层跟租户都可建
     */
    public static final Integer DEPARTMENT = 4;

    /**
     * 工作组 顶层跟租户都可建
     */
    public static final Integer WORKING_TEAM = 5;

    private OrganizationTypeConstants() {}
}
