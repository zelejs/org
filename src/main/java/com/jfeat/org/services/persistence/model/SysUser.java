package com.jfeat.org.services.persistence.model;

import com.baomidou.mybatisplus.extension.activerecord.Model;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import com.baomidou.mybatisplus.annotation.IdType;
import com.fasterxml.jackson.annotation.JsonFormat;
import org.springframework.format.annotation.DateTimeFormat;

import java.io.Serializable;
import java.util.Date;

/**
 * <p>
 *
 * </p>
 *
 * @author admin
 * @since 2018-12-13
 */
@TableName("t_sys_user")
public class SysUser extends Model<SysUser> {

	private static final long serialVersionUID = 1L;


	/**
	 * 主键id
	 */
	@TableId(value = "id", type = IdType.AUTO)
	private Long id;
	/**
	 * 头像
	 */
	private String avatar;

	@TableField(exist = false)
	private String avatarUrl;
	/**
	 * 账号(登录账号)
	 */
	private String account;
	/**
	 * 部门 id
	 */
	@TableField("org_id")
	private Long orgId;
	/**
	 * 第三方授权ID
	 */
	private String openid;
	/**
	 * 密码MD5值
	 */
	private String password;
	/**
	 * 盐值
	 */
	private String salt;
	/**
	 * 昵称
	 */
	private String name;
	/**
	 * 生日
	 */
	@DateTimeFormat(pattern = "yyyy-MM-dd")
	@JsonFormat(locale="zh", timezone="GMT+8", pattern="yyyy-MM-dd")
	private Date birthday;
	/**
	 * 性别
	 */
	private Integer sex;
	/**
	 * 邮箱
	 */
	private String email;
	/**
	 * 邮箱 <result column="email_validated" property="emailValidated" />
	 */
	@TableField("email_validated")
	private Integer emailValidated;
	/**
	 * 电话
	 */
	private String phone;
	/**
	 * 状态
	 */
	private Integer status;
	/**
	 * 创建时间，可理解为注册时间
	 */
	@DateTimeFormat(pattern = "yyyy-MM-dd HH:mm:ss")
	@JsonFormat(locale="zh", timezone="GMT+8", pattern="yyyy-MM-dd HH:mm:ss")
	private Date createtime;
	/**
	 * 版本
	 */
	private Integer version;
	/**
	 * 逻辑删除字段
	 */
	//0-正常 1-已删除
	@TableField("delete_flag")
	private Boolean deleteFlag;
	/**
	 * 需要重置密码
	 */
	@TableField("require_password_reset")
	private Integer requirePasswordReset;

	/**
	 * 用户所属app
	 */
	private String appid;

	@TableField("user_type")
	private Integer userType;

	@TableField("tenant_org_id")
	private Long tenantOrgId;

	@TableField("dev_user_type")
	private Integer devUserType;

	@TableField("registered_phone")
	private Integer registeredPhone;

	@TableField("registered_email")
	private Integer registeredEmail;

	private String unionid;

	public String getUnionid() {
		return unionid;
	}

	public void setUnionid(String unionid) {
		this.unionid = unionid;
	}

	public String getAvatarUrl() {
		return avatarUrl;
	}

	public void setAvatarUrl(String avatarUrl) {
		this.avatarUrl = avatarUrl;
	}

	public Integer getRegisteredPhone() {
		return registeredPhone;
	}

	public void setRegisteredPhone(Integer registeredPhone) {
		this.registeredPhone = registeredPhone;
	}

	public Integer getRegisteredEmail() {
		return registeredEmail;
	}

	public void setRegisteredEmail(Integer registeredEmail) {
		this.registeredEmail = registeredEmail;
	}

	public Integer getDevUserType() {
		return devUserType;
	}

	public void setdevUserType(Integer devUserType) {
		this.devUserType = devUserType;
	}

	public Long getTenantOrgId() {
		return tenantOrgId;
	}

	public void setTenantOrgId(Long tenantOrgId) {
		this.tenantOrgId = tenantOrgId;
	}

	public Integer getUserType() {
		return userType;
	}

	public void setUserType(Integer userType) {
		this.userType = userType;
	}

	public Long getId() {
		return id;
	}

	public void setId(Long id) {
		this.id = id;
	}

	public String getAvatar() {
		return avatar;
	}

	public void setAvatar(String avatar) {
		this.avatar = avatar;
	}

	public String getAccount() {
		return account;
	}

	public void setAccount(String account) {
		this.account = account;
	}

	public Long getOrgId() {
		return orgId;
	}

	public void setOrgId(Long orgId) {
		this.orgId = orgId;
	}

	public String getOpenid() {
		return openid;
	}

	public void setOpenid(String openid) {
		this.openid = openid;
	}

	public String getPassword() {
		return password;
	}

	public void setPassword(String password) {
		this.password = password;
	}

	public String getSalt() {
		return salt;
	}

	public void setSalt(String salt) {
		this.salt = salt;
	}

	public String getName() {
		return name;
	}

	public void setName(String name) {
		this.name = name;
	}

	public Date getBirthday() {
		return birthday;
	}

	public void setBirthday(Date birthday) {
		this.birthday = birthday;
	}

	public Integer getSex() {
		return sex;
	}

	public void setSex(Integer sex) {
		this.sex = sex;
	}

	public String getEmail() {
		return email;
	}

	public void setEmail(String email) {
		this.email = email;
	}

	public Integer getEmailValidated() {
		return emailValidated;
	}

	public void setEmailValidated(Integer emailValidated) {
		this.emailValidated = emailValidated;
	}

	public String getPhone() {
		return phone;
	}

	public void setPhone(String phone) {
		this.phone = phone;
	}

	public Integer getStatus() {
		return status;
	}

	public void setStatus(Integer status) {
		this.status = status;
	}

	public Date getCreatetime() {
		return createtime;
	}

	public void setCreatetime(Date createtime) {
		this.createtime = createtime;
	}

	public Integer getVersion() {
		return version;
	}

	public void setVersion(Integer version) {
		this.version = version;
	}

	public Boolean getDeleteFlag() {
		return deleteFlag;
	}

	public void setDeleteFlag(Boolean deleteFlag) {
		this.deleteFlag = deleteFlag;
	}

	public Integer getRequirePasswordReset() {
		return requirePasswordReset;
	}

	public void setRequirePasswordReset(Integer requirePasswordReset) {
		this.requirePasswordReset = requirePasswordReset;
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
		return "SysUser{" +
				", id=" + id +
				", avatar=" + avatar +
				", account=" + account +
				", orgId=" + orgId +
				", openid=" + openid +
				", password=" + password +
				", salt=" + salt +
				", name=" + name +
				", birthday=" + birthday +
				", sex=" + sex +
				", email=" + email +
				", emailValidated=" + emailValidated +
				", phone=" + phone +
				", status=" + status +
				", createtime=" + createtime +
				", version=" + version +
				", deleteFlag=" + deleteFlag +
				", requirePasswordReset=" + requirePasswordReset +
				"}";
	}
}
