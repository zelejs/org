# Spring Boot 3.x 启动问题分析与修复报告

## 执行摘要

通过对比分析可以正常启动的 `org` 项目和无法启动的 `menu` 项目，识别并修复了 **6 个关键配置问题**，使 menu 项目成功启动。

---

## 问题对比表

| # | 问题 | org 项目 | menu 项目 (修复前) | 影响 |
|---|------|----------|-------------------|------|
| 1 | Profile 语法 | ✅ `spring.config.activate.on-profile` | ❌ `spring.profiles` | 启动失败 |
| 2 | MySQL 驱动类 | ✅ `com.mysql.cj.jdbc.Driver` | ❌ `com.mysql.jdbc.Driver` | 警告 |
| 3 | logging.config | ✅ 无配置 | ❌ `classpath:logback-spring.xml` | 启动失败 |
| 4 | MySQL 依赖 | ✅ 存在 | ❌ 缺失 | 启动失败 |
| 5 | mainClass 配置 | ✅ 已配置 | ❌ 缺失 | JAR 无法启动 |
| 6 | @MapperScan | ✅ 已配置 | ❌ 缺失 | Bean 创建失败 |

---

## 修复详情

### 1. Profile 配置语法 (Spring Boot 3.x 强制要求)

**文件**: `menu/src/main/resources/application.yml`

```diff
---
spring:
-  profiles: dev
+  config:
+    activate:
+      on-profile: dev
  datasource:
    ...
```

---

### 2. MySQL 驱动类名更新

**文件**: `menu/src/main/resources/application.yml`

```diff
- driver-class-name: com.mysql.jdbc.Driver
+ driver-class-name: com.mysql.cj.jdbc.Driver
```

---

### 3. 移除无效的 logging.config

**文件**: `menu/config/application-dev.yaml`

```diff
logging:
  level:
    root: INFO
    ...
-  config: classpath:logback-spring.xml
```

---

### 4. 添加 MySQL 驱动依赖

**文件**: `menu/pom.xml`

```xml
<!-- MySQL Driver -->
<dependency>
    <groupId>mysql</groupId>
    <artifactId>mysql-connector-java</artifactId>
</dependency>
```

---

### 5. 配置 mainClass

**文件**: `menu/pom.xml`

```xml
<plugin>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-maven-plugin</artifactId>
    <configuration>
+       <mainClass>com.jfeat.AmApplication</mainClass>
        <skip>${skipStandalone}</skip>
    </configuration>
```

---

### 6. 添加 @MapperScan 注解

**文件**: `menu/src/main/java/com/jfeat/AmApplication.java`

```java
+ import org.mybatis.spring.annotation.MapperScan;

@SpringBootApplication
+@MapperScan({
+    "com.jfeat.am.module.menu.services.gen.persistence.dao",
+    "com.jfeat.am.module.menu.services.domain.dao",
+    "com.jfeat.crud.plus.service.dao"
+})
public class AmApplication {
```

---

## 验证结果

修复后应用成功启动：

```
Started AmApplication in 7.441 seconds
Tomcat started on port 8080 (http) with context path '/'
Application run success!
```

---

## 文档更新

已更新以下文档以记录本次分析经验：

1. **TROUBLESHOOTING.md** - 详细的故障排除指南
2. **skill.md** - 更新了故障排除和预诊断命令
3. **MENU-FIX-SUMMARY.md** - 本次修复的详细记录
4. **ANALYSIS-SUMMARY.md** - 本报告

---

## 预诊断脚本

添加到 `skill.md` 的预诊断命令：

```bash
# 快速检查脚本
grep -r "^spring:" --include="*.yml" src/ config/ | grep "profiles:" | grep -v "active:"
grep -r "import javax\." src/ | grep -v "jakarta"
grep "mysql-connector" pom.xml || echo "WARNING: MySQL driver missing!"
grep "@MapperScan" src/main/java/com/jfeat/AmApplication.java || echo "WARNING: @MapperScan missing!"
grep -r "logging.config" config/ && echo "WARNING: logging.config may cause issues!"
grep "mainClass" pom.xml || echo "WARNING: mainClass not configured!"
```

---

## 经验教训

1. **对比分析是高效的问题诊断方法** - 通过对比可以正常工作的 org 项目，快速定位问题

2. **Spring Boot 3.x 的破坏性变更**：
   - Profile 配置语法变更
   - javax.* → jakarta.* 命名空间迁移
   - Swagger 2 → SpringDoc OpenAPI

3. **完整的启动配置检查清单**：
   - 依赖完整性（MySQL 驱动等）
   - 插件配置（mainClass）
   - 注解配置（@MapperScan）
   - 配置文件（logging.config, profile 语法）

---

## 日期

2026-03-31
