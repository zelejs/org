package com.jfeat.org.api.crud;

import com.jfeat.crud.base.tips.SuccessTip;
import com.jfeat.crud.base.tips.Tip;
import com.jfeat.org.services.domain.model.ExtOrgDTO;
import com.jfeat.org.services.domain.service.SysOrgService;
import com.jfeat.org.services.persistence.dao.SysOrgMapper;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;

import jakarta.annotation.Resource;

/**
 * 扩展组织管理接口
 * 提供扩展组织的增删改查、同步等功能
 */
@RestController
@Tag(name = "扩展组织管理", description = "提供扩展组织的增删改查、组织列表查询、数据同步等功能")
@RequestMapping("/api/adm/sys/extOrg")
public class ExtOrgEndpoint {
    protected static Logger logger = LoggerFactory.getLogger(ExtOrgEndpoint.class);

    @Resource
    SysOrgService sysOrgService;

    @Resource
    SysOrgMapper sysOrgMapper;

    @Operation(summary = "添加扩展组织",
            description = "创建新的扩展组织。需要提供扩展组织的基本信息，包括组织名称、上级组织ID、组织类型等。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "创建成功", content = @Content(schema = @Schema(implementation = Tip.class))),
            @ApiResponse(responseCode = "400", description = "请求参数错误")
    })
    @PostMapping("/add")
    public Tip add(
            @Parameter(description = "扩展组织信息", required = true)
            @RequestBody @Validated ExtOrgDTO extOrgDTO) {
        return SuccessTip.create(sysOrgService.extAdd(extOrgDTO));
    }

    @Operation(summary = "删除扩展组织",
            description = "根据组织ID删除指定的扩展组织。删除前会检查该组织下是否存在子组织或关联数据。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "删除成功", content = @Content(schema = @Schema(implementation = Tip.class))),
            @ApiResponse(responseCode = "400", description = "该组织下存在子组织或关联数据，无法删除"),
            @ApiResponse(responseCode = "404", description = "组织不存在")
    })
    @DeleteMapping("/delete")
    public Tip delete(
            @Parameter(description = "要删除的扩展组织ID", required = true, example = "1")
            @RequestParam(value = "id", required = true) Long id) {
        return SuccessTip.create(sysOrgService.extDelete(id));
    }

    @PutMapping("/update")
    @Operation(summary = "修改扩展组织信息",
            description = "根据提供的扩展组织ID和更新信息，修改对应扩展组织的基本信息。" +
                    "组织ID和租户信息不可修改。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "更新成功", content = @Content(schema = @Schema(implementation = Tip.class))),
            @ApiResponse(responseCode = "400", description = "请求参数错误"),
            @ApiResponse(responseCode = "404", description = "组织不存在")
    })
    public Tip update(
            @Parameter(description = "扩展组织更新信息", required = true)
            @RequestBody @Validated ExtOrgDTO extOrgDTO) {
        return SuccessTip.create(sysOrgService.extUpdate(extOrgDTO));
    }

    @GetMapping("/list")
    @Operation(summary = "获取扩展组织列表",
            description = "获取所有扩展组织的列表信息，返回结果包含组织ID、名称、上级组织ID等基本信息。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "查询成功", content = @Content(schema = @Schema(implementation = Tip.class)))
    })
    public Tip list() {
        return SuccessTip.create(sysOrgService.extList());
    }

    @PostMapping("/sync")
    @Operation(summary = "同步扩展组织数据",
            description = "从外部数据源同步扩展组织数据，更新本地扩展组织信息。" +
                    "同步过程会匹配已有组织并更新其信息，对于新组织则进行创建。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "同步成功", content = @Content(schema = @Schema(implementation = Tip.class))),
            @ApiResponse(responseCode = "500", description = "同步失败，请检查外部数据源连接")
    })
    public Tip sync() {
        return SuccessTip.create(sysOrgService.sync());
    }
}
