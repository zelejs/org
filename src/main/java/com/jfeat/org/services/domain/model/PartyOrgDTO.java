package com.jfeat.org.services.domain.model;


import lombok.Data;

import javax.validation.constraints.NotNull;

@Data
public class PartyOrgDTO {
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
}
