#!/bin/bash
# Build script for org-rust project

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BINARY_NAME="org-cli"
OUTPUT_DIR="target/release"
FEATURES=""

# Parse arguments
BUILD_TYPE="release"
RUN_CLI=false
RUN_SERVER=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            BUILD_TYPE="debug"
            OUTPUT_DIR="target/debug"
            shift
            ;;
        --run-cli)
            RUN_CLI=true
            shift
            ;;
        --run-server)
            RUN_SERVER=true
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --debug       Build debug version (default: release)"
            echo "  --run-cli     Build and run CLI mode"
            echo "  --run-server  Build and run server mode"
            echo "  --help        Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}=====================================${NC}"
echo -e "${BLUE}  org-rust Build Script${NC}"
echo -e "${BLUE}=====================================${NC}"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed${NC}"
    exit 1
fi

# Display build info
echo -e "${YELLOW}Build type:${NC} $BUILD_TYPE"
echo -e "${YELLOW}Output directory:${NC} $OUTPUT_DIR"
echo ""

# Build
echo -e "${BLUE}Building...${NC}"
if [ "$BUILD_TYPE" = "release" ]; then
    cargo build --release
else
    cargo build
fi

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build successful!${NC}"
    echo ""
    echo -e "${YELLOW}Binary location:${NC} $OUTPUT_DIR/$BINARY_NAME"

    # Create a symlink for easier access
    ln -sf "$OUTPUT_DIR/$BINARY_NAME" "$BINARY_NAME" 2>/dev/null || true
    echo -e "${YELLOW}Symlink:${NC} ./$BINARY_NAME"
    echo ""

    # Run if requested
    if [ "$RUN_CLI" = true ]; then
        echo -e "${BLUE}Running CLI...${NC}"
        echo ""
        "$OUTPUT_DIR/$BINARY_NAME" --help
    elif [ "$RUN_SERVER" = true ]; then
        echo -e "${BLUE}Running server...${NC}"
        echo ""
        "$OUTPUT_DIR/$BINARY_NAME"
    else
        echo -e "${GREEN}To run the CLI:${NC}"
        echo "  $OUTPUT_DIR/$BINARY_NAME org --help"
        echo ""
        echo -e "${GREEN}To run the server:${NC}"
        echo "  $OUTPUT_DIR/$BINARY_NAME"
    fi
else
    echo -e "${RED}✗ Build failed!${NC}"
    exit 1
fi
