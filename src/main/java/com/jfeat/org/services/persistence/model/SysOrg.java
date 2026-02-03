package com.jfeat.org.services.persistence.model;

import com.baomidou.mybatisplus.annotation.IdType;

import java.util.Date;

import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.extension.activerecord.Model;
import com.baomidou.mybatisplus.annotation.TableName;

import java.io.Serializable;

/**
 * <p>
 *
 * </p>
 *
 * @author admin
 * @since 2018-10-26
 */
@TableName("t_sys_org")
public class SysOrg extends Model<SysOrg> {

    private static final long serialVersionUID = 1L;

    /**
     * 主键id
     */
    @TableId(value = "id", type = IdType.AUTO)
    private Long id;
    /**
     * 父部门id
     */
    private Long pid;
    /**
     * 简称
     */
    private String name;
    /**
     * 全称
     */
    @TableField("full_name")
    private String fullName;

    /**
     * 全称
     */
    @TableField("org_code")
    private String orgCode;
    /**
     * 所在层级
     */
    @TableField("node_level")
    private Integer nodeLevel;
    /**
     * 左下标
     */
    @TableField("left_num")
    private Integer leftNum;
    /**
     * 右下标
     */
    @TableField("right_num")
    private Integer rightNum;
    /**
     * 部门描述
     */
    private String note;
    /**
     * 状态
     */
    private String status;
    @TableField("create_time")
    private Date createTime;
    @TableField("update_time")
    private Date updateTime;

    @TableField("org_type")
    private Integer orgType;

    @TableField("b_type")
    private String bType;

    @TableField("icon")
    private String icon;

    @TableField("is_visible")
    private Boolean isVisible;

    @TableField("need_validate")
    private Boolean needValidate;

    //0-正常 1-已删除
    @TableField("delete_flag")
    private Boolean deleteFlag;

    /**
     * 组织所属租户
     */
    @TableField("tenant_id")
    private Long tenantId;

    /**
     * 组织所属租户, 租户为项级组织 (org_id)
     */
    @TableField("tenant_org_id")
    private Long tenantOrgId;
    //end CR


    public String getIcon() {
        return icon;
    }

    public void setIcon(String icon) {
        this.icon = icon;
    }

    public Boolean getVisible() {
        return isVisible;
    }

    public void setVisible(Boolean visible) {
        isVisible = visible;
    }

    public Boolean getNeedValidate() {
        return needValidate;
    }

    public void setNeedValidate(Boolean needValidate) {
        this.needValidate = needValidate;
    }

    public String getbType() {
        return bType;
    }

    public void setbType(String bType) {
        this.bType = bType;
    }

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public Long getPid() {
        return pid;
    }

    public void setPid(Long pid) {
        this.pid = pid;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getFullName() {
        return fullName;
    }


    public String getOrgCode() {
        return orgCode;
    }

    public void setOrgCode(String orgCode) {
        this.orgCode = orgCode;
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

    public Integer getLeftNum() {
        return leftNum;
    }

    public void setLeftNum(Integer leftNum) {
        this.leftNum = leftNum;
    }

    public Integer getRightNum() {
        return rightNum;
    }

    public void setRightNum(Integer rightNum) {
        this.rightNum = rightNum;
    }

    public String getNote() {
        return note;
    }

    public void setNote(String note) {
        this.note = note;
    }

    public String getStatus() {
        return status;
    }

    public void setStatus(String status) {
        this.status = status;
    }

    public Date getCreateTime() {
        return createTime;
    }

    public void setCreateTime(Date createTime) {
        this.createTime = createTime;
    }

    public Date getUpdateTime() {
        return updateTime;
    }

    public void setUpdateTime(Date updateTime) {
        this.updateTime = updateTime;
    }

    public Integer getOrgType() {
        return orgType;
    }

    public void setOrgType(Integer orgType) {
        this.orgType = orgType;
    }

    public Boolean getDeleteFlag() {
        return deleteFlag;
    }

    public void setDeleteFlag(Boolean deleteFlag) {
        this.deleteFlag = deleteFlag;
    }

    public Long getTenantOrgId() {
        return tenantOrgId;
    }

    public void setTenantOrgId(Long tenantOrgId) {
        this.tenantOrgId = tenantOrgId;
    }

    @Override
    public Serializable pkVal() {
        return this.id;
    }

    @Override
    public String toString() {
        return "SysOrg{" +
                ", id=" + id +
                ", pid=" + pid +
                ", name=" + name +
                ", orgCode=" + orgCode +
                ", fullName=" + fullName +
                ", nodeLevel=" + nodeLevel +
                ", leftNum=" + leftNum +
                ", rightNum=" + rightNum +
                ", note=" + note +
                ", status=" + status +
                ", createTime=" + createTime +
                ", updateTime=" + updateTime +
                ", orgType=" + orgType +
                ", deleteFlag=" + deleteFlag +
                ", tenantOrgId=" + tenantOrgId +
                "}";
    }

    public Long getTenantId() {
        return tenantId;
    }

    public void setTenantId(Long tenantId) {
        this.tenantId = tenantId;
    }
}
