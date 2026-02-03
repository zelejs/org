package com.jfeat.org.services.domain.model;

import com.jfeat.org.services.persistence.model.SysOrg;

import java.util.List;

public class SysOrgChildParentsModel extends SysOrg {

    List<SysOrg> childNode;

    List<SysOrg> parents;


    public List<SysOrg> getChildNode() {
        return childNode;
    }

    public void setChildNode(List<SysOrg> childNode) {
        this.childNode = childNode;
    }

    public List<SysOrg> getParents() {
        return parents;
    }

    public void setParents(List<SysOrg> parents) {
        this.parents = parents;
    }
}
