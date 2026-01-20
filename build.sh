#!/bin/bash
# Build wasmi-daisy for native platform

set -e

echo "Building wasmi-daisy for native platform..."

# Get the host target triple
HOST_TARGET=$(rustc -vV | grep host | cut -d ' ' -f2)
echo "Building for host target: $HOST_TARGET"

# Build for native host, overriding the .cargo/config.toml default
cargo build --release --target $HOST_TARGET

echo ""
echo "✅ Build complete!"
echo "Static library: target/$HOST_TARGET/release/libwasmi_daisy.a"
echo "Header file: wasmi_daisy.h"
echo ""
echo "To use in your C++ project, add to your Makefile:"
echo "  C_INCLUDES += -I/path/to/wasmi-daisy"
echo "  LDFLAGS += /path/to/wasmi-daisy/target/$HOST_TARGET/release/libwasmi_daisy.a"
