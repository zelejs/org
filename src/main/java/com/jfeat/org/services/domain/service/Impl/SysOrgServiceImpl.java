package com.jfeat.org.services.domain.service.Impl;

import com.alibaba.fastjson2.JSON;
import com.alibaba.fastjson2.JSONArray;
import com.alibaba.fastjson2.JSONObject;
import com.alibaba.fastjson2.JSONWriter;
import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.jfeat.am.core.jwt.JWTKit;
import com.xinzhi.plat.common.exception.BusinessException;
import com.jfeat.org.config.BaseQueryParam;
import com.jfeat.org.constant.CommonConstants;
import com.jfeat.org.constant.OrganizationTypeConstants;
import com.jfeat.org.services.domain.dao.UaasOrgDao;
import com.jfeat.org.services.domain.definition.Platform;
import com.jfeat.org.services.domain.model.*;
import com.jfeat.org.services.domain.service.SysOrgService;
import com.jfeat.org.services.persistence.dao.SysOrgExtMapper;
import com.jfeat.org.services.persistence.dao.SysOrgMapper;
import com.jfeat.org.services.persistence.model.SysOrg;
import com.jfeat.org.services.persistence.model.SysOrgExt;
import com.jfeat.org.utills.CodeGenerator;
import org.slf4j.Logger;

import java.security.SecureRandom;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import jakarta.annotation.Resource;
import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.*;
import java.util.stream.Collectors;


@Service("SysOrgService")
public class SysOrgServiceImpl implements SysOrgService {
    protected static Logger logger = LoggerFactory.getLogger(SysOrgServiceImpl.class);


    @Resource
    UaasOrgDao uaasOrgDao;

    @Resource
    SysOrgMapper sysOrgMapper;

    @Resource
    SysOrgExtMapper sysOrgExtMapper;


    private String genAndCheckOrgCode(Long orgId, String orgCode) {
        if(orgCode != null) {
            // 检查重复
            SysOrg sysOrg = sysOrgMapper.findOrgByCode(orgCode);
            if((sysOrg != null && orgId == null)) {
                throw new BusinessException(-1, "组织代码已经存在，不可重复");
            }
            if((sysOrg != null && orgId != null && !sysOrg.getId().equals(orgId))) {
                throw new BusinessException(-1, "组织代码已经存在，不可重复");
            }
            return orgCode;
        } else {
            // 生成代码
            for (int i = 0; i < 100; i++) {
                String newCode = generateRandomBase62(8);
                SysOrg sysOrg = sysOrgMapper.findOrgByCode(newCode);
                if(sysOrg == null) {
                    return newCode;
                }
            }
            throw new BusinessException(-1001, "自动生成组织代码重复，再尝试！");
        }
    }

    private static final String BASE62_CHARS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    private static final SecureRandom RANDOM = new SecureRandom();

    private String generateRandomBase62(int length) {
        StringBuilder sb = new StringBuilder(length);
        for (int i = 0; i < length; i++) {
            sb.append(BASE62_CHARS.charAt(RANDOM.nextInt(BASE62_CHARS.length())));
        }
        return sb.toString();
    }

    @Override
    @Transactional
    public Long createNewNode(Long opsOrgId, SysOrg entity, Boolean useEntityOrgId) {
        return createNewNode(JWTKit.getOrgId(),0L, entity,useEntityOrgId);
    }

    /**
     * 新建节点
     * @param opsOrgId
     * @param entity
     * @param useEntityOrgId
     * @return
     */
    @Override
    @Transactional
    public Long createNewNode(Long opsOrgId, Long tenantOrgId, SysOrg entity, Boolean useEntityOrgId) {
        entity.setOrgCode(genAndCheckOrgCode(null,entity.getOrgCode()));
        if (entity.getPid() == null) {
            throw new BusinessException(-1, "必须指定上级组织");
        }
        SysOrg parentOrg = getVisibleOrg(opsOrgId, entity.getPid());
        if(parentOrg == null) {
            throw new BusinessException(-1, "上级组织不存在，请核准并重新提交");
        }

        uaasOrgDao.updateLeftValueWhileCreateNode(entity.getPid());
        uaasOrgDao.updateRightValueWhileCreateNode(entity.getPid());
        entity.setTenantOrgId(tenantOrgId);
        entity.setNodeLevel(parentOrg.getNodeLevel() + 1);
        entity.setLeftNum(parentOrg.getRightNum());
        entity.setRightNum(parentOrg.getRightNum() + 1);
        entity.setAppid(parentOrg.getAppid());
        entity.setUpdateTime(new Date());
        entity.setCreateTime(new Date());
        sysOrgMapper.insert(entity);
        return entity.getId();
    }

