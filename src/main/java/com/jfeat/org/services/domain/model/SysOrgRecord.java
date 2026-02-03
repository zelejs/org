package com.jfeat.org.services.domain.model;


import com.jfeat.org.services.persistence.model.SysOrg;
import com.jfeat.org.services.persistence.model.SysUser;

import java.util.List;

/**
 * <p>
 * 节点和它所有父节点
 * </p>
 *
 * @author 莫昌廉
 */
public class SysOrgRecord extends SysOrg {
    /**
     * 电话
     */
    private String phone;
    /**
     * （负责人）昵称
     */
    private String userName;
    /**
     * 人数
     */
    private Integer peopleCount;
    /**
     * 总人数
     */
    private Integer employeeCount;

    //部门人数
    private Integer deptCount;

    private Long orgId;

    private Long userId;

    public Long getUserId() {
        return userId;
    }

    public void setUserId(Long userId) {
        this.userId = userId;
    }

    public Long getOrgId() {
        return orgId;
    }

    public void setOrgId(Long orgId) {
        this.orgId = orgId;
    }

    public Integer getEmployeeCount() {
        return employeeCount;
    }

    public void setEmployeeCount(Integer employeeCount) {
        this.employeeCount = employeeCount;
    }

    public String getPhone() {
        return phone;
    }

    public void setPhone(String phone) {
        this.phone = phone;
    }

    public String getUserName() {
        return userName;
    }

    public void setUserName(String userName) {
        this.userName = userName;
    }

    public Integer getPeopleCount() {
        return peopleCount;
    }

    public void setPeopleCount(Integer peopleCount) {
        this.peopleCount = peopleCount;
    }

    public Integer getDeptCount() {
        return deptCount;
    }

    public void setDeptCount(Integer deptCount) {
        this.deptCount = deptCount;
    }
}
