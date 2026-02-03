package com.jfeat.org.services.domain.dao;

import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.jfeat.org.services.domain.model.SysOrgRecord;
import com.jfeat.org.services.persistence.model.SysOrg;
import com.jfeat.org.services.persistence.model.SysUser;
import org.apache.ibatis.annotations.Param;

import java.util.List;

public interface UaasOrgDao {

    Integer updateLeftValueWhileCreateNode(@Param("pid") Long pid);

    Integer updateRightValueWhileCreateNode(@Param("pid") Long pid);

    Integer deleteByIds(@Param("ids") List<Long> ids);

    Integer updateLeftValueWhileDeleteNode(@Param("leftNum") int leftNum, @Param("rightNum") int rightNum);

    Integer updateRightValueWhileDeleteNode(@Param("leftNum") int leftNum, @Param("rightNum") int rightNum);

    // 组织调整架构
    Integer updateChildrenAndSelfNode(@Param("leftNum") int leftNum, @Param("rightNum") int rightNum);

    Integer updateLeftValueWhileUpdateNodeDel(@Param("leftNum") int leftNum, @Param("rightNum") int rightNum);

    Integer updateRightValueWhileUpdateNodeDel(@Param("leftNum") int leftNum, @Param("rightNum") int rightNum);

    Integer updateRightValueWhileUpdateNodeAdd(@Param("pRightNum") int pRightNum, @Param("leftNum") int leftNum, @Param("rightNum") int rightNum);

    Integer updateLeftValueWhileUpdateNodeAdd(@Param("pRightNum") int pRightNum, @Param("leftNum") int leftNum, @Param("rightNum") int rightNum);

    Integer updateChildrenAndSelfNodeRe(@Param("leftNum") int leftNum, @Param("pRightNum") int pRightNum);

    Integer updateNodeLevelWhileUpdateNode(@Param("pNodeLevel") int pNodeLevel, @Param("nodeLevel") int nodeLevel);

    List<SysOrg> getAllDescendant(@Param("id") Long id);

    List<SysOrgRecord> searchAllOrgRecord(@Param("search")String search, @Param("bType")String bType);

    List<SysOrgRecord> searchDescendantOrgRecord(@Param("orgId") Long orgId, @Param("search") String search, @Param("bType")String bType);

    List<SysOrg> getRootOrg();

    List<SysUser> orgManagers(@Param("orgId") Long orgId);

    List<SysOrg> myOrgList(@Param("userId") Long userId);


}
