package com.jfeat.org.api.crud;

import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.jfeat.am.core.jwt.JWTKit;
import com.xinzhi.plat.common.result.ApiResult;
import com.jfeat.org.config.BaseQueryParam;
import com.jfeat.org.constant.OrganizationTypeConstants;
import com.jfeat.org.services.domain.model.SysUserOrgRequest;
import com.jfeat.org.services.domain.service.SysOrgService;
import com.jfeat.org.services.persistence.dao.SysOrgMapper;
import com.jfeat.org.services.persistence.model.SysOrg;
import com.jfeat.org.tree.SysOrgTenantTreeItemDTO;
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

    @PostMapping("/{id}/children")
    @Operation(summary = "增加子组织",
            description = "在指定组织节点下创建新的子组织。创建时会自动继承父组织的租户ID和租户组织ID。" +
                    "平台类型和租户类型的组织不允许创建子节点。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "创建成功", content = @Content(schema = @Schema(implementation = ApiResult.class))),
            @ApiResponse(responseCode = "400", description = "请求参数错误"),
            @ApiResponse(responseCode = "401", description = "未授权访问")
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_ADD)
    public ApiResult<Long> createNodeChildren(
            @Parameter(description = "父组织ID", required = true, example = "1")
            @PathVariable Long id,
            @Parameter(description = "子组织信息", required = true)
            @RequestBody SysUserOrgRequest entity) {
        entity.setPid(id);
        Long orgId = JWTKit.getOrgId();
        if (orgId == null) {
            return ApiResult.error("用户未登录");
        }
        // 获取父组织 并使用继承的租户组织id
        SysOrg sysOrg = sysOrgMapper.findById(id);
        if(sysOrg == null) {
            return ApiResult.error("父组织不存在");
        }
        entity.setTenantId(sysOrg.getTenantId());
        entity.setTenantOrgId(sysOrg.getTenantOrgId());
        if(entity.getOrgType() <= OrganizationTypeConstants.TENANT) {
           return ApiResult.error("平台跟租户类型的组织不可创建");
        }
        return ApiResult.success(sysOrgService.createNewNode(orgId, sysOrg.getTenantOrgId(), entity, true));
    }

    @DeleteMapping("/{id}")
    @Operation(summary = "删除组织",
            description = "根据组织ID删除指定的组织节点。删除组织前会检查该组织下是否存在子组织，" +
                    "如果存在子组织则需要先删除所有子组织才能删除父组织。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "删除成功", content = @Content(schema = @Schema(implementation = ApiResult.class))),
            @ApiResponse(responseCode = "400", description = "该组织下存在子组织，无法删除"),
            @ApiResponse(responseCode = "404", description = "组织不存在")
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_DEL)
    public ApiResult<Long> deleteNode(
            @Parameter(description = "要删除的组织ID", required = true, example = "1")
            @PathVariable Long id) {
        Long orgId = JWTKit.getOrgId();
        if (orgId == null) {
            return ApiResult.error("用户未登录");
        }
        return ApiResult.success(sysOrgService.deleteNode(orgId, id));
    }

    @PutMapping("/{id}")
    @Operation(summary = "更新组织信息",
            description = "根据组织ID更新组织的基本信息。可以更新组织名称、组织类型、描述等信息。" +
                    "组织ID和租户信息不可修改。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "更新成功", content = @Content(schema = @Schema(implementation = ApiResult.class))),
            @ApiResponse(responseCode = "400", description = "请求参数错误"),
            @ApiResponse(responseCode = "404", description = "组织不存在")
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_EDIT)
    public ApiResult<Long> updateNode(
            @Parameter(description = "要更新的组织ID", required = true, example = "1")
            @PathVariable Long id,
            @Parameter(description = "组织更新信息", required = true)
            @RequestBody SysOrg entity) {
        Long orgId = JWTKit.getOrgId();
        if (orgId == null) {
            return ApiResult.error("用户未登录");
        }
        return ApiResult.success(sysOrgService.updateNode(orgId, id, entity));
    }

    @GetMapping("/{id}")
    @Operation(summary = "获取组织详情",
            description = "根据组织ID获取单个组织的详细信息，包括组织名称、类型、父组织ID、租户信息等。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "获取成功", content = @Content(schema = @Schema(implementation = ApiResult.class))),
            @ApiResponse(responseCode = "404", description = "组织不存在")
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_VIEW)
    public ApiResult<SysOrg> getOrg(
            @Parameter(description = "组织ID", required = true, example = "1")
            @PathVariable Long id) {
        Long orgId = JWTKit.getOrgId();
        if (orgId == null) {
            return ApiResult.error("用户未登录");
        }
        SysOrg visibleOrg = sysOrgService.getVisibleOrg(orgId, id);
        if (visibleOrg == null) {
            return ApiResult.error("组织不存在");
        }
        return ApiResult.success(visibleOrg);
    }

    @GetMapping()
    @Operation(summary = "分页查询组织列表",
            description = "分页查询组织列表，支持按组织名称进行模糊搜索。返回结果包含组织基本信息和分页数据。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "查询成功", content = @Content(schema = @Schema(implementation = ApiResult.class)))
    })
