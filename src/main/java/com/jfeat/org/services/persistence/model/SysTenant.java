package com.jfeat.org.services.persistence.model;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.extension.activerecord.Model;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import java.util.List;

/**
 * <p>
 *
 * </p>
 *
 * @author Code Generator
 * @since 2020-03-24
 */
@TableName("t_sys_tenant")
public class SysTenant extends Model<SysTenant> {

    private static final long serialVersionUID = 1L;

	public static final String ORG_ID = "org_id";

	@TableId(value="id", type= IdType.AUTO)
	private Long id;

	/**
	 * 租户名
	 */
	private String name;

	/**
	 * orgId
	 */
	@TableField("org_id")
	private Long orgId;

	/**
	 * 域名
	 */
	private String domain;

	/**
	 * 0 -> 停用, 1-> 启动
	 */
	private Integer status;

	/**
	 * 上次启动的时间
	 */
	@TableField("start_time")
	private Date startTime;

	/**
	 * 租户标志
	 */
	private String logo;

	/**
	 * 租户标题
	 */
	private String title;

	private String contractId;
	private Long endUserId;
	private String phone;

	/**
	 * 应用ID
	 */
	private String appid;

	private Boolean hasChildren;
	private Boolean locked;

	public Boolean getLocked() {
		return locked;
	}

	public void setLocked(Boolean locked) {
		this.locked = locked;
	}

	//0-正常 1-已删除
	@TableField("delete_flag")
	private Boolean deleteFlag;

	@TableField(exist = false)
	private List<Long> permIds;

	public List<Long> getPermIds() {
		return permIds;
	}

	public void setPermIds(List<Long> permIds) {
		this.permIds = permIds;
	}

	public String getLogo() {
		return logo;
	}

	public void setLogo(String logo) {
		this.logo = logo;
	}

	public String getTitle() {
		return title;
	}

	public void setTitle(String title) {
		this.title = title;
	}

	public Long getId() {
		return id;
	}

	public SysTenant setId(Long id) {
		this.id = id;
		return this;
	}

	public String getName() {
		return name;
	}

	public SysTenant setName(String name) {
		this.name = name;
		return this;
	}

	public Long getOrgId() {
		return orgId;
	}

	public SysTenant setOrgId(Long orgId) {
		this.orgId = orgId;
		return this;
	}

	public String getDomain() {
		return domain;
	}

	public SysTenant setDomain(String domain) {
		this.domain = domain;
		return this;
	}

	public Integer getStatus() {
		return status;
	}

	public SysTenant setStatus(Integer status) {
		this.status = status;
		return this;
	}

	public Date getStartTime() {
		return startTime;
	}

	public SysTenant setStartTime(Date startTime) {
		this.startTime = startTime;
		return this;
	}

	public Boolean getDeleteFlag() {
		return deleteFlag;
	}

	public void setDeleteFlag(Boolean deleteFlag) {
		this.deleteFlag = deleteFlag;
	}

	public String getPhone() {
		return phone;
	}

	public void setPhone(String phone) {
		this.phone = phone;
	}

	public String getAppid() {
		return appid;
	}

	public void setAppid(String appid) {
		this.appid = appid;
	}

	@Override
	public Serializable pkVal() {
		return this.id;
	}

	@Override
	public String toString() {
		return "Tenant{" +
			"id=" + id +
			", name=" + name +
			", orgId=" + orgId +
			", domain=" + domain +
			", status=" + status +
			", startTime=" + startTime +
			", deleteFlag=" + deleteFlag +
			"}";
	}
}