package com.jfeat.org.api.crud;

import com.baomidou.mybatisplus.extension.plugins.pagination.Page;
import com.jfeat.crud.base.tips.SuccessTip;
import com.jfeat.crud.base.tips.Tip;
import com.jfeat.org.services.domain.model.SysTenantDTO;
import com.jfeat.org.services.persistence.dao.SysTenantMapper;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.springframework.web.bind.annotation.*;

import jakarta.annotation.Resource;
import java.util.List;

/**
 * Tenant Management Endpoint
 *
 * @author Claude
 */
@RestController
@Tag(name = "租户管理", description = "提供租户信息的查询功能")
@RequestMapping("/api/adm/tenant")
public class TenantEndpoint {

    @Resource
    private SysTenantMapper sysTenantMapper;

    @GetMapping()
    @Operation(summary = "分页查询租户列表",
            description = "分页查询租户列表，支持按租户名称进行模糊搜索。返回结果包含租户基本信息和关联的组织信息。")
    @ApiResponses({
            @ApiResponse(responseCode = "200", description = "查询成功", content = @Content(schema = @Schema(implementation = Tip.class)))
    })
    public Tip pageTenant(
            Page<SysTenantDTO> page,
            @Parameter(description = "页码，从1开始", example = "1")
            @RequestParam(value = "pageNum", defaultValue = "1") Integer pageNum,
            @Parameter(description = "每页记录数", example = "10")
            @RequestParam(value = "pageSize", defaultValue = "10") Integer pageSize,
            @Parameter(description = "租户名称（支持模糊搜索）", example = "平台")
            @RequestParam(value = "search", required = false) String search) {
        page.setCurrent(pageNum);
        page.setSize(pageSize);
        List<SysTenantDTO> tenants = sysTenantMapper.listTenantWithOrg(page, search);
        page.setRecords(tenants);
        return SuccessTip.create(page);
    }
}
