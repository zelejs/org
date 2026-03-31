# Menu 项目启动问题修复总结

## 问题描述
menu 项目无法启动，而 org 项目可以正常启动。两个项目都是基于 Spring Boot 3.x 和 Java 17。

## 问题分析与解决方案

### 问题 1: Profile 配置语法错误 ❌
**错误信息**:
```
Property 'spring.profiles' is invalid and should be replaced with 'spring.config.activate.on-profile'
```

**原因**: Spring Boot 3.x 强制使用新的 profile 配置语法

**修复文件**: `src/main/resources/application.yml`

**修改内容**:
```yaml
# 旧语法
spring:
  profiles: dev

# 新语法
spring:
  config:
    activate:
      on-profile: dev
```

---

### 问题 2: MySQL 驱动类名过时 ❌
**错误信息**:
```
Loading class `com.mysql.jdbc.Driver'. This is deprecated.
```

**修复文件**: `src/main/resources/application.yml`

**修改内容**:
```yaml
# 旧驱动
driver-class-name: com.mysql.jdbc.Driver

# 新驱动
driver-class-name: com.mysql.cj.jdbc.Driver
```

---

### 问题 3: logging.config 配置错误 ❌
**错误信息**:
```
java.io.FileNotFoundException: class path resource [logback-spring.xml] cannot be resolved
```

**修复文件**: `config/application-dev.yaml`

**修改内容**: 删除以下行
```yaml
logging:
  config: classpath:logback-spring.xml  # 删除这一行
```

---

### 问题 4: MySQL 驱动依赖缺失 ❌
**错误信息**:
```
ClassNotFoundException: com.mysql.cj.jdbc.Driver
```

**修复文件**: `pom.xml`

**修改内容**: 添加依赖
```xml
<!-- MySQL Driver -->
<dependency>
    <groupId>mysql</groupId>
    <artifactId>mysql-connector-java</artifactId>
</dependency>
```

---

### 问题 5: mainClass 配置缺失 ❌
**症状**: standalone JAR 无法正确启动

**修复文件**: `pom.xml`

**修改内容**: 添加 mainClass 配置
```xml
<plugin>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-maven-plugin</artifactId>
    <configuration>
        <mainClass>com.jfeat.AmApplication</mainClass>
        <skip>${skipStandalone}</skip>
    </configuration>
```

---

### 问题 6: @MapperScan 注解缺失 ❌
**错误信息**:
```
A component required a bean of type 'com.jfeat.**.dao.XxxMapper' that could not be found.
```

**修复文件**: `src/main/java/com/jfeat/AmApplication.java`

**修改内容**: 添加注解
```java
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

---

## 对比分析: org vs menu

| 配置项 | org 项目 | menu 项目（修复前） | 状态 |
|--------|----------|---------------------|------|
| Profile 语法 | `spring.config.activate.on-profile` | `spring.profiles` | ❌ 已修复 |
| MySQL 驱动类 | `com.mysql.cj.jdbc.Driver` | `com.mysql.jdbc.Driver` | ❌ 已修复 |
| logging.config | 无配置 | `classpath:logback-spring.xml` | ❌ 已修复 |
| MySQL 依赖 | 存在 | 缺失 | ❌ 已修复 |
| mainClass | 已配置 | 缺失 | ❌ 已修复 |
| @MapperScan | 已配置 | 缺失 | ❌ 已修复 |

## 验证结果

修复后应用成功启动：
```
Started AmApplication in 7.441 seconds
Tomcat started on port 8080 (http) with context path '/'
Application run success!
```

## 经验总结

1. **Spring Boot 3.x 升级要点**:
   - Profile 配置必须使用新语法
   - javax.* 需要迁移到 jakarta.*
   - 旧版 Swagger 需要替换为 SpringDoc

2. **依赖配置检查清单**:
   - 确保所有必需的数据库驱动依赖已添加
   - 检查 spring-boot-maven-plugin 的 mainClass 配置
   - 验证 @MapperScan 扫描路径正确

3. **配置文件检查清单**:
   - 移除不存在的 logging.config 配置
   - 使用正确的 MySQL 驱动类名
   - 使用新的 profile 配置语法

## 相关文档

- `/home/ubuntu/workspace/edu/auth/org/skills/java17-app-start/TROUBLESHOOTING.md`
- `/home/ubuntu/workspace/edu/auth/org/skills/java17-app-start/skill.md`
