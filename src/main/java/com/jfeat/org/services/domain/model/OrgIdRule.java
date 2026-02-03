package com.jfeat.org.services.domain.model;

public class OrgIdRule {
    //最大长度
    public final static Integer MAX_LENGTH = 18;
    //组织长度
    public final static Integer ORG_LENGTH = 9;
    //子组织长度
    public final static Integer SON_ORG_LENGTH = MAX_LENGTH - ORG_LENGTH;

    //权限组长度
    public final static Integer PERM_GROUP_LENGTH = 9;
    //子权限组长度
    public final static Integer PERM_LENGTH = MAX_LENGTH - PERM_GROUP_LENGTH;

}
