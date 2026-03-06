package com.jfeat;

import org.mybatis.spring.annotation.MapperScan;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@MapperScan({
    "com.jfeat.org.services.persistence.dao",
    "com.jfeat.org.services.domain.dao",
    "com.jfeat.crud.plus.service.dao"
})
public class AmApplication {
    protected final static Logger logger = LoggerFactory.getLogger(AmApplication.class);

    public static void main(String[] args) {
        SpringApplication.run(AmApplication.class, args);
        logger.info("Org module running success!");
    }
}
