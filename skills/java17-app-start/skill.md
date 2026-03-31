---
description: Start Java 17 Spring Boot application (UaaS)
arguments:
  - name: profile
    description: Spring profile: dev (default), test, or produce
    required: false
  - name: mode
    description: Start mode: jar (standalone), maven, or check
    required: false
---

# Java 17 Spring Boot Application Starter

Start Java 17 Spring Boot application with health check and endpoint verification.

## Application Location

```
/home/ubuntu/workspace/edu/auth/uaas/uaas/
├── target/
│   ├── uaas-21.0.0.jar           # Standard JAR
│   └── uaas-21.0.0-standalone.jar # Executable JAR with dependencies
├── config/
│   └── application-dev.yaml      # Dev environment configuration
├── src/main/resources/
│   └── application.yml           # Main configuration
└── pom.xml                       # Maven configuration
```

## Service Endpoints

| Endpoint | Description | Auth |
|----------|-------------|------|
| `GET /` | Root endpoint | None |
| `GET /swagger-ui/index.html` | Swagger UI | None |
| `GET /api-docs` | OpenAPI JSON | None |
| `GET /actuator/health` | Health check | None |

## Prerequisites

- Java 17+ (OpenJDK or Oracle)
- Maven 3.6+
- MySQL database (configured in profile)
- Redis (configured in profile)

## Modes

### `jar` (default)
Start using standalone executable JAR:
```bash
cd /home/ubuntu/workspace/edu/auth/uaas/uaas
java -jar target/uaas-21.0.0-standalone.jar --spring.profiles.active=dev
```

### `maven`
Start using Maven Spring Boot plugin:
```bash
cd /home/ubuntu/workspace/edu/auth/uaas/uaas
mvn spring-boot:run -Dspring-boot.run.profiles=dev
```

### `check`
Check if service is running and healthy:
```bash
curl http://localhost:8080/actuator/health
# or
curl http://localhost:8080/
```

## Profiles

### `dev` (default)
Development environment with local database and Redis.

### `test`
Test environment with H2 in-memory database.

### `produce`
Production environment configuration.

## Execution Steps

1. Check if service already running (port 8080)
2. Kill existing process if found
3. Start application in selected mode
4. Wait for Spring Boot startup (typically 15-30 seconds)
5. Verify health endpoint responds
6. Display service URLs and status

## Service URLs

- Application: `http://localhost:8080/`
- Swagger UI: `http://localhost:8080/swagger-ui/index.html`
- API Docs: `http://localhost:8080/api-docs`
- Health: `http://localhost:8080/actuator/health`

## Environment

```bash
JAVA_VERSION=17
SPRING_BOOT_VERSION=3.4.2
SERVER_PORT=8080
```

## Configuration Notes

**Important**: Ensure `config/application-{profile}.yaml` does NOT contain:
```yaml
logging:
  config: classpath:logback-spring.xml  # REMOVE THIS LINE
```

This causes startup failure if logback-spring.xml doesn't exist. Let Spring Boot use default logging configuration.

## Common Commands

```bash
# Start with dev profile
java -jar target/uaas-21.0.0-standalone.jar --spring.profiles.active=dev

# Start with Maven
mvn spring-boot:run -Dspring-boot.run.profiles=dev

# Build standalone JAR
mvn clean package -DskipTests

# Check if running
curl http://localhost:8080/actuator/health

# Stop application
pkill -f "uaas-21.0.0-standalone"
```

## Troubleshooting

### Spring Boot 3.x Profile Configuration Error (Critical)
```
org.springframework.boot.context.config.InvalidConfigDataPropertyException:
Property 'spring.profiles' imported from location 'class path resource [application.yml]'
is invalid and should be replaced with 'spring.config.activate.on-profile'
```

**Cause**: In Spring Boot 3.x, the old `spring.profiles` syntax is deprecated.

**Old syntax (Spring Boot 2.x)**:
```yaml
---
spring:
  profiles: dev
  datasource:
    url: jdbc:mysql://...
```

**New syntax (Spring Boot 3.x)**:
```yaml
---
spring:
  config:
    activate:
      on-profile: dev
  datasource:
    url: jdbc:mysql://...
```

**Solution**: Update all `application.yml` profile sections to use `spring.config.activate.on-profile`.

**Files to check**:
- `src/main/resources/application.yml`
- `config/application-*.yaml`

### Logback Configuration Error
```
java.io.FileNotFoundException: class path resource [logback-spring.xml] cannot be resolved
```
**Solution**: Remove `logging.config: classpath:logback-spring.xml` from config files.

### Port Already in Use
```bash
# Find and kill process on port 8080
lsof -ti:8080 | xargs kill -9
```

### Bean Validation Provider Warning
```
Unable to create a Configuration, because no Jakarta Bean Validation provider could be found
```
This is a warning, not an error. Application will start successfully. To fix, add Hibernate Validator to pom.xml.

### MyBatis Mapper Not Found Error
```
A component required a bean of type 'com.jfeat.**.dao.XxxMapper' that could not be found.
```
**Solution**: Add `@MapperScan` annotation to main application class:
```java
@SpringBootApplication
@MapperScan({
    "com.jfeat.**.persistence.dao",
    "com.jfeat.**.domain.dao",
    "com.jfeat.crud.plus.service.dao"
})
public class AmApplication {
    // ...
}
```

### MySQL Driver Not Found Error
```
ClassNotFoundException: com.mysql.cj.jdbc.Driver
```
**Solution**: Add MySQL driver dependency to pom.xml:
```xml
<dependency>
    <groupId>mysql</groupId>
    <artifactId>mysql-connector-java</artifactId>
</dependency>
```

## Pre-flight Diagnostics

Run these commands before starting to catch common issues:

```bash
# Check profile syntax (should be empty if correct)
grep -r "^spring:" --include="*.yml" src/ config/ | grep "profiles:" | grep -v "active:"

# Check for javax imports (should be empty for Spring Boot 3.x)
grep -r "import javax\." src/ | grep -v "jakarta"

# Check for MySQL driver dependency
grep "mysql-connector" pom.xml || echo "WARNING: MySQL driver dependency missing!"

# Check @MapperScan annotation
grep "@MapperScan" src/main/java/com/jfeat/AmApplication.java || echo "WARNING: @MapperScan missing!"

# Check for problematic logging.config
grep -r "logging.config" config/ && echo "WARNING: logging.config may cause issues!"

# Check mainClass configuration
grep "mainClass" pom.xml || echo "WARNING: mainClass not configured!"
```

## Startup Success Indicators

- Log: `Started AmApplication in X.XXX seconds`
- Log: `Tomcat started on port 8080`
- Log: `UAAS-PERM running success!`
- HTTP: `curl http://localhost:8080/` returns response
