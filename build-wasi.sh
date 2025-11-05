#!/bin/bash

# Build script for WASI target with woff crate support

set -e

echo "Building convert-otf-woff2 for WASI target..."

# Check if WASI SDK is installed
if [ ! -d "/tmp/wasi-sdk-24.0-x86_64-linux" ]; then
    echo "WASI SDK not found. Downloading..."
    cd /tmp
    wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-24/wasi-sdk-24.0-x86_64-linux.tar.gz
    tar -xzf wasi-sdk-24.0-x86_64-linux.tar.gz
    cd -
fi

export WASI_SDK_PATH=/tmp/wasi-sdk-24.0-x86_64-linux
export CC_wasm32_wasip1="${WASI_SDK_PATH}/bin/clang --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
export CXX_wasm32_wasip1="${WASI_SDK_PATH}/bin/clang++ --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
export AR_wasm32_wasip1="${WASI_SDK_PATH}/bin/llvm-ar"
export CARGO_TARGET_WASM32_WASIP1_RUSTFLAGS="-L ${WASI_SDK_PATH}/share/wasi-sysroot/lib/wasm32-wasip1 -l c++"

# Add WASI target if not already added
rustup target add wasm32-wasip1

# Build
cargo build --target wasm32-wasip1 --release

echo "✅ Build complete: target/wasm32-wasip1/release/convert_otf_woff2.wasm"
echo "Size: $(ls -lh target/wasm32-wasip1/release/convert_otf_woff2.wasm | awk '{print $5}')"