    /**
     * 删除节点
     * @param opsOrgId
     * @param targetOrgId
     * @return
     */
    @Override
    @Transactional
    public Long deleteNode(Long opsOrgId, Long targetOrgId) {
        SysOrg sysOrg = getVisibleOrg(opsOrgId, targetOrgId);
        if (sysOrg == null) {
            throw new BusinessException(-1, "指定部门不存在，请核准并重新提交");
        }

        if(sysOrg.getOrgType() <= OrganizationTypeConstants.TENANT) {
            throw new BusinessException(-1, "平台跟租户类型的组织不可删除");
        }

        if(targetOrgId.equals(CommonConstants.SYS_ORG_ID)) {
            throw new BusinessException(-1, "顶层组织不能删除");
        }

        // TODO
        // 组织绑定了用户
        // 组织有子节点
        List<SysOrg> sysOrgs = sysOrgMapper.listLikeNameAndOrgId(BaseQueryParam.createChildFilter(sysOrg.getTenantOrgId(), sysOrg.getId()), null);
        if(sysOrgs.size() > 1) {
            throw new BusinessException(-1, "有子组织，不可删除");
        }

        List<Long> chileIds = sysOrgs.stream().map(SysOrg::getId).filter(id -> !id.equals(targetOrgId)).
                collect(Collectors.toList());
        if(!chileIds.isEmpty()) {
            uaasOrgDao.deleteByIds(chileIds);
        }

        uaasOrgDao.updateLeftValueWhileDeleteNode(sysOrg.getLeftNum(), sysOrg.getRightNum());
        uaasOrgDao.updateRightValueWhileDeleteNode(sysOrg.getLeftNum(), sysOrg.getRightNum());
        sysOrg.setDeleteFlag(true);
        sysOrgMapper.updateById(sysOrg);

        sysOrgExtMapper.deleteById(sysOrg.getId());
        return targetOrgId;
    }

