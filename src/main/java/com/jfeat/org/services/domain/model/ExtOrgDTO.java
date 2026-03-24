package com.jfeat.org.services.domain.model;

import lombok.Data;

import jakarta.validation.constraints.NotNull;

/**
 * 扩展组织DTO
 * 对应 org-rust 中的 ExtOrgRequest
 */
@Data
public class ExtOrgDTO {
    /**
     * 扩展组织id
     */
    @NotNull(message = "扩展组织id必填")
    private Long id;

    /**
     * 扩展组织父id
     */
    @NotNull(message = "扩展组织父id必填")
    private Long parentId;

    /**
     * 扩展组织名称
     */
    @NotNull(message = "扩展组织名称必填")
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
}
