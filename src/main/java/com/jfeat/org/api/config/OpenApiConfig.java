package com.jfeat.org.api.config;

import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.info.Contact;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.info.License;
import io.swagger.v3.oas.models.servers.Server;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import java.util.List;

/**
 * OpenAPI 3 配置类
 * 用于生成 Swagger API 文档
 *
 * @author JFeat
 */
@Configuration
public class OpenApiConfig {

    /**
     * 配置 OpenAPI 文档信息
     *
     * @return OpenAPI 配置对象
     */
    @Bean
    public OpenAPI customOpenAPI() {
        return new OpenAPI()
                .info(new Info()
                        .title("组织管理模块 API 文档")
                        .description("提供组织架构管理、党组织管理等功能的 RESTful API 接口文档")
                        .version("21.0.0")
                        .contact(new Contact()
                                .name("JFeat Team")
                                .email("support@jfeat.com"))
                        .license(new License()
                                .name("Apache 2.0")
                                .url("https://www.apache.org/licenses/LICENSE-2.0.html")))
                .servers(List.of(
                        new Server()
                                .url("http://localhost:8080")
                                .description("本地开发环境"),
                        new Server()
                                .url("https://api.example.com")
                                .description("生产环境")
                ));
    }
}