    /**
     * 新部门信息 限制更新部门信息时，不能修改 层级数/父节点ID/左右值 若提供的信息不匹配 需要跑出异常
     * @param opsOrgId
     * @param targetOrgId
     * @param entity
     * @return
     */
    @Override
    @Transactional
    public Long updateNode(Long opsOrgId, Long targetOrgId, SysOrg entity) {
        SysOrg sysOrg = getVisibleOrg(opsOrgId, targetOrgId);
        if (sysOrg == null) {
            throw new BusinessException(-1, "指定部门不存在，请核准并重新提交");
        }
        if(sysOrg.getOrgType() <= OrganizationTypeConstants.TENANT) {
            if(!sysOrg.getOrgType().equals(entity.getOrgType())) {
                throw new BusinessException(-1, "平台跟租户类型的组织不能修改类型");
            }
        }
        if(entity.getOrgType() <= OrganizationTypeConstants.TENANT) {
            if(!sysOrg.getOrgType().equals(entity.getOrgType())) {
                throw new BusinessException(-1, "平台跟租户类型的组织不能修改类型");
            }
        }
        // 不支持调整组织架构
        if ((entity.getPid() != null && !Objects.equals(entity.getPid(), sysOrg.getPid())) ||
                (entity.getNodeLevel() != null && !Objects.equals(entity.getNodeLevel(), sysOrg.getNodeLevel()))
                || (entity.getLeftNum() != null && !Objects.equals(entity.getLeftNum(), sysOrg.getLeftNum()))
                || (entity.getRightNum() != null && !Objects.equals(entity.getRightNum(), sysOrg.getRightNum()))) {
            throw new BusinessException(-1, "非法数据:\"不能执行修改组织部门 父组织|层级数|左右值 信息操作\"");
        }

        if (entity.getPid()!=null&&entity.getPid().equals(targetOrgId)) {
            throw new BusinessException(-1, "非法数据:\"无法将自己设置为自身的父类部门\"");
        }

        String orgCode = sysOrg.getOrgCode();
        if(entity.getOrgCode() != null) {
            orgCode = genAndCheckOrgCode(sysOrg.getId(), entity.getOrgCode());
        }

        // 不支持调整组织架构 逻辑不会进入
        if((sysOrg.getPid()!=null&&entity.getPid() != null && (!entity.getPid().equals(sysOrg.getPid()) ))){
            //处理自身和子节点的数值，防止出现相同左右节点数值的节点
            uaasOrgDao.updateChildrenAndSelfNode(sysOrg.getLeftNum(),sysOrg.getRightNum());
            //处理左右两侧的坐标，将要修改的对象先从结构中取出
            uaasOrgDao.updateLeftValueWhileUpdateNodeDel(sysOrg.getLeftNum(), sysOrg.getRightNum());
            uaasOrgDao.updateRightValueWhileUpdateNodeDel(sysOrg.getLeftNum(), sysOrg.getRightNum());

            SysOrg pSysOrg = sysOrgMapper.selectById(entity.getPid());
            if (pSysOrg == null) {
                throw new BusinessException(-1, "指定父部门不存在，请核准并重新提交");
                //如果pid为空，则不存在循环引用的问题，跳过。   此处判断是否循环引用，如：20部门的pid是21部门，21部门的pid是20部门
            }else if(pSysOrg.getPid()!=null&&pSysOrg.getPid().equals(entity.getId())){
                throw new BusinessException(-1, "请不要循环引用");
            }
            //目标层与自身相差超过1 进行处理
            if(sysOrg.getNodeLevel()-pSysOrg.getNodeLevel()!=1){
                uaasOrgDao.updateNodeLevelWhileUpdateNode(pSysOrg.getNodeLevel(),sysOrg.getNodeLevel());
            }
            //重新插入处理
            uaasOrgDao.updateRightValueWhileUpdateNodeAdd(pSysOrg.getRightNum(),sysOrg.getLeftNum(), sysOrg.getRightNum());
            uaasOrgDao.updateLeftValueWhileUpdateNodeAdd(pSysOrg.getRightNum(),sysOrg.getLeftNum(), sysOrg.getRightNum());
            uaasOrgDao.updateChildrenAndSelfNodeRe(sysOrg.getLeftNum(),pSysOrg.getRightNum());
            entity.setId(targetOrgId);
            sysOrgMapper.updateById(entity);
        }else {
            sysOrg.setFullName(entity.getFullName());
            sysOrg.setName(entity.getName());
            sysOrg.setNote(entity.getNote());
            sysOrg.setOrgType(entity.getOrgType());
            sysOrg.setOrgCode(orgCode);
           sysOrgMapper.updateById(sysOrg);
        }
        return sysOrg.getId();
    }

    /**
     * 分页
     * @param page
     * @param name
     * @return
     */
    @Override
    public List<SysOrg> pageLikeNameAndOrgId(Page<SysOrg> page, String name) {
        return sysOrgMapper.pageLikeNameAndOrgId(BaseQueryParam.create(),page,name);
    }

    /**
     * 列表
     * @param search
     * @return
     */
    @Override
    public List<SysOrg> listLikeNameAndOrgId(BaseQueryParam p, String search) {
        return sysOrgMapper.listLikeNameAndOrgId(p, search);
    }

    /**
     * 返回org节点与其子孙节点
     * @param orgId
     * @return
     */
    @Override
    public List<SysOrg> getListWithOrgAndSubs(Long orgId) {
        SysOrg sysOrg = sysOrgMapper.selectById(orgId);
        if (sysOrg == null) {
            throw new BusinessException(-1, "指定部门不存在，请核准并重新提交");
        }
        List<SysOrg> result = new ArrayList<>();
        result.add(sysOrg);
        result.addAll(uaasOrgDao.getAllDescendant(orgId));
        return result;
    }

    /**
     * 返回org节点与其子孙节点 包含部门负责人信息
     * @param orgId
     * @return
     **/
    @Override
    public List<SysOrgRecord> getListWithOrgAndSubsSysOrgRecord(Long orgId, String search) {
        SysOrg sysOrg = sysOrgMapper.selectById(orgId);
        if (sysOrg == null) {
            throw new BusinessException(-1, "指定部门不存在，请核准并重新提交");
        }
        // 平台组织
        if(Platform.ID.getValue().equals(orgId)){
            return uaasOrgDao.searchAllOrgRecord(search, sysOrg.getAppid());
        }else {
            return uaasOrgDao.searchDescendantOrgRecord(orgId, search, sysOrg.getAppid());
        }
    }

