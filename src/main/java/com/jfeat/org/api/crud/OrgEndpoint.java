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
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.BeanUtils;
import org.springframework.web.bind.annotation.*;

import jakarta.annotation.Resource;
import java.util.List;
import java.util.stream.Collectors;

/**
 * 组织管理接口
 * 提供组织架构的增删改查、树形结构查询等功能
 *
 * @author 莫昌廉
 */
@RestController
@Tag(name = "组织管理", description = "提供组织架构的增删改查、树形结构查询等功能")
@RequestMapping("/api/adm/org")
public class OrgEndpoint {
    protected static Logger logger = LoggerFactory.getLogger(OrgEndpoint.class);

    @Resource
    SysOrgService sysOrgService;

    @Resource
    SysOrgMapper sysOrgMapper;

    @BusinessLog(name = "组织", value = "增加子组织")
    @PostMapping("/{id}/children")
    @Operation(summary = "增加子组织",
            description = "在指定组织节点下创建新的子组织。创建时会自动继承父组织的租户ID和租户组织ID。" +
                    "平台类型和租户类型的组织不允许创建子节点。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "创建成功", content = @Content(schema = @Schema(implementation = Tip.class))),
            @ApiResponse(responseCode = "400", description = "请求参数错误"),
            @ApiResponse(responseCode = "401", description = "未授权访问")
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_ADD)
    public Tip createNodeChildren(
            @Parameter(description = "父组织ID", required = true, example = "1")
            @PathVariable Long id,
            @Parameter(description = "子组织信息", required = true)
            @RequestBody SysUserOrgRequest entity) {
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
    @Operation(summary = "删除组织",
            description = "根据组织ID删除指定的组织节点。删除组织前会检查该组织下是否存在子组织，" +
                    "如果存在子组织则需要先删除所有子组织才能删除父组织。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "删除成功", content = @Content(schema = @Schema(implementation = Tip.class))),
            @ApiResponse(responseCode = "400", description = "该组织下存在子组织，无法删除"),
            @ApiResponse(responseCode = "404", description = "组织不存在")
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_DEL)
    public Tip deleteNode(
            @Parameter(description = "要删除的组织ID", required = true, example = "1")
            @PathVariable Long id) {
        return SuccessTip.create(sysOrgService.deleteNode(JWTKit.getOrgId(), id));
    }

    @BusinessLog(name = "组织", value = "更新组织")
    @PutMapping("/{id}")
    @Operation(summary = "更新组织信息",
            description = "根据组织ID更新组织的基本信息。可以更新组织名称、组织类型、描述等信息。" +
                    "组织ID和租户信息不可修改。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "更新成功", content = @Content(schema = @Schema(implementation = Tip.class))),
            @ApiResponse(responseCode = "400", description = "请求参数错误"),
            @ApiResponse(responseCode = "404", description = "组织不存在")
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_EDIT)
    public Tip updateNode(
            @Parameter(description = "要更新的组织ID", required = true, example = "1")
            @PathVariable Long id,
            @Parameter(description = "组织更新信息", required = true)
            @RequestBody SysOrg entity) {
        return SuccessTip.create(sysOrgService.updateNode(JWTKit.getOrgId(), id, entity));
    }

    @GetMapping("/{id}")
    @Operation(summary = "获取组织详情",
            description = "根据组织ID获取单个组织的详细信息，包括组织名称、类型、父组织ID、租户信息等。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "获取成功", content = @Content(schema = @Schema(implementation = Tip.class))),
            @ApiResponse(responseCode = "404", description = "组织不存在")
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_VIEW)
    public Tip getOrg(
            @Parameter(description = "组织ID", required = true, example = "1")
            @PathVariable Long id) {
        SysOrg visibleOrg = sysOrgService.getVisibleOrg(JWTKit.getOrgId(), id);
        if (visibleOrg == null) {
            return ErrorTip.create(-1,"组织不存在");
        }
        return SuccessTip.create(visibleOrg);
    }

    @GetMapping()
    @Operation(summary = "分页查询组织列表",
            description = "分页查询组织列表，支持按组织名称进行模糊搜索。返回结果包含组织基本信息和分页数据。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "查询成功", content = @Content(schema = @Schema(implementation = Tip.class)))
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_VIEW)
    public Tip pageSysOrg(Page<SysOrg> page,
                            @Parameter(description = "页码，从1开始", example = "1")
                            @RequestParam(value = "pageNum", defaultValue = "1") Integer pageNum,
                            @Parameter(description = "每页记录数", example = "10")
                            @RequestParam(value = "pageSize", defaultValue = "10") Integer pageSize,
                            @Parameter(description = "组织名称（支持模糊搜索）", example = "技术部")
                            @RequestParam(value = "name", required = false) String name) {
        page.setCurrent(pageNum);
        page.setSize(pageSize);
        List<SysOrg> sysOrgs = sysOrgService.pageLikeNameAndOrgId(page,  name);
        page.setRecords(sysOrgs);
        return SuccessTip.create(page);
    }

    @GetMapping("/tree")
    @Operation(summary = "获取组织树形结构",
            description = "以树形结构返回组织列表。每个组织节点包含其子组织列表，" +
                    "支持按组织名称进行搜索过滤。返回的树形结构从当前用户的组织开始。" +
                    "appid参数仅在JWT中appid为null时生效。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "查询成功", content = @Content(schema = @Schema(implementation = Tip.class))),
            @ApiResponse(responseCode = "400", description = "请求参数错误：JWTKit.getOrgId()=null 且 appid=null")
    })
    public Tip treeSysOrg(
            @Parameter(description = "搜索关键词（组织名称，支持模糊搜索）", example = "技术")
            @RequestParam(value = "search", required = false) String search,
            @Parameter(description = "应用ID，仅在JWT中appid为null时生效", example = "school-app")
            @RequestParam(value = "appid", required = false) String appid) {
        // 获取JWT中的appid和orgId
        String jwtAppid = JWTKit.getAppid();
        Long jwtOrgId = JWTKit.getOrgId();

        // 优先使用JWT中的appid，如果JWT中appid为null才使用参数中的appid
        String finalAppid = (jwtAppid != null && !jwtAppid.isEmpty()) ? jwtAppid : appid;
        Long orgId = jwtOrgId;

        // 如果JWT返回的orgId为null且最终appid也为null，抛异常
        if (orgId == null && (finalAppid == null || finalAppid.isEmpty())) {
            throw new com.jfeat.crud.base.exception.BusinessException(-1,
                "JWTKit.getOrgId()=null && appid=null");
        }

        // 创建查询参数，设置appid
        BaseQueryParam queryParam = BaseQueryParam.create();
        queryParam.setAppid(finalAppid);
        List<SysOrg> orgList = sysOrgService.listLikeNameAndOrgId(queryParam, search);

        // 如果最终使用的appid来自参数（JWT中appid为null），查找该appid对应的根组织
        if (jwtAppid == null && appid != null && !appid.isEmpty()) {
            SysOrgTreeItemDTO rootItem = orgList.stream()
                .filter(org -> appid.equals(org.getAppid()) && org.getPid() == null)
                .findFirst()
                .map(org -> {
                    SysOrgTreeItemDTO dto = new SysOrgTreeItemDTO();
                    dto.setTenantFlag(org.getId().equals(org.getTenantOrgId()));
                    BeanUtils.copyProperties(org, dto);
                    return dto;
                })
                .orElse(null);

            if (rootItem != null) {
                SysOrgTreeItemDTO top = new SysOrgTreeItemDTO();
                top.setChildren(List.of(rootItem));
                return SuccessTip.create(top);
            }
        }

        // 转换为树节点DTO
        List<SysOrgTreeItemDTO> treeItems = orgList.stream().map(s -> {
            SysOrgTreeItemDTO sysOrgTreeItemDTO = new SysOrgTreeItemDTO();
            sysOrgTreeItemDTO.setTenantFlag(s.getId().equals(s.getTenantOrgId()));
            BeanUtils.copyProperties(s, sysOrgTreeItemDTO);
            return sysOrgTreeItemDTO;
        }).collect(Collectors.toList());

        SysOrgTreeItemDTO sysOrgTreeItemDTO = TreeUtls.buildTree(treeItems, orgId);
        SysOrgTreeItemDTO top = new SysOrgTreeItemDTO();
        top.setChildren(List.of(sysOrgTreeItemDTO));
        return SuccessTip.create(top);
    }
}