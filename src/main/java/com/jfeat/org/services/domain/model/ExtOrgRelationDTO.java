package com.jfeat.org.services.domain.model;

import lombok.Data;

import jakarta.validation.constraints.NotNull;

/**
 * 扩展组织关系DTO
 * 对应 org-rust 中的 ExtOrgRelation
 */
@Data
public class ExtOrgRelationDTO {
    /**
     * 扩展组织id
     */
    @NotNull(message = "扩展组织id必填")
    private Long id;

    /**
     * 扩展组织父id
     */
    private Long parentId;

    /**
     * 扩展组织名称
     */
    private String name;

    /**
     * 扩展组织简称
     */
    private String shortName;

    /**
     * 扩展组织代码
     */
    private String orgNum;

    /**
     * 扩展组织类型
     */
    private Integer type;

    /**
     * 组织id 创建更新不传
     */
    private Long orgId;

    /**
     * 租户组织id 创建更新不传
     */
    private Long tenantOrgId;

    /**
     * 是否租户 创建更新不传
     */
    private Boolean tenantFlag;
}
