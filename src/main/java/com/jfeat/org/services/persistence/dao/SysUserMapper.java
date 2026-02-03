package com.jfeat.org.services.persistence.dao;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.jfeat.org.config.BaseQueryParam;
import com.jfeat.org.services.persistence.model.SysUser;
import org.apache.ibatis.annotations.Param;
import org.apache.ibatis.annotations.Update;

import java.util.List;

/**
 * <p>
  *  Mapper 接口
 * </p>
 *
 * @author admin
 * @since 2018-01-20
 */
public interface SysUserMapper extends BaseMapper<SysUser> {
    SysUser findById(@Param("id")Long id);

    SysUser visibleById(@Param("_base") BaseQueryParam p, @Param("id")Long id);

    List<SysUser> page(@Param("_base") BaseQueryParam p, Page<SysUser> page, @Param("selectOrgId") Long selectOrgId, @Param("roleId") Long roleId, @Param("name") String name, @Param("account") String account);

    List<SysUser> listTreeByAccount(@Param("_base") BaseQueryParam p, @Param("account") String account);

    Integer deleteTreaByOrgId(@Param("orgId") Long orgId);

    Integer getAdminPassowrdStatus(@Param("orgId") Long orgId);
    //更新是否需要重置密码的状态 更新为是
    Integer updatePasswordStatus(@Param("orgId") Long orgId,@Param("userId") Long userId);

    //根据用户id获取租户id
    Long getTenantOrgId(@Param("userId")Long userId);

    //更新某个用户的租户id
    @Update("update t_sys_user set tenant_org_id = #{tenantOrgId} where id = #{userId}")
    Integer updateTenantOrgId(@Param("userId")Long userId,@Param("tenantOrgId")Long tenantOrgId);


    //@when 2025-12 update below, add appid to filter user by appid

    // 直接通过租户ID和账号获取用户信息
    // SysUser findByTenantOrgIdAndAccount(@Param("tenantOrgId")Long tenantOrgId, @Param("account")String account);
    SysUser findByTenantOrgIdAndAccount(@Param("tenantOrgId")Long tenantOrgId, @Param("account")String account, @Param("appid") String appid);

    //根据租户id和账号获取用户信息, 还需要过滤appid
    // SysUser findByAccountInTenant(@Param("accountOrgId") Long accountOrgId, @Param("account") String account);
    SysUser findByAccountInTenant(@Param("accountOrgId") Long accountOrgId, @Param("account") String account, @Param("appid") String appid);
}