    /**
     * 返回org节点与其子孙节点
     * @param orgId
     * @return
     */
    @Override
    public SysOrgChildNodeModel getModelWithOrgAndSubs(Long orgId) {
        SysOrg sysOrg = sysOrgMapper.selectById(orgId);
        if (sysOrg == null) {
            throw new BusinessException(-1, "指定部门不存在，请核准并重新提交");
        }
        SysOrgChildNodeModel result = JSON.parseObject(JSON.toJSONString(sysOrg), SysOrgChildNodeModel.class);
        result.setChildNode(uaasOrgDao.getAllDescendant(orgId));
        return result;
    }

    /**
     * 校验组织数据字段唯一性
     * @param pid
     * @param name
     * @param orgCode
     */
    @Override
    public void orgUniqueValidate(Long pid, String name, String orgCode) {
        SysOrg sysOrgNameUnique = sysOrgMapper.selectOne(new QueryWrapper<SysOrg>()
                .eq("pid",pid)
                .eq("name",name));
        if(sysOrgNameUnique!=null){
            throw new BusinessException(-1002, "已存在该名字的组织");
        }
    }

    /**
     * 获取组织
     * @param id
     * @return
     */
    @Override
    public SysOrg getById(Long id) {
        if (id == null) {
            return null;
        }
        return sysOrgMapper.findById(id);
    }

    /**
     * 获取当前用户 可见的org
     * @param ownerOrgId
     * @param targetOrgId
     * @return
     */
    @Override
    public SysOrg getVisibleOrg(Long ownerOrgId, Long targetOrgId) {
        List<SysOrg> listOrgs = listLikeNameAndOrgId(BaseQueryParam.createChildFilter(),null);
        SysOrg result = listOrgs.stream().filter(item -> targetOrgId.equals(item.getId())).findAny().orElse(null);
        return result;
    }

    /**
     * 返回对于某个org,可见的org id列表 (当前org及子孙)
     * @param orgId
     * @return
     **/
    @Override
    public List<Long> getVisibleOrgIds(Long orgId) {
        List<Long> result = new ArrayList<>();
        SysOrgChildNodeModel model = getModelWithOrgAndSubs(orgId);
        result.add(model.getId());
        model.getChildNode().forEach(item -> result.add(item.getId()));
        return result;
    }

    @Override
    public SysOrg getByOrgCode(String orgCode) {
        return sysOrgMapper.findOrgByCode(orgCode);
    }

    private void checkZeroOrgId(ExtOrgDTO extOrgDTO) {
        if(extOrgDTO.getId() == null || extOrgDTO.getId().equals(0L)) {
            throw new BusinessException(-1, "扩展组织id不能为空");
        }
        if(extOrgDTO.getParentId() == null || extOrgDTO.getParentId().equals(0L)) {
            throw new BusinessException(-1, "扩展父组织id不能为空");
        }
    }
    @Override
    @Transactional
    public Long extAdd(ExtOrgDTO extOrgDTO) {
        logger.info("extAdd extOrgDTO:" + JSONObject.toJSONString(extOrgDTO));
        checkZeroOrgId(extOrgDTO);

        // 通过扩展父组织id获取组织
        SysOrgExt sysOrgExtParent = sysOrgExtMapper.findByExtOrgId(extOrgDTO.getParentId());
        if(sysOrgExtParent == null) {
            throw new BusinessException(-1, "父扩展组织不存在");
        }
        // 判断组织是否存在
        SysOrgExt sysOrgExt = sysOrgExtMapper.findByExtOrgId(extOrgDTO.getId());
        if(sysOrgExt != null) {
            throw new BusinessException(-1, "扩展组织已存在，不可重复创建");
        }

        SysOrg sysOrgParent = sysOrgMapper.findById(sysOrgExtParent.getId());
        if(sysOrgParent == null) {
            throw new BusinessException(-1, "父组织不存在");
        }

        SysOrg sysOrg = new SysOrg();
        sysOrg.setPid(sysOrgExtParent.getId());
        sysOrg.setName(extOrgDTO.getShortName());
        sysOrg.setFullName(extOrgDTO.getName());
        sysOrg.setOrgCode(extOrgDTO.getOrgNum());
        sysOrg.setOrgType(OrganizationTypeConstants.COMPANY);
        sysOrg.setTenantOrgId(sysOrgParent.getTenantOrgId());
        sysOrg.setTenantId(sysOrgParent.getTenantId());
        Long newOrgId = createNewNode(sysOrg.getPid(),sysOrg.getTenantId(), sysOrg, true);

        SysOrgExt newSysOrgExt = new SysOrgExt();
        newSysOrgExt.setId(newOrgId);
        newSysOrgExt.setExtOrgId(extOrgDTO.getId());
        newSysOrgExt.setExtOrgType(extOrgDTO.getType());
        sysOrgExtMapper.insert(newSysOrgExt);

       return newOrgId;
    }

