package com.jfeat.org.services.domain.model;


import com.jfeat.org.services.persistence.model.SysOrg;

import java.util.List;

/**
 * <p>
 * 节点和它所有父节点
 * </p>
 *
 * @author 莫昌廉
 */
public class SysOrgParentsModel extends SysOrg {

    List<SysOrg> parents;

    public List<SysOrg> getParents() {
        return parents;
    }

    public void setParents(List<SysOrg> parents) {
        this.parents = parents;
    }
}
