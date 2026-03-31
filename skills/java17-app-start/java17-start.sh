#!/bin/bash
# Java 17 Spring Boot Application Starter Script

set -e

# Default values
PROFILE="${1:-dev}"
MODE="${2:-jar}"
APP_DIR="/home/ubuntu/workspace/edu/auth/uaas/uaas"
APP_NAME="uaas-21.0.0-standalone"
SERVER_PORT=8080
MAX_WAIT=60

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Change to app directory
cd "$APP_DIR" || exit 1

# Check for existing process
check_existing() {
    local pid=$(lsof -ti:$SERVER_PORT 2>/dev/null)
    if [ -n "$pid" ]; then
        log_warn "Port $SERVER_PORT is in use (PID: $pid)"
        log_info "Stopping existing process..."
        pkill -f "$APP_NAME" || kill -9 "$pid" 2>/dev/null || true
        sleep 2
    fi
}

# Build JAR if needed
build_jar() {
    if [ ! -f "target/$APP_NAME.jar" ]; then
        log_info "Building standalone JAR..."
        mvn clean package -DskipTests -q
    fi
}

# Start application
start_app() {
    case "$MODE" in
        jar)
            build_jar
            log_info "Starting with JAR mode (profile: $PROFILE)..."
            java -jar "target/$APP_NAME.jar" --spring.profiles.active="$PROFILE" &
            APP_PID=$!
            ;;
        maven)
            log_info "Starting with Maven mode (profile: $PROFILE)..."
            mvn spring-boot:run -Dspring-boot.run.profiles="$PROFILE" -q &
            APP_PID=$!
            ;;
        check)
            log_info "Checking application status..."
            check_health
            exit $?
            ;;
        *)
            log_error "Unknown mode: $MODE"
            exit 1
            ;;
    esac

    echo $APP_PID > /tmp/uaas-app.pid
    log_info "Application started with PID: $APP_PID"
}

# Wait for startup
wait_for_startup() {
    log_info "Waiting for application to start..."

    for i in $(seq 1 $MAX_WAIT); do
        if curl -s "http://localhost:$SERVER_PORT/" >/dev/null 2>&1; then
            log_info "Application started successfully! (${i}s)"
            return 0
        fi
        sleep 1
        echo -n "."
    done
    echo

    log_error "Application failed to start within ${MAX_WAIT}s"
    return 1
}

# Check health
check_health() {
    local response=$(curl -s "http://localhost:$SERVER_PORT/" 2>/dev/null)

    if [ $? -eq 0 ]; then
        log_info "Application is responding"
        echo ""
        echo "=================================="
        echo "Application URLs:"
        echo "  - App:    http://localhost:$SERVER_PORT/"
        echo "  - Swagger: http://localhost:$SERVER_PORT/swagger-ui/index.html"
        echo "  - API:    http://localhost:$SERVER_PORT/api-docs"
        echo "=================================="
        return 0
    else
        log_error "Application is not responding"
        return 1
    fi
}

# Main execution
main() {
    log_info "Java 17 Spring Boot Application Starter"
    log_info "Profile: $PROFILE, Mode: $MODE"

    check_existing

    if [ "$MODE" != "check" ]; then
        start_app
        sleep 5
        wait_for_startup
        check_health
    fi
}

# Handle interruption
trap 'log_info "Stopping..."; pkill -f "$APP_NAME" 2>/dev/null || true; exit 0' INT TERM

main
