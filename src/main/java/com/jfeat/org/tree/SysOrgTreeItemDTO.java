package com.jfeat.org.tree;

/**
 * @author : lvsongxin
 * @date :
 */
public class SysOrgTreeItemDTO extends TreeItemDTO<SysOrgTreeItemDTO> {
    private String name;

    private String fullName;

    private Integer nodeLevel;

    private String note;

    private Integer orgType;

    private Integer type;

    private String appid;

    private Long tenantId;

    private Long tenantOrgId;

    private Boolean tenantFlag;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getFullName() {
        return fullName;
    }

    public void setFullName(String fullName) {
        this.fullName = fullName;
    }

    public Integer getNodeLevel() {
        return nodeLevel;
    }

    public void setNodeLevel(Integer nodeLevel) {
        this.nodeLevel = nodeLevel;
    }

    public String getNote() {
        return note;
    }

    public void setNote(String note) {
        this.note = note;
    }

    public Integer getOrgType() {
        return orgType;
    }

    public void setOrgType(Integer orgType) {
        this.orgType = orgType;
    }

    public Integer getType() {
        return type;
    }

    public void setType(Integer type) {
        this.type = type;
    }

    public String getAppid() {
        return appid;
    }

    public void setAppid(String appid) {
        this.appid = appid;
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

    public Boolean getTenantFlag() {
        return tenantFlag;
    }

    public void setTenantFlag(Boolean tenantFlag) {
        this.tenantFlag = tenantFlag;
    }
}
