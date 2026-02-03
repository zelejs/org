package com.jfeat.org.services.domain.definition;

/**
 * Created by zy on 2018/12/25.
 */
public enum Platform {
    ID(1L),
    LEVER(1L),
    SYSTEM(100000000000000010L);

    private Long value;
    Platform(Long value) {
        this.value = value;
    }
    public Long getValue() {
        return value;
    }
}
