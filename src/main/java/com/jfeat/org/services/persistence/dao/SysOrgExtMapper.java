package com.jfeat.org.services.persistence.dao;
import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.jfeat.org.services.persistence.model.SysOrgExt;
import org.apache.ibatis.annotations.Param;

/**
 * @author : lvsongxin
 * @date :
 */
public interface SysOrgExtMapper extends BaseMapper<SysOrgExt>  {
    /**
     * 根据 ID 查询组织扩展信息
     *
     * @param id 组织扩展信息的 ID
     * @return 组织扩展信息对象
     */
    SysOrgExt findById(@Param("id") Long id);
    /**
     * 根据党组织 ID 查询组织扩展信息
     *
     * @param partyOrgId 党组织 ID
     * @return 组织扩展信息对象
     */
    SysOrgExt findByPartyId(@Param("partyOrgId") Long partyOrgId);
}
