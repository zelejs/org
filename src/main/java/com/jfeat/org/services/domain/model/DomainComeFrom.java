package com.jfeat.org.services.domain.model;

public class DomainComeFrom {

    private Long orgId;
    private String comeFrom;
    private Long rootOrg;

    public Long getRootOrg() {
        return rootOrg;
    }

    public void setRootOrg(Long rootOrg) {
        this.rootOrg = rootOrg;
    }

    public Long getOrgId() {
        return orgId;
    }

    public void setOrgId(Long orgId) {
        this.orgId = orgId;
    }

    public String getComeFrom() {
        return comeFrom;
    }

    public void setComeFrom(String comeFrom) {
        this.comeFrom = comeFrom;
    }
}
