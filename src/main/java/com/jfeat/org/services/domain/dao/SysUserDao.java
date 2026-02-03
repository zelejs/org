package com.jfeat.org.services.domain.dao;

import java.util.List;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.jfeat.org.services.domain.model.DomainComeFrom;
import com.jfeat.org.services.persistence.model.SysUser;
import org.apache.ibatis.annotations.Param;

/**
 * 管理员的dao
 *
 * @author Admin
 * @date 2017年2月12日 下午8:43:52
 */
public interface SysUserDao extends BaseMapper<SysUser> {


    int updateUserById(@Param("entity")SysUser entity);

    /**
     * 修改用户状态
     * @date 2017年2月12日 下午8:42:31
     */
    int setStatus(@Param("userId") Long userId, @Param("status") int status);

    //删除用户，只改flag 和phone org_id
    int setUserDeleteFlag(@Param("id") Long id, @Param("flag") Integer flag);
    /**
     * 修改密码
     *
     * @param userId
     * @param pwd
     * @date 2017年2月12日 下午8:54:19
     */
    int changePwd(@Param("userId") Long userId, @Param("pwd") String pwd);




    /**
     * 根据条件查询用户列表
     *
     * @return
     * @date 2017年2月12日 下午9:14:34
     */
    List<SysUser> selectUsers(Page<SysUser> page,
                              @Param("roleId") Long roleId,
                              @Param("name") String name,
                              @Param("beginTime") String beginTime,
                              @Param("endTime") String endTime);

    /**
     * 通过账号获取用户
     *
     * @param account
     * @return
     * @date 2017年2月17日 下午11:07:46
     */
    SysUser getByAccount(@Param("account") String account,
                         @Param("orgId") Long orgId);


    /**
     *  通过手机号获取账户
     * */
    SysUser getAccountByPhone(@Param("phone")String phone);

    /**
     *  通过邮箱获取账户
     * */
    SysUser getAccountByEmail(@Param("email")String email);

    /**
     * 通过组织和账号查询当前orgId下用户 仅判断当前orgId不判断orgId之下的其他组织
     *
     */
    SysUser getUserByOrgIdNoTree(@Param("account") String account,
                                 @Param("orgId") Long orgId);

    /**
     * 通过code获取用户
     * @param account
     * @return
     */
    SysUser getByOrgCode(@Param("account") String account,@Param("orgCode")String orgCode);


    /*
    * 检查用户是否存在，存在就返回用户
    * */
    SysUser checkUserExist(@Param("account") String account,@Param("tenantOrgId") Long tenantOrgId);

    /*
     * 检查用户是否存在，存在就返回用户 登录时需检查手机或邮箱是否已验证
     * */
    SysUser checkUserExistLogin(@Param("account") String account,@Param("orgId")Long orgId);

    /**
     * 通过code获取用户 会根据code下的组织树查询用户 可能会有多个
     * @param account
     * @return
     */
    SysUser getByOrgCodeTree(@Param("account") String account,
                         @Param("orgCode") String orgCode);

    //根据域名获取账号
    SysUser getByAccountOne(@Param("account") String account,
                            @Param("domain") String domain);

    /**
     * 通过账号获取用户 (含密码)
     * @param account
     * @return
     **/
    SysUser getByAccountWithPwd(@Param("account") String account);


    /**
     * 通过账号/orgId获取用户 (含密码) orgId 通过 域名获取
     * @param account
     * @return
     **/
    SysUser getByAccountAndOrgIdWithPwd(@Param("account") String account,
                                @Param("orgId") Long orgId);



    /**
     * 通过用户Id获取用户
     *
     * @param userId
     * @return
     * @date 2017年2月17日 下午11:07:46
     */
    SysUser getByUserId(@Param("userId") Long userId);

    /**
     * 通过电话号码获取用户
     * @param phone
     * @return
     */
    SysUser getByPhone(@Param("phone") String phone);

    /**
     * 根据openid查找用户
     * @param openid
     * @return
     */
    SysUser getByOpenid(@Param("openid") String openid);

    Integer selectCount(@Param("roleId") Long roleId);


    SysUser getUnique(@Param("account") String account,
                      @Param("phone") String phone,
                      @Param("email") String email,
                      @Param("domain") String domain);

    Integer updateOrgId(@Param("id") Long id);


    /***
     * 查询对应orgId下 某个account
     * */
    SysUser selectUserByFatherOrgId(@Param("account") String account, @Param("orgId") Long orgId);

    int getUserCount();

    List<SysUser> getTestUserList();

    SysUser loginByOpenidUnionid(@Param("openid")String openid,@Param("unionid")String unionid);

 //根据域名获取组织id
    Long getTenantOrgIdByDomain(@Param("domain")String domain);

    //根据 合约 使用 域名和账户 获取用户 （用户可能在顶部组织会出现多个 顶层组织单独查询）
    SysUser getUserByAppDomain(@Param("domain")String domain,@Param("account")String account);

    //根据域名获取组织id 全部（同时搜索合约）
    DomainComeFrom getTenantOrgIdByDomainAll(@Param("domain")String domain);

    SysUser getUserByTenant(@Param("domain")String domain , @Param("account")String account);

    Long deleteTreeByOrgId(Long orgId);

}