    @Override
    @Transactional
    public Long extDelete(Long id) {
        SysOrgExt sysOrgExt = sysOrgExtMapper.findByExtOrgId(id);
        if(sysOrgExt == null) {
            throw new BusinessException(-1, "扩展组织不存在");
        }

        SysOrg sysOrgParent = sysOrgMapper.findById(sysOrgExt.getId());
        if(sysOrgParent == null) {
            throw new BusinessException(-1, "组织不存在");
        }

        sysOrgExtMapper.deleteById(sysOrgExt);
        return deleteNode(sysOrgParent.getPid(), sysOrgParent.getId());
    }

    @Override
    @Transactional
    public Long extUpdate(ExtOrgDTO extOrgDTO) {
        logger.info("extUpdate extOrgDTO:" + JSONObject.toJSONString(extOrgDTO));

        checkZeroOrgId(extOrgDTO);
        // 判断组织是否存在
        SysOrgExt sysOrgExt = sysOrgExtMapper.findByExtOrgId(extOrgDTO.getId());
        if(sysOrgExt == null) {
            throw new BusinessException(-1, "扩展组织不存在");
        }

        SysOrg sysOrg = sysOrgMapper.findById(sysOrgExt.getId());
        if(sysOrg == null) {
            throw new BusinessException(-1, "组织不存在");
        }

        sysOrgExt.setExtOrgType(extOrgDTO.getType());
        sysOrgExtMapper.updateById(sysOrgExt);

        sysOrg.setName(extOrgDTO.getShortName());
        sysOrg.setFullName(extOrgDTO.getName());
        sysOrg.setOrgCode(extOrgDTO.getOrgNum());
        return updateNode(sysOrg.getPid(),sysOrg.getId(), sysOrg);
    }

    private JSONArray parseJsonData() {
        try (BufferedReader reader = new BufferedReader(new FileReader("D:\\hanpeng\\uaas\\org\\src\\main\\resources\\partyorg.txt"))) {
            StringBuilder jsonContent = new StringBuilder();
            String line;
            while ((line = reader.readLine()) != null) {
                jsonContent.append(line);
            }

            JSONObject jsonObject = JSON.parseObject(jsonContent.toString());
            JSONArray dataArray = jsonObject.getJSONArray("data");
            return dataArray;
        } catch (IOException e) {
            e.printStackTrace();
           return null;
        }
    }


    private void syncList(JSONArray dataArray) {
        if (dataArray != null) {
            for (int i = 0; i < dataArray.size(); i++) {
                JSONObject orgObject = dataArray.getJSONObject(i);
                ExtOrgDTO extOrgDTO = new ExtOrgDTO();
                extOrgDTO.setId(orgObject.getLong("id"));
                extOrgDTO.setParentId(orgObject.getLong("parentId"));
                extOrgDTO.setName(orgObject.getString("name"));
                extOrgDTO.setShortName(orgObject.getString("shortName"));
                extOrgDTO.setOrgNum(orgObject.getString("orgNum"));
//                extOrgDTO.setType(orgObject.getInteger("type"));
                // 检查组织是否已存在
                SysOrgExt existingOrg = sysOrgExtMapper.findByExtOrgId(extOrgDTO.getId());
                if (existingOrg != null) {
                    logger.info("组织已存在，执行更新操作 {}", JSON.toJSONString(extOrgDTO));
                    // 如果存在，调用更新方法
                    extUpdate(extOrgDTO);
                } else {
                    // 如果不存在，调用添加方法
                    SysOrgExt parant = sysOrgExtMapper.findByExtOrgId(extOrgDTO.getParentId());
                    if(parant != null) {
                        logger.info("组织不存在，执行新增操作 {}", JSON.toJSONString(extOrgDTO));
                        extAdd(extOrgDTO);
                    }

                }

                JSONArray childes = orgObject.getJSONArray("childes");
                if(childes != null) {
                    syncList(childes);
                }
            }
        }
    }

    @Override
    @Transactional
    public Long sync() {
        JSONArray dataArray = parseJsonData();
        syncList(dataArray);
        return 0L;
    }

