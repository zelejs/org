package com.jfeat.org.services.domain.model;

import com.jfeat.org.services.persistence.model.SysOrg;

public class SysUserOrgRequest extends SysOrg {

    Long userId;
    String phone;

    public Long getUserId() {
        return userId;
    }

    public void setUserId(Long userId) {
        this.userId = userId;
    }

    public String getPhone() {
        return phone;
    }

    public void setPhone(String phone) {
        this.phone = phone;
    }
}
