# Java 17 Spring Boot Application Starter

You are a Java 17 Spring Boot application starter. Your task is to start the UaaS application and verify it's running correctly.

## Parameters
- profile: {{profile | default: "dev"}}
- mode: {{mode | default: "jar"}}

## Working Directory
/home/ubuntu/workspace/edu/auth/uaas/uaas

## Steps

1. **Check for existing process**
   ```bash
   lsof -ti:8080 2>/dev/null
   ```
   If port 8080 is in use, kill the existing process:
   ```bash
   pkill -f "uaas-21.0.0-standalone" || true
   sleep 2
   ```

2. **Build standalone JAR if needed** (if mode is "jar" and JAR doesn't exist)
   ```bash
   if [ ! -f target/uaas-21.0.0-standalone.jar ]; then
       mvn clean package -DskipTests
   fi
   ```

3. **Start application** based on mode:

   **For "jar" mode:**
   ```bash
   java -jar target/uaas-21.0.0-standalone.jar --spring.profiles.active={{profile}} &
   ```

   **For "maven" mode:**
   ```bash
   mvn spring-boot:run -Dspring-boot.run.profiles={{profile}} &
   ```

4. **Wait for startup** (monitor logs)
   ```bash
   # Wait for up to 60 seconds for startup
   for i in {1..60}; do
       if curl -s http://localhost:8080/actuator/health >/dev/null 2>&1; then
           echo "Application started successfully!"
           break
       fi
       sleep 1
   done
   ```

5. **Verify health**
   ```bash
   curl -s http://localhost:8080/actuator/health | jq . || curl -s http://localhost:8080/
   ```

6. **Display service information**
   - Application URL: http://localhost:8080/
   - Swagger UI: http://localhost:8080/swagger-ui/index.html
   - API Docs: http://localhost:8080/api-docs

## Success Criteria
- Process is running
- HTTP endpoint responds
- No critical errors in logs

## Error Handling
- If logback-spring.xml error: Remove `logging.config` from config/application-dev.yaml
- If port conflict: Kill existing process
- If build fails: Check Maven dependencies and Java version
