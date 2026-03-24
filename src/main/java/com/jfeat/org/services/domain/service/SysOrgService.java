package com.jfeat.org.services.domain.service;

import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.jfeat.org.config.BaseQueryParam;
import com.jfeat.org.services.domain.model.*;
import com.jfeat.org.services.persistence.model.SysOrg;

import java.util.List;

/**
 * @author 莫昌廉
 */
public interface SysOrgService {

    Long createNewNode(Long opsOrgId, SysOrg entity, Boolean useEntityOrgId);

    Long createNewNode(Long opsOrgId, Long tenantOrgId, SysOrg entity, Boolean useEntityOrgId);

    Long deleteNode(Long opsOrgId, Long targetOrgId);

    Long updateNode(Long opsOrgId, Long targetOrgId, SysOrg entity);

    List<SysOrg> pageLikeNameAndOrgId(Page<SysOrg> page, String name);

    List<SysOrg> listLikeNameAndOrgId(BaseQueryParam p, String search);

    List<SysOrg> getListWithOrgAndSubs(Long orgId);

    List<SysOrgRecord> getListWithOrgAndSubsSysOrgRecord(Long orgId,String search);

    SysOrgChildNodeModel getModelWithOrgAndSubs(Long orgId);

    void orgUniqueValidate(Long pid,String Name,String orgCode);

    SysOrg getById(Long id);

    SysOrg getVisibleOrg(Long ownerOrgId, Long targetOrgId);

    List<Long> getVisibleOrgIds(Long orgId);

    SysOrg getByOrgCode(String orgCode);

    Long extAdd(ExtOrgDTO extOrgDTO);

    Long extDelete(Long id);

    Long extUpdate(ExtOrgDTO extOrgDTO);

    Long sync();

    /**
     * 根据组织id列表查询组织列表
     * @param orgIdList 组织id列表
     * @return
     */
    List<SysOrg> listByOrgIdList(List<Long> orgIdList);

    List<ExtOrgRelationDTO> extList();

    SysOrgExtDTO getSysOrgExt(Long orgId, Long extOrgId);
}
