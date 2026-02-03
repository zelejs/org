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
public class SysOrgChildNodeModel extends SysOrg {

    List<SysOrg> childNode;


    // only one ,only search parent,not and never search gra parent
    SysOrg parentNode;


    public List<SysOrg> getChildNode() {
        return childNode;
    }

    public void setChildNode(List<SysOrg> childNode) {
        this.childNode = childNode;
    }


    public SysOrg getParentNode() {
        return parentNode;
    }

    public void setParentNode(SysOrg parentNode) {
        this.parentNode = parentNode;
    }
}
