package com.jfeat.org.services.persistence.dao;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.jfeat.org.config.BaseQueryParam;
import com.jfeat.org.services.persistence.model.SysOrg;
import org.apache.ibatis.annotations.Param;
import org.apache.ibatis.annotations.Select;

import java.util.List;

/**
 * <p>
 *  Mapper 接口
 * </p>
 *
 * @author admin
 * @since 2018-10-26
 */
public interface SysOrgMapper extends BaseMapper<SysOrg> {
    SysOrg findById(Long id);

    SysOrg visibleById(@Param("_base") BaseQueryParam p, @Param("id")Long id);

    List<SysOrg> pageLikeNameAndOrgId(@Param("_base") BaseQueryParam p, Page<SysOrg> page,  @Param("name") String name);

    List<SysOrg> listLikeNameAndOrgId(@Param("_base") BaseQueryParam p,  @Param("search") String search);

    @Select("select \n" +
            "t_sys_org.id\n" +
            "from t_sys_org \n" +
            "ORDER BY  LEFT(t_sys_org.id,#{length}) desc\n" +
            "LIMIT 0,1")
    Long getMaxOrgId(@Param("length")Integer length);

    @Select("select \n" +
            "t_sys_org.id\n" +
            "from t_sys_org \n" +
            "where LEFT(t_sys_org.id,#{length})=LEFT(#{id},#{length}) "+
            "ORDER BY  t_sys_org.id desc\n" +
            "LIMIT 0,1")
    Long getMaxOrgIdById(@Param("id")Long id, @Param("length")Integer length);

    SysOrg findOrgByCode(@Param("code") String code);

}