//    @com.jfeat.am.common.annotation.Permission(Permission.ORG_VIEW)
    public ApiResult<Page<SysOrg>> pageSysOrg(Page<SysOrg> page,
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
        return ApiResult.success(page);
    }

    @GetMapping("/tree")
    @Operation(summary = "获取组织树形结构",
            description = "以树形结构返回组织列表。每个组织节点包含其子组织列表，" +
                    "支持按组织名称进行搜索过滤。返回的树形结构从当前用户的组织开始。" +
                    "appid参数仅在JWT中appid为null时生效。" +
                    "当JWT中appid和orgId都为null时，默认使用orgId=1查询所有组织。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "查询成功", content = @Content(schema = @Schema(implementation = ApiResult.class)))
    })
    public ApiResult<SysOrgTreeItemDTO> treeSysOrg(
            @Parameter(description = "搜索关键词（组织名称，支持模糊搜索）", example = "技术")
            @RequestParam(value = "search", required = false) String search,
            @Parameter(description = "应用ID，仅在JWT中appid为null时生效", example = "school-app")
            @RequestParam(value = "appid", required = false) String appid) {
        // 获取JWT中的appid和orgId
        String jwtAppid = JWTKit.getAppid();
        Long jwtOrgId = JWTKit.getOrgId();

        // 优先使用JWT中的appid，如果JWT中appid为null才使用参数中的appid
        String finalAppid = (jwtAppid != null && !jwtAppid.isEmpty()) ? jwtAppid : appid;

        // 确定orgId：如果JWT返回的orgId为null且最终appid也为null，设置默认值orgId=1
        final Long orgId;
        if (jwtOrgId == null && (finalAppid == null || finalAppid.isEmpty())) {
            logger.debug("JWTKit.getOrgId()=null && appid=null, setting default orgId=1");
            orgId = 1L;
        } else {
            orgId = jwtOrgId;
        }

        // 创建查询参数，设置appid
        BaseQueryParam queryParam;
        // 如果通过appid查询（JWT中的appid为null），使用appid对应的顶级组织作为根节点
        if (jwtAppid == null && appid != null && !appid.isEmpty()) {
            // 创建不带过滤条件的查询参数
            queryParam = new BaseQueryParam();
            queryParam.setAppid(finalAppid);
            queryParam.setOrgId(1L); // 设置为1以避免子树过滤（SQL中 orgId != 1 的条件不会触发）
            queryParam.setFilterType(0); // 设置为0以确保不触发过滤条件
            logger.debug("Querying by appid, set orgId=1 and filterType=0 to avoid subtree filtering");
        } else {
            queryParam = BaseQueryParam.create();
            queryParam.setAppid(finalAppid);
        }

        logger.info("Executing query with appid: {}, orgId: {}, filterType: {}, tenantOrgId: {}",
            queryParam.getAppid(), queryParam.getOrgId(), queryParam.getFilterType(), queryParam.getTenantOrgId());

        List<SysOrg> orgList = sysOrgService.listLikeNameAndOrgId(queryParam, search);
        logger.info("Query returned {} organizations", orgList.size());

        // 转换为树节点DTO
        List<SysOrgTreeItemDTO> treeItems = orgList.stream().map(s -> {
            SysOrgTreeItemDTO sysOrgTreeItemDTO = new SysOrgTreeItemDTO();
            sysOrgTreeItemDTO.setTenantFlag(s.getId().equals(s.getTenantOrgId()));
            BeanUtils.copyProperties(s, sysOrgTreeItemDTO);
            return sysOrgTreeItemDTO;
        }).collect(Collectors.toList());

        // 如果没有查询到任何组织，返回空树
        if (treeItems.isEmpty()) {
            logger.info("No organizations found for appid: {}, search: {}", finalAppid, search);
            SysOrgTreeItemDTO top = new SysOrgTreeItemDTO();
            top.setChildren(List.of());
            return ApiResult.success(top);
        }

        logger.debug("Query returned {} organizations", treeItems.size());
        treeItems.forEach(item -> logger.debug("Org: id={}, name={}, pid={}, appid={}",
            item.getId(), item.getName(), item.getPid(), item.getAppid()));

        // 确定树的根节点ID
        Long treeRootId = orgId;
        logger.debug("Initial treeRootId from JWT: {}, jwtAppid: {}, paramAppid: {}", treeRootId, jwtAppid, appid);

        // 如果通过appid参数查询（JWT中的appid为null），使用appid对应的顶级组织作为根节点
        if (jwtAppid == null && appid != null && !appid.isEmpty()) {
            // 查找appid匹配的顶级组织
            treeRootId = treeItems.stream()
                .filter(item -> {
                    String itemAppid = item.getAppid();
                    // 处理字符串比较，包括null值和类型转换
                    boolean matches = itemAppid != null && itemAppid.equals(appid) && item.getPid() == null;
                    logger.debug("Filtering item: id={}, appid='{}', pid={}, matches={}",
                        item.getId(), itemAppid, item.getPid(), matches);
                    return matches;
                })
                .map(SysOrgTreeItemDTO::getId)
                .findFirst()
                .orElseGet(() -> {
                    // 如果找不到appid匹配的顶级组织，尝试找第一个顶级组织
                    logger.warn("No top-level org found with appid={}, trying to find any top-level org", appid);
                    return treeItems.stream()
                        .filter(item -> item.getPid() == null)
                        .map(SysOrgTreeItemDTO::getId)
                        .findFirst()
                        .orElse(orgId);
                });
            logger.debug("After appid filtering, treeRootId: {}", treeRootId);
        }

        // 检查treeRootId是否在treeItems中
        Long finalTreeRootId = treeRootId;
        if (treeItems.stream().noneMatch(item -> item.getId().equals(finalTreeRootId))) {
            // 如果指定的根节点不在查询结果中，使用第一个顶级组织作为根节点
            logger.warn("treeRootId {} not in query results, finding first top-level org", treeRootId);
            treeRootId = treeItems.stream()
                .filter(item -> item.getPid() == null)
                .map(SysOrgTreeItemDTO::getId)
                .findFirst()
                .orElse(treeItems.get(0).getId());
            logger.debug("After fallback, treeRootId: {}", treeRootId);
        }

        logger.info("Building tree with rootId: {}", treeRootId);
        SysOrgTreeItemDTO sysOrgTreeItemDTO = TreeUtls.buildTree(treeItems, treeRootId);
        SysOrgTreeItemDTO top = new SysOrgTreeItemDTO();
        top.setChildren(List.of(sysOrgTreeItemDTO));
        return ApiResult.success(top);
    }

    @GetMapping("/tenant/tree")
    @Operation(summary = "获取组织树形结构（含租户信息）",
            description = "以树形结构返回组织列表，每个组织节点包含租户信息（tenantCode, tenantName, tenantId, tenantOrgId）。" +
                    "支持按组织名称进行搜索过滤。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "查询成功", content = @Content(schema = @Schema(implementation = ApiResult.class)))
    })
    public ApiResult<SysOrgTenantTreeItemDTO> treeSysOrgWithTenant(
            @Parameter(description = "搜索关键词（组织名称，支持模糊搜索）", example = "技术")
            @RequestParam(value = "search", required = false) String search,
            @Parameter(description = "应用ID", example = "school-app")
            @RequestParam(value = "appid", required = false) String appid) {
        // 获取JWT中的appid和orgId
        String jwtAppid = JWTKit.getAppid();
        Long jwtOrgId = JWTKit.getOrgId();

        // 优先使用JWT中的appid，如果JWT中appid为null才使用参数中的appid
        String finalAppid = (jwtAppid != null && !jwtAppid.isEmpty()) ? jwtAppid : appid;

        // 确定orgId
        final Long orgId;
        if (jwtOrgId == null && (finalAppid == null || finalAppid.isEmpty())) {
            logger.debug("JWTKit.getOrgId()=null && appid=null, setting default orgId=1");
            orgId = 1L;
        } else {
            orgId = jwtOrgId;
        }

        // 创建查询参数
        BaseQueryParam queryParam;
        if (jwtAppid == null && appid != null && !appid.isEmpty()) {
            queryParam = new BaseQueryParam();
            queryParam.setAppid(finalAppid);
            queryParam.setOrgId(1L);
            queryParam.setFilterType(0);
            logger.debug("Querying by appid, set orgId=1 and filterType=0 to avoid subtree filtering");
        } else {
            queryParam = BaseQueryParam.create();
            queryParam.setAppid(finalAppid);
        }

        logger.info("Executing tenant tree query with appid: {}, orgId: {}, filterType: {}, tenantOrgId: {}",
            queryParam.getAppid(), queryParam.getOrgId(), queryParam.getFilterType(), queryParam.getTenantOrgId());

        // 查询组织树与租户信息
        List<SysOrgTenantTreeItemDTO> orgList = sysOrgService.listOrgWithTenant(queryParam, search);
        logger.info("Query returned {} organizations with tenant info", orgList.size());

        // 如果没有查询到任何组织，返回空树
        if (orgList.isEmpty()) {
            logger.info("No organizations found for appid: {}, search: {}", finalAppid, search);
            SysOrgTenantTreeItemDTO top = new SysOrgTenantTreeItemDTO();
            top.setChildren(List.of());
            return ApiResult.success(top);
        }

        // 确定树的根节点ID
        Long treeRootId = orgId;
        logger.debug("Initial treeRootId from JWT: {}, jwtAppid: {}, paramAppid: {}", treeRootId, jwtAppid, appid);

        if (jwtAppid == null && appid != null && !appid.isEmpty()) {
            treeRootId = orgList.stream()
                .filter(item -> {
                    String itemAppid = item.getAppid();
                    boolean matches = itemAppid != null && itemAppid.equals(appid) && item.getPid() == null;
                    logger.debug("Filtering item: id={}, appid='{}', pid={}, matches={}",
                        item.getId(), itemAppid, item.getPid(), matches);
                    return matches;
                })
                .map(SysOrgTenantTreeItemDTO::getId)
                .findFirst()
                .orElseGet(() -> {
                    logger.warn("No top-level org found with appid={}, trying to find any top-level org", appid);
                    return orgList.stream()
                        .filter(item -> item.getPid() == null)
                        .map(SysOrgTenantTreeItemDTO::getId)
                        .findFirst()
                        .orElse(orgId);
                });
            logger.debug("After appid filtering, treeRootId: {}", treeRootId);
        }

        // 检查treeRootId是否在orgList中
        Long finalTreeRootId = treeRootId;
        if (orgList.stream().noneMatch(item -> item.getId().equals(finalTreeRootId))) {
            logger.warn("treeRootId {} not in query results, finding first top-level org", treeRootId);
            treeRootId = orgList.stream()
                .filter(item -> item.getPid() == null)
                .map(SysOrgTenantTreeItemDTO::getId)
                .findFirst()
                .orElse(orgList.get(0).getId());
            logger.debug("After fallback, treeRootId: {}", treeRootId);
        }

        logger.info("Building tenant tree with rootId: {}", treeRootId);
        SysOrgTenantTreeItemDTO tenantTree = TreeUtls.buildTree(orgList, treeRootId);
        SysOrgTenantTreeItemDTO top = new SysOrgTenantTreeItemDTO();
        top.setChildren(List.of(tenantTree));
        return ApiResult.success(top);
    }
}