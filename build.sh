#!/usr/bin/env bash
# Meridian RS build script - configurable for core and web interface
# Usage: ./build.sh [OPTIONS]
#
# Options:
#   --core-branch BRANCH      Branch for core backend (default: master)
#   --web-branch BRANCH       Branch for web interface (default: web-ui)
#   --web-enabled true|false  Include web UI in build (default: false for core-only)
#   --output DIR              Output directory for artifacts (default: ./target/release)
#   --docker                  Build Docker image instead of native binary
#   --docker-tag TAG          Docker image tag (default: meridian:latest)
#   --help                    Show this help text

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default config
CORE_BRANCH="master"
WEB_BRANCH="web-ui"
WEB_ENABLED=false
OUTPUT_DIR="./target/release"
DOCKER_BUILD=false
DOCKER_TAG="meridian:latest"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${SCRIPT_DIR}"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --core-branch)
            CORE_BRANCH="$2"
            shift 2
            ;;
        --web-branch)
            WEB_BRANCH="$2"
            shift 2
            ;;
        --web-enabled)
            WEB_ENABLED="$2"
            shift 2
            ;;
        --output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --docker)
            DOCKER_BUILD=true
            shift
            ;;
        --docker-tag)
            DOCKER_TAG="$2"
            shift 2
            ;;
        --help)
            grep "^#" "$0" | grep -v "^#!/" | sed 's/^# //'
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Logging functions
log_info() {
    echo -e "${BLUE}→${NC} $1"
}

log_success() {
    echo -e "${GREEN}✓${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

log_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    if ! command -v git &> /dev/null; then
        log_error "git not found. Please install git."
        exit 1
    fi

    if [ "$DOCKER_BUILD" = true ]; then
        if ! command -v docker &> /dev/null; then
            log_error "docker not found. Please install docker."
            exit 1
        fi
        log_success "Docker found"
    else
        if ! command -v cargo &> /dev/null; then
            log_error "cargo not found. Please install Rust."
            exit 1
        fi
        log_success "Cargo found"
    fi
}

# Create temporary working directory for multi-branch builds
setup_build_workspace() {
    if [ "$WEB_ENABLED" = true ]; then
        log_info "Setting up workspace for multi-branch build..."

        TEMP_DIR=$(mktemp -d)
        trap "rm -rf $TEMP_DIR" EXIT

        # Copy core backend
        log_info "Checking out core from branch: $CORE_BRANCH"
        git clone --depth 1 --branch "$CORE_BRANCH" --single-branch \
            "$(git config --get remote.origin.url)" "$TEMP_DIR/core" 2>/dev/null || {
            log_error "Failed to checkout $CORE_BRANCH"
            exit 1
        }

        # Copy web interface
        log_info "Checking out web UI from branch: $WEB_BRANCH"
        git clone --depth 1 --branch "$WEB_BRANCH" --single-branch \
            "$(git config --get remote.origin.url)" "$TEMP_DIR/web" 2>/dev/null || {
            log_error "Failed to checkout $WEB_BRANCH"
            exit 1
        }

        # Copy web assets to core backend
        log_info "Integrating web UI with core..."
        mkdir -p "$TEMP_DIR/core/web-dist"

        # Build and copy web assets
        if [ -f "$TEMP_DIR/web/package.json" ]; then
            log_info "Building Next.js web UI..."
            cd "$TEMP_DIR/web"

            if ! command -v npm &> /dev/null && ! command -v pnpm &> /dev/null; then
                log_warn "npm/pnpm not found. Web UI will not be built."
                WEB_ENABLED=false
            else
                if command -v pnpm &> /dev/null; then
                    pnpm install
                    pnpm build
                else
                    npm install
                    npm run build
                fi

                # Copy Next.js build output
                if [ -d ".next" ]; then
                    cp -r .next "$TEMP_DIR/core/web-dist/"
                    cp -r public "$TEMP_DIR/core/web-dist/" 2>/dev/null || true
                    log_success "Web UI built and copied"
                fi
            fi
            cd - > /dev/null
        fi

        BUILD_DIR="$TEMP_DIR/core"
        log_success "Workspace ready"
    else
        log_info "Building core only from branch: $CORE_BRANCH"

        # Verify we're on the right branch
        CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
        if [ "$CURRENT_BRANCH" != "$CORE_BRANCH" ]; then
            log_warn "Currently on $CURRENT_BRANCH, switching to $CORE_BRANCH"
            git checkout "$CORE_BRANCH" 2>/dev/null || {
                log_error "Failed to checkout $CORE_BRANCH"
                exit 1
            }
        fi

        BUILD_DIR="$SCRIPT_DIR"
    fi
}

