# Java 17 Application Startup Fix Summary

## Parent POM: plat-parent

This project now uses `plat-parent` as the parent POM, which provides:

- Spring Boot 3.4.0
- Java 17
- Lombok 1.18.36
- Fastjson2 2.0.53
- MySQL Connector 8.0.28

### Benefits of plat-parent

1. **No need to redefine** common properties like `spring-boot.version`, `lombok.version`
2. **No need for BOM** - Spring Boot BOM is already included
3. **Consistent versions** across all internal projects
4. **Simpler pom.xml** - remove redundant dependencyManagement

---

## Problem
UaaS module failed to start with standalone JAR, while org module started successfully.

## Root Cause
The `config/application-dev.yaml` file contained:
```yaml
logging:
  config: classpath:logback-spring.xml
```

But the `logback-spring.xml` file did not exist, causing startup failure:
```
java.io.FileNotFoundException: class path resource [logback-spring.xml] cannot be resolved
```

## Solution
Removed the `logging.config` line from `/home/ubuntu/workspace/edu/auth/uaas/uaas/config/application-dev.yaml` to let Spring Boot use default logging configuration.

## Comparison with org Module
The org module's `config/application-dev.yaml` does NOT have the `logging.config` setting, which allows it to use Spring Boot's default logging and start successfully.

## Files Modified
- `/home/ubuntu/workspace/edu/auth/uaas/uaas/config/application-dev.yaml` - Removed `logging.config: classpath:logback-spring.xml`

## Verification
```bash
# Before fix - startup fails with FileNotFoundException
java -jar target/uaas-21.0.0-standalone.jar --spring.profiles.active=dev

# After fix - startup succeeds
java -jar target/uaas-21.0.0-standalone.jar --spring.profiles.active=dev
# Output: Started AmApplication in 12.32 seconds
#         Tomcat started on port 8080 (http)
#         UAAS-PERM running success!
```

## Skill Created
A new skill `java17-start` has been created at:
```
/home/ubuntu/workspace/edu/auth/uaas/uaas/.claude/skills/java17-start/
├── skill.md              # Skill documentation
├── prompt.md             # AI prompt template
├── java17-start.sh       # Executable startup script
└── README.md            # This file
```

## Usage
```bash
# Start with dev profile (default)
./.claude/skills/java17-start/java17-start.sh dev jar

# Start with maven mode
./.claude/skills/java17-start/java17-start.sh dev maven

# Check status
./.claude/skills/java17-start/java17-start.sh dev check
```

## Service URLs After Startup
- Application: http://localhost:8080/
- Swagger UI: http://localhost:8080/swagger-ui/index.html
- API Docs: http://localhost:8080/api-docs
