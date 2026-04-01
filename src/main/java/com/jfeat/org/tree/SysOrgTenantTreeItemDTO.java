package com.jfeat.org.tree;

/**
 * Organization tree item with tenant information
 *
 * @author Claude
 */
public class SysOrgTenantTreeItemDTO extends SysOrgTreeItemDTO {
    /**
     * Tenant code from t_sys_org.org_code
     */
    private String tenantCode;

    /**
     * Tenant name from t_sys_tenant.name
     */
    private String tenantName;

    /**
     * Tenant ID from t_sys_tenant.id
     */
    private Long tenantId;

    /**
     * Tenant organization ID from t_sys_tenant.org_id
     */
    private Long tenantOrgId;

    public String getTenantCode() {
        return tenantCode;
    }

    public void setTenantCode(String tenantCode) {
        this.tenantCode = tenantCode;
    }

    public String getTenantName() {
        return tenantName;
    }

    public void setTenantName(String tenantName) {
        this.tenantName = tenantName;
    }

    public Long getTenantId() {
        return tenantId;
    }

    public void setTenantId(Long tenantId) {
        this.tenantId = tenantId;
    }

    public Long getTenantOrgId() {
        return tenantOrgId;
    }

    public void setTenantOrgId(Long tenantOrgId) {
        this.tenantOrgId = tenantOrgId;
    }
}
