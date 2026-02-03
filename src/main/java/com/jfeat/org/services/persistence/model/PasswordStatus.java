package com.jfeat.org.services.persistence.model;

public class PasswordStatus {
    //未重置密码
    public static Integer NOT_REST=0;
    //已经重置密码
    public static Integer REST=1;
    //需要验证
    public static String START_CHECK = "true";
    //不开启验证
    public static String NO_CHECK = "false";
}
