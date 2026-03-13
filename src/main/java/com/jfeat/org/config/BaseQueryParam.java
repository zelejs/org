package com.jfeat.org.config;

import com.jfeat.am.core.jwt.JWTKit;
import lombok.Data;

@Data
public class BaseQueryParam {
    public static final Integer FILTER_CHILDREN = 1;
    public static final Integer FILTER_CHILDREN_AND_TENANT = 2;

    private Long orgId;
    private Long tenantOrgId;
    private String appid;
    // 1 所有子树 2 所有子树&&自己的租户
    private Integer filterType;

    public static BaseQueryParam create(Long tenantOrgId, Long orgId, Integer filterType) {
        if(tenantOrgId == null) {
            tenantOrgId = JWTKit.getOrgId();
        }
        if(orgId == null) {
            orgId = JWTKit.getTenantOrgId();
        }
        if(filterType == null) {
            filterType = BaseQueryParam.FILTER_CHILDREN_AND_TENANT;
        }
        BaseQueryParam baseQueryParam = new BaseQueryParam();
        baseQueryParam.setOrgId(orgId);
        baseQueryParam.setTenantOrgId(tenantOrgId);
        baseQueryParam.setAppid(JWTKit.getAppid());
        baseQueryParam.setFilterType(filterType);
        return baseQueryParam;
    }

    public static BaseQueryParam create() {
        return BaseQueryParam.create(null,null,null);
    }

    public static BaseQueryParam create(Long tenantOrgId, Long orgId) {
        return BaseQueryParam.create(tenantOrgId,orgId,null);
    }

    public static BaseQueryParam createChildFilter() {
        return BaseQueryParam.create(null,null,FILTER_CHILDREN);
    }

    public static BaseQueryParam createChildFilter(Long tenantOrgId, Long orgId) {
        return BaseQueryParam.create(tenantOrgId,orgId,FILTER_CHILDREN);
    }
}
