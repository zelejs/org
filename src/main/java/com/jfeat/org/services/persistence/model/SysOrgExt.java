package com.jfeat.org.services.persistence.model;

import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import com.baomidou.mybatisplus.extension.activerecord.Model;

import java.io.Serializable;
import java.util.Date;

/**
 * @author : lvsongxin
 * @date :
 */
@TableName("t_sys_org_ext")
public class SysOrgExt extends Model<SysOrgExt> {
    @TableId(value = "id")
    private Long id;

    @TableField("ext_org_id")
    private Long extOrgId;

    @TableField("create_time")
    private Date createTime;

    @TableField("update_time")
    private Date updateTime;

    @TableField("delete_flag")
    private Integer deleteFlag;

    @TableField("ext_org_type")
    private Integer extOrgType;

    @Override
    public Serializable pkVal() {
        return this.id;
    }

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public Long getExtOrgId() {
        return extOrgId;
    }

    public void setExtOrgId(Long extOrgId) {
        this.extOrgId = extOrgId;
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

    public Integer getDeleteFlag() {
        return deleteFlag;
    }

    public void setDeleteFlag(Integer deleteFlag) {
        this.deleteFlag = deleteFlag;
    }

    public Integer getExtOrgType() {
        return extOrgType;
    }

    public void setExtOrgType(Integer extOrgType) {
        this.extOrgType = extOrgType;
    }
}

