# Spring Boot 3.x 升级故障排除指南

本文档记录了从 Spring Boot 2.x 升级到 3.x 过程中的常见问题和解决方案。

## 问题 1: Profile 配置语法错误（最常见）

### 症状
```
org.springframework.boot.context.config.InvalidConfigDataPropertyException:
Property 'spring.profiles' imported from location 'class path resource [application.yml]'
is invalid and should be replaced with 'spring.config.activate.on-profile'
[origin: class path resource [application.yml] - 42:13]
```

### 原因
Spring Boot 2.4+ 引入了新的配置格式，Spring Boot 3.x 强制使用新格式。

### 解决方案

**旧语法（Spring Boot 2.x）**:
```yaml
---
spring:
  profiles: dev
  datasource:
    url: jdbc:mysql://localhost:3306/mydb
    username: root
    password: password
```

**新语法（Spring Boot 3.x）**:
```yaml
---
spring:
  config:
    activate:
      on-profile: dev
  datasource:
    url: jdbc:mysql://localhost:3306/mydb
    username: root
    password: password
```

### 注意事项
- `spring.profiles.active` 用于激活 profile 仍然是有效的
- `spring.profiles` 用于定义 profile 块已被弃用
- 需要更新所有 profile 配置块（dev, test, produce 等）

### 检查命令
```bash
# 搜索所有使用旧语法的地方
grep -r "spring:" --include="*.yml" --include="*.yaml" | grep "profiles:"
```

## 问题 2: MySQL 驱动类名过时

### 症状
```
Loading class `com.mysql.jdbc.Driver'. This is deprecated.
The new driver class is `com.mysql.cj.jdbc.Driver'
```

### 解决方案

**旧驱动**:
```yaml
driver-class-name: com.mysql.jdbc.Driver
```

**新驱动**:
```yaml
driver-class-name: com.mysql.cj.jdbc.Driver
```

## 问题 3: javax.* 到 jakarta.* 命名空间迁移

### 症状
```
package javax.servlet does not exist
package javax.persistence does not exist
```

### 解决方案
所有 `javax.*` 包名需要替换为 `jakarta.*`:

| 旧包名 (javax.*) | 新包名 (jakarta.*) |
|-----------------|-------------------|
| javax.servlet | jakarta.servlet |
| javax.persistence | jakarta.persistence |
| javax.validation | jakarta.validation |
| javax.annotation | jakarta.annotation |

### 批量替换命令
```bash
# 查找所有 javax 导入
find src -name "*.java" -exec grep -l "import javax\." {} \;

# 替换示例（需要手动验证）
sed -i 's/import javax\.servlet/import jakarta.servlet/g' $(find src -name "*.java")
```

## 问题 4: MyBatis Mapper 扫描配置缺失

### 症状
```
A component required a bean of type 'com.jfeat.**.dao.XxxMapper' that could not be found.
```

### 原因
主启动类缺少 `@MapperScan` 注解，导致 MyBatis 无法扫描到 Mapper 接口。

### 解决方案

在 `AmApplication.java` 中添加 `@MapperScan` 注解：

```java
package com.jfeat;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@MapperScan({
    "com.jfeat.am.module.menu.services.gen.persistence.dao",
    "com.jfeat.am.module.menu.services.domain.dao",
    "com.jfeat.crud.plus.service.dao"
})
public class AmApplication {
    // ...
}
```

### 注意事项
- 扫描路径需要包含所有 Mapper 接口所在的包
- 通常需要包含项目的 DAO 包和 `crud-plus` 的 DAO 包

## 问题 5: MySQL 驱动依赖缺失

### 症状
```
ClassNotFoundException: com.mysql.cj.jdbc.Driver
```

### 原因
pom.xml 中缺少 MySQL JDBC 驱动依赖。

### 解决方案

在 pom.xml 的 `<dependencies>` 中添加：

```xml
<!-- MySQL Driver -->
<dependency>
    <groupId>mysql</groupId>
    <artifactId>mysql-connector-java</artifactId>
</dependency>
```

**注意**: 版本由 `plat-parent` 管理，无需指定版本号。

## 问题 6: skipStandalone 配置

### 症状
```bash
java -jar target/app-21.0.0-standalone.jar
# Error: Unable to access jarfile
```

### 原因
pom.xml 中设置了 `<skipStandalone>true</skipStandalone>`，导致 standalone JAR 没有被打包。

### 解决方案

**方案 1: 修改 pom.xml**
```xml
<properties>
    <skipStandalone>false</skipStandalone>
</properties>
```

**方案 2: 构建时覆盖**
```bash
mvn clean package -DskipTests -DskipStandalone=false
```

## 问题 5: SpringDoc 替代 Swagger 2

### 症状
```
ClassNotFoundException: springfox.swagger2.annotations.Swagger2
```

### 解决方案
移除 Swagger 2 依赖，添加 SpringDoc OpenAPI:

```xml
<!-- 移除 -->
<dependency>
    <groupId>io.springfox</groupId>
    <artifactId>springfox-swagger2</artifactId>
</dependency>

<!-- 添加 -->
<dependency>
    <groupId>org.springdoc</groupId>
    <artifactId>springdoc-openapi-starter-webmvc-ui</artifactId>
    <version>2.7.0</version>
</dependency>
```

### 访问路径变更
- Swagger UI: `/swagger-ui/index.html` (旧: `/swagger-ui.html`)
- OpenAPI JSON: `/api-docs` (旧: `/v2/api-docs`)

## 升级检查清单

- [ ] 更新 `spring.profiles` 为 `spring.config.activate.on-profile`
- [ ] 更新 MySQL 驱动类名为 `com.mysql.cj.jdbc.Driver`
- [ ] 移除 `logging.config: classpath:logback-spring.xml` 配置
- [ ] 添加 MySQL 驱动依赖（如果缺失）
- [ ] 添加 `@MapperScan` 注解到启动类
- [ ] 添加 `mainClass` 配置到 spring-boot-maven-plugin
- [ ] 替换所有 `javax.*` 为 `jakarta.*`
- [ ] 移除 Swagger 2，添加 SpringDoc OpenAPI
- [ ] 更新 Java 版本到 17
- [ ] 检查 `skipStandalone` 配置
- [ ] 更新 FastJSON 为 FastJSON2
- [ ] 检查第三方库兼容性

## 参考项目

- **org** 项目: `/home/ubuntu/workspace/edu/auth/org/` - 已完成 Spring Boot 3.x 升级，可参考配置
- **menu** 项目: `/home/ubuntu/workspace/edu/auth/uaas/menu/` - 已修复上述问题

## 快速诊断命令

```bash
# 检查 profile 配置语法
grep -A 2 "^spring:" src/main/resources/application.yml | grep "profiles:"

# 检查 javax 导入
grep -r "import javax\." src/

# 检查 MySQL 驱动
grep -r "com.mysql.jdbc.Driver" src/

# 检查 logging.config 配置
grep -r "logging.config" config/ src/

# 检查 @MapperScan 注解
grep "@MapperScan" src/main/java/com/jfeat/AmApplication.java

# 检查 MySQL 驱动依赖
grep "mysql-connector" pom.xml

# 检查 Swagger 2 依赖
grep "springfox" pom.xml
```