# Build native binary
build_native() {
    log_info "Building native binary..."

    cd "$BUILD_DIR"

    # Check Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        log_error "Cargo.toml not found in $BUILD_DIR"
        exit 1
    fi

    mkdir -p "$OUTPUT_DIR"

    if [ "$WEB_ENABLED" = true ]; then
        log_info "Building with web UI support..."
        # Set feature flag if needed for web assets
        cargo build --release --features "web-ui" 2>&1 || \
            cargo build --release 2>&1
    else
        cargo build --release 2>&1
    fi

    BINARY_NAME="meridian-rs"
    BINARY_PATH="$BUILD_DIR/target/release/$BINARY_NAME"

    if [ -f "$BINARY_PATH" ]; then
        log_success "Binary built: $BINARY_PATH"

        # Copy to output directory if different
        if [ "$OUTPUT_DIR" != "$BUILD_DIR/target/release" ]; then
            cp "$BINARY_PATH" "$OUTPUT_DIR/"
            log_success "Binary copied to: $OUTPUT_DIR/$BINARY_NAME"
        fi

        # Show size
        SIZE=$(du -h "$BINARY_PATH" | cut -f1)
        log_info "Binary size: $SIZE"
    else
        log_error "Build failed: binary not found"
        exit 1
    fi

    cd - > /dev/null
}

# Build Docker image
build_docker() {
    log_info "Building Docker image: $DOCKER_TAG"

    if [ ! -f "$SCRIPT_DIR/Dockerfile" ]; then
        log_error "Dockerfile not found in $SCRIPT_DIR"
        exit 1
    fi

    # Build with build args for branch selection
    docker build \
        --build-arg "CORE_BRANCH=$CORE_BRANCH" \
        --build-arg "WEB_BRANCH=$WEB_BRANCH" \
        --build-arg "WEB_ENABLED=$WEB_ENABLED" \
        -t "$DOCKER_TAG" \
        -f "$SCRIPT_DIR/Dockerfile" \
        "$SCRIPT_DIR" 2>&1

    if [ $? -eq 0 ]; then
        log_success "Docker image built: $DOCKER_TAG"

        # Show image size
        IMAGE_SIZE=$(docker images --format "table {{.Repository}}:{{.Tag}}\t{{.Size}}" | grep "$DOCKER_TAG" | awk '{print $NF}')
        log_info "Image size: $IMAGE_SIZE"
    else
        log_error "Docker build failed"
        exit 1
    fi
}

# Print build info
print_info() {
    echo ""
    log_info "Build Configuration:"
    echo "  Core branch:     $CORE_BRANCH"
    echo "  Web enabled:     $WEB_ENABLED"
    if [ "$WEB_ENABLED" = true ]; then
        echo "  Web branch:      $WEB_BRANCH"
    fi
    if [ "$DOCKER_BUILD" = true ]; then
        echo "  Build method:    Docker"
        echo "  Docker tag:      $DOCKER_TAG"
    else
        echo "  Build method:    Native"
        echo "  Output dir:      $OUTPUT_DIR"
    fi
    echo ""
}

# Main execution
main() {
    print_info
    check_prerequisites
    setup_build_workspace

    if [ "$DOCKER_BUILD" = true ]; then
        build_docker
    else
        build_native
    fi

    log_success "Build complete!"

    # Usage hints
    echo ""
    log_info "Next steps:"
    if [ "$DOCKER_BUILD" = true ]; then
        echo "  Run container: docker run -it --rm -p 3000:3000 $DOCKER_TAG"
    else
        echo "  Run binary:    $OUTPUT_DIR/meridian-rs"
        echo "  With config:   MERIDIAN_DATA_DIR=~/.meridian $OUTPUT_DIR/meridian-rs"
    fi
    echo ""
}

main
