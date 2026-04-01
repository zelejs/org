package com.jfeat.org.services.domain.model;

/**
 * Tenant Data Transfer Object with organization information
 *
 * @author Claude
 */
public class SysTenantDTO {
    /**
     * Tenant ID from t_sys_tenant.id
     */
    private Long id;

    /**
     * Tenant name from t_sys_tenant.name
     */
    private String name;

    /**
     * Tenant organization ID from t_sys_tenant.org_id
     */
    private Long orgId;

    /**
     * Organization code from t_sys_org.org_code
     */
    private String orgCode;

    /**
     * Organization name from t_sys_org.name
     */
    private String orgName;

    /**
     * Domain from t_sys_tenant.domain
     */
    private String domain;

    /**
     * Status from t_sys_tenant.status
     */
    private Integer status;

    /**
     * Application ID from t_sys_tenant.app_id
     */
    private String appId;

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public Long getOrgId() {
        return orgId;
    }

    public void setOrgId(Long orgId) {
        this.orgId = orgId;
    }

    public String getOrgCode() {
        return orgCode;
    }

    public void setOrgCode(String orgCode) {
        this.orgCode = orgCode;
    }

    public String getOrgName() {
        return orgName;
    }

    public void setOrgName(String orgName) {
        this.orgName = orgName;
    }

    public String getDomain() {
        return domain;
    }

    public void setDomain(String domain) {
        this.domain = domain;
    }

    public Integer getStatus() {
        return status;
    }

    public void setStatus(Integer status) {
        this.status = status;
    }

    public String getAppId() {
        return appId;
    }

    public void setAppId(String appId) {
        this.appId = appId;
    }
}
