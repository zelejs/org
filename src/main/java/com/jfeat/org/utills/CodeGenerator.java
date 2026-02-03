package com.jfeat.org.utills;

import java.util.Random;

/**
 * @author : lvsongxin
 */
@Deprecated
public class CodeGenerator {
    private static String str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    public static String gen(int length){
        Random random = new Random();
        StringBuilder stringBuffer = new StringBuilder();
        for (int i = 0; i < length; i++) {
            int number = random.nextInt(62);
            stringBuffer.append(str.charAt(number));

        }
        return stringBuffer.toString();
    }
}
