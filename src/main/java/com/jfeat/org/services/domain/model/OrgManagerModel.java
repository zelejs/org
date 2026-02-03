package com.jfeat.org.services.domain.model;

import com.jfeat.org.services.persistence.model.SysOrg;
import com.jfeat.org.services.persistence.model.SysUser;

import java.util.List;

/**
 * <p></p>
 *
 * @Author 莫昌廉
 * @Date 2018/11/13 16:17
 **/
public class OrgManagerModel extends SysOrg {
    List<SysUser> managers;

    public List<SysUser> getManagers() {
        return managers;
    }

    public void setManagers(List<SysUser> managers) {
        this.managers = managers;
    }
}