    /**
     * 根据组织id列表查询组织列表
     *
     * @param orgIdList 组织id列表
     */
    @Override
    public List<SysOrg> listByOrgIdList(List<Long> orgIdList) {
        if (orgIdList == null || orgIdList.isEmpty()) {
            return List.of();
        }
        LambdaQueryWrapper<SysOrg> sysOrgLambdaQueryWrapper = new LambdaQueryWrapper<>();
        sysOrgLambdaQueryWrapper.in(SysOrg::getId, orgIdList);
        return sysOrgMapper.selectList(sysOrgLambdaQueryWrapper);
    }

    @Override
    public List<ExtOrgRelationDTO> extList() {
        // 调整组织
        LambdaQueryWrapper<SysOrg> sysOrgLambdaQueryWrapper = new LambdaQueryWrapper<>();
        sysOrgLambdaQueryWrapper.in(SysOrg::getDeleteFlag, false);
        List<SysOrg> sysOrgs = sysOrgMapper.selectList(sysOrgLambdaQueryWrapper);
        // 列表转map
        Map<Long, SysOrg> sysOrgMap = sysOrgs.stream().collect(Collectors.toMap(SysOrg::getId, sysOrg -> sysOrg));

        LambdaQueryWrapper<SysOrgExt> sysOrgExtLambdaQueryWrapper = new LambdaQueryWrapper<>();
        sysOrgExtLambdaQueryWrapper.in(SysOrgExt::getDeleteFlag, false);
        List<SysOrgExt> sysOrgExts = sysOrgExtMapper.selectList(sysOrgExtLambdaQueryWrapper);
        // 列表转map
        Map<Long, SysOrgExt> sysOrgExtMap = sysOrgExts.stream().collect(Collectors.toMap(SysOrgExt::getExtOrgId, sysOrgExt -> sysOrgExt));

        List<ExtOrgRelationDTO> result = new ArrayList<>();
        // 遍历
        for (SysOrgExt sysOrgExt : sysOrgExts) {
            ExtOrgRelationDTO extOrgRelationDTO = new ExtOrgRelationDTO();
            extOrgRelationDTO.setId(sysOrgExt.getExtOrgId());
            extOrgRelationDTO.setType(sysOrgExt.getExtOrgType());
            SysOrg orDefault = sysOrgMap.getOrDefault(sysOrgExt.getId(), null);
            if(orDefault != null) {
                extOrgRelationDTO.setName(orDefault.getFullName());
                extOrgRelationDTO.setShortName(orDefault.getName());
                extOrgRelationDTO.setOrgNum(orDefault.getOrgCode());
                extOrgRelationDTO.setOrgId(orDefault.getId());
                extOrgRelationDTO.setTenantOrgId(orDefault.getTenantOrgId());
                extOrgRelationDTO.setTenantFlag(orDefault.getId().equals(orDefault.getTenantOrgId()));
                if (orDefault.getPid() != null && sysOrgExtMap.containsKey(orDefault.getPid())) {
                    SysOrgExt parent = sysOrgExtMap.get(orDefault.getPid());
                    extOrgRelationDTO.setParentId(parent.getExtOrgId());
                }
            }
            result.add(extOrgRelationDTO);
        }
        return result;
    }

    @Override
    public SysOrgExtDTO getSysOrgExt(Long orgId, Long extOrgId) {
        if(orgId == null && extOrgId == null) {
            throw new BusinessException(-1, "组织id和扩展组织id不能同时为空");
        }
        SysOrgExt sysOrgExt = null;
        if(orgId != null) {
            sysOrgExt = sysOrgExtMapper.findById(orgId);
        } else  {
            sysOrgExt = sysOrgExtMapper.findByExtOrgId(extOrgId);
        }

        if(sysOrgExt == null) {
            throw new BusinessException(-1, "组织不存在");
        }

        SysOrgExtDTO sysOrgExtDTO = new SysOrgExtDTO();
        sysOrgExtDTO.setOrgId(sysOrgExt.getId());
        sysOrgExtDTO.setExtOrgId(sysOrgExt.getExtOrgId());
        return sysOrgExtDTO;
    }

    @Override
    public List<com.jfeat.org.tree.SysOrgTenantTreeItemDTO> listOrgWithTenant(BaseQueryParam p, String search) {
        return sysOrgMapper.listOrgWithTenant(p, search);
    }
}
