#!/bin/bash

echo "🚀 Setting up Qt GUI development environment..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Start Xvfb for GUI testing
print_info "Starting virtual display server..."
Xvfb :99 -screen 0 1920x1080x24 -ac +extension GLX +render -noreset &
XVFB_PID=$!
sleep 2

if kill -0 $XVFB_PID 2>/dev/null; then
    print_status "Virtual display started (DISPLAY=:99)"
    echo $XVFB_PID > /tmp/xvfb.pid
else
    print_warning "Failed to start virtual display"
fi

# Verify Qt installation
print_info "Verifying Qt installation..."

if command -v qmake6 &> /dev/null; then
    QT_VERSION=$(qmake6 -version | grep "Qt version" | cut -d' ' -f4)
    print_status "Qt $QT_VERSION found"
else
    print_warning "qmake6 not found"
fi

# Check available Qt modules
print_info "Available Qt modules:"
for module in Core Widgets Gui Test Quick Qml Svg Multimedia Network; do
    if pkg-config --exists Qt6${module} 2>/dev/null; then
        VERSION=$(pkg-config --modversion Qt6${module})
        echo "  ✓ Qt6${module} ($VERSION)"
    else
        echo "  ✗ Qt6${module} (not available)"
    fi
done
