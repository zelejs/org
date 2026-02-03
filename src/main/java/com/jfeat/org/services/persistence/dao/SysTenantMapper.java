package com.jfeat.org.services.persistence.dao;

import com.jfeat.org.services.persistence.model.SysTenant;

import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import org.apache.ibatis.annotations.Param;

import java.util.List;

/**
 * <p>
  *  Mapper 接口
 * </p>
 *
 * @author Code Generator
 * @since 2020-03-24
 */
public interface SysTenantMapper extends BaseMapper<SysTenant> {
    SysTenant findById(Long id);

    SysTenant findByOrgId(@Param("orgId") Long orgId);

    SysTenant findByDomain(@Param("domain") String domain);

    List<SysTenant> page(Page<SysTenant> page, @Param("tenant") SysTenant tenant, @Param("search") String search);

}