package com.jfeat.org.services.domain.model;


import lombok.Data;

import jakarta.validation.constraints.NotNull;

@Data
public class PartyOrgRelationDTO {
    // 党组织id
    @NotNull(message = "党组织id必填")
    private Long id;

    // 党组织父id
    @NotNull(message = "党组织父id必填")
    private Long parentId;

    // 党组织名称
    @NotNull(message = "党组织名称")
    private String name;

    // 党组织简称
    private String shortName;

    // 党组织代码
    private String orgNum;

    // 党组织类型
    private Integer type;

    // 组织id 创建更新不传
    private Long orgId;
    // 租户组织id 创建更新不传
    private Long tenantOrgId;
    // 是否租户 创建更新不传
    private Boolean tenantFlag;
}
