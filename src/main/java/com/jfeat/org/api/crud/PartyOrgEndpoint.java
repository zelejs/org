package com.jfeat.org.api.crud;

import com.jfeat.crud.base.tips.SuccessTip;
import com.jfeat.crud.base.tips.Tip;
import com.jfeat.org.services.domain.model.PartyOrgDTO;
import com.jfeat.org.services.domain.service.SysOrgService;
import com.jfeat.org.services.persistence.dao.SysOrgMapper;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;

import jakarta.annotation.Resource;

@RestController
@Tag(name = "sys-党组织结构")
@RequestMapping("/api/adm/sys/partyOrg")
public class PartyOrgEndpoint {
    protected static Logger logger = LoggerFactory.getLogger(PartyOrgEndpoint.class);

    @Resource
    SysOrgService sysOrgService;

    @Resource
    SysOrgMapper sysOrgMapper;

    @Operation(summary = "添加")
    @PostMapping("/add")
    public Tip add(@RequestBody @Validated PartyOrgDTO partyOrgDTO) {
        return SuccessTip.create(sysOrgService.partyAdd(partyOrgDTO));
    }

    @Operation(summary = "删除")
    @DeleteMapping("/delete")
    public Tip delete(@RequestParam(value = "id", required = true) Long id) {
        return SuccessTip.create(sysOrgService.partyDelete(id));
    }

    @PutMapping("/update")
    @Operation(summary = "修改")
    public Tip update(@RequestBody @Validated PartyOrgDTO partyOrgDTO) {
        return SuccessTip.create(sysOrgService.partyUpdate(partyOrgDTO));
    }

    @GetMapping("/list")
    @Operation(summary = "组织列表")
    public Tip list() {
        return SuccessTip.create(sysOrgService.partyList());
    }

    @PostMapping("/sync")
    @Operation(summary = "同步")
    public Tip sync() {
        return SuccessTip.create(sysOrgService.sync());
    }




}