package com.jfeat.org.services.domain.model;


import com.jfeat.org.services.persistence.model.SysOrg;
import com.jfeat.org.services.persistence.model.SysUser;

import java.util.List;

/**
 * <p>
 * 员工部门关系实体
 * </p>
 *
 * @author 莫昌廉
 */
public class OrgUserRelationModel extends SysOrg {
   List<SysUser> users;

    public List<SysUser> getUsers() {
        return users;
    }

    public void setUsers(List<SysUser> users) {
        this.users = users;
    }
}
