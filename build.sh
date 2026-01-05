#!/bin/bash
# Build wasmi-daisy for Daisy Seed (ARM Cortex-M7F)

set -e

echo "Building wasmi-daisy for Daisy Seed (thumbv7em-none-eabihf)..."

# Ensure target is installed
rustup target add thumbv7em-none-eabihf

# Build
cargo build --release

echo ""
echo "✅ Build complete!"
echo "Static library: target/thumbv7em-none-eabihf/release/libwasmi_daisy.a"
echo "Header file: wasmi_daisy.h"
echo ""
echo "To use in your C++ project, add to your Makefile:"
echo "  C_INCLUDES += -I/path/to/wasmi-daisy"
echo "  LDFLAGS += /path/to/wasmi-daisy/target/thumbv7em-none-eabihf/release/libwasmi_daisy.a"
