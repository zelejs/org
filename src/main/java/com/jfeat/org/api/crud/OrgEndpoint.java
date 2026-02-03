package com.jfeat.org.api.crud;

import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.jfeat.am.core.jwt.JWTKit;
import com.jfeat.crud.base.annotation.BusinessLog;
import com.jfeat.crud.base.tips.ErrorTip;
import com.jfeat.crud.base.tips.SuccessTip;
import com.jfeat.crud.base.tips.Tip;
import com.jfeat.org.config.BaseQueryParam;
import com.jfeat.org.constant.OrganizationTypeConstants;
import com.jfeat.org.services.domain.model.SysUserOrgRequest;
import com.jfeat.org.services.domain.service.SysOrgService;
import com.jfeat.org.services.persistence.dao.SysOrgMapper;
import com.jfeat.org.services.persistence.model.SysOrg;
import com.jfeat.org.tree.SysOrgTreeItemDTO;
import com.jfeat.org.tree.TreeUtls;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.BeanUtils;
import org.springframework.web.bind.annotation.*;

import jakarta.annotation.Resource;
import java.util.List;
import java.util.stream.Collectors;

/**
 * <p>
 * api
 * </P>
 *
 * @author 莫昌廉
 */
@RestController
@Tag(name = "sys-组织结构")
@RequestMapping("/api/adm/org")
public class OrgEndpoint {
    protected static Logger logger = LoggerFactory.getLogger(OrgEndpoint.class);

    @Resource
    SysOrgService sysOrgService;

    @Resource
    SysOrgMapper sysOrgMapper;

    @BusinessLog(name = "组织", value = "增加子组织")
    @PostMapping("/{id}/children")
    @Operation(summary = "增加节点")
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_ADD)
    public Tip createNodeChildren(@PathVariable Long id, @RequestBody SysUserOrgRequest entity) {
        entity.setPid(id);
        Long orgId = JWTKit.getOrgId();
        // 获取父组织 并使用继承的租户组织id
        SysOrg sysOrg = sysOrgMapper.findById(id);
        if(sysOrg == null) {
            return ErrorTip.create(-1,"父组织不存在");
        }
        entity.setTenantId(sysOrg.getTenantId());
        entity.setTenantOrgId(sysOrg.getTenantOrgId());
        if(entity.getOrgType() <= OrganizationTypeConstants.TENANT) {
           return ErrorTip.create(-1,"平台跟租户类型的组织不可创建");
        }
        return SuccessTip.create(sysOrgService.createNewNode(orgId, sysOrg.getTenantOrgId(), entity, true));
    }

    @BusinessLog(name = "组织", value = "删除组织")
    @DeleteMapping("/{id}")
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_DEL)
    @Operation(summary = "删除节点")
    public Tip deleteNode(@PathVariable Long id) {
        return SuccessTip.create(sysOrgService.deleteNode(JWTKit.getOrgId(), id));
    }

    @BusinessLog(name = "组织", value = "更新组织")
    @PutMapping("/{id}")
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_EDIT)
    @Operation(summary = "更新节点信息")
    public Tip updateNode(@PathVariable Long id, @RequestBody SysOrg entity) {
        return SuccessTip.create(sysOrgService.updateNode(JWTKit.getOrgId(), id, entity));
    }

    @GetMapping("/{id}")
    @Operation(summary = "查看单个节点的信息")
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_VIEW)
    public Tip getOrg(@PathVariable Long id) {
        SysOrg visibleOrg = sysOrgService.getVisibleOrg(JWTKit.getOrgId(), id);
        if (visibleOrg == null) {
            return ErrorTip.create(-1,"组织不存在");
        }
        return SuccessTip.create(visibleOrg);
    }

    @GetMapping()
    @Operation(summary = "分页查询")
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_VIEW)
    public Tip pageSysOrg(Page<SysOrg> page,
                            @RequestParam(value = "pageNum", defaultValue = "1") Integer pageNum,
                            @RequestParam(value = "pageSize", defaultValue = "10") Integer pageSize,
                            @RequestParam(value = "name", required = false) String name) {
        page.setCurrent(pageNum);
        page.setSize(pageSize);
        List<SysOrg> sysOrgs = sysOrgService.pageLikeNameAndOrgId(page,  name);
        page.setRecords(sysOrgs);
        return SuccessTip.create(page);
    }

    @GetMapping("/tree")
    @Operation(summary = "树状返回组织信息列表")
    public Tip treeSysOrg(@RequestParam(value = "search", required = false) String search) {
        List<SysOrg> orgList = sysOrgService.listLikeNameAndOrgId(BaseQueryParam.create(), search);
        List<SysOrgTreeItemDTO> treeItems = orgList.stream().map(s -> {
            SysOrgTreeItemDTO sysOrgTreeItemDTO = new SysOrgTreeItemDTO();
            sysOrgTreeItemDTO.setTenantFlag(s.getId().equals(s.getTenantOrgId()));
            BeanUtils.copyProperties(s, sysOrgTreeItemDTO);
            return sysOrgTreeItemDTO;
        }).collect(Collectors.toList());
        SysOrgTreeItemDTO sysOrgTreeItemDTO = TreeUtls.buildTree(treeItems, JWTKit.getOrgId());
        SysOrgTreeItemDTO top = new SysOrgTreeItemDTO();
        top.setChildren(List.of(sysOrgTreeItemDTO));
        return SuccessTip.create(top);
    }
}