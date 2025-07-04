# --- Variables ---
# Project name
TARGET_NAME := convert_otf_woff2

# Build artifacts
WASM_INPUT_PATH := target/wasm32-wasip1/release/$(TARGET_NAME).wasm
OUT_DIR := pkg

# Tool paths (adjust if your installation paths are different)
# Homebrew on Apple Silicon default path
HOMEBREW_PREFIX ?= /opt/homebrew
WASI_SDK_PATH ?= /Users/lizihan/wasi-sdk/wasi-sdk-25.0-arm64-macos/
BINARYEN_PATH ?= /Users/lizihan/binaryen/binaryen-version_123

# Environment variables for build
export WASI_SYSROOT := $(WASI_SDK_PATH)/share/wasi-sysroot
export PATH := $(PATH):$(BINARYEN_PATH)/bin

# --- Targets ---

.PHONY: all build bindgen clean help

all: bindgen

# Build the Rust code to a Wasm module
build:
	@echo "Building Rust code to Wasm..."
	@cargo build --target wasm32-wasip1 --release

# Run wasm-bindgen to generate web-compatible files
bindgen: build
	@echo "Running wasm-bindgen..."
	@wasm-bindgen $(WASM_INPUT_PATH) --out-dir $(OUT_DIR) --target web

# Clean up build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf $(OUT_DIR)

# Display help
help:
	@echo "Makefile for building the Wasm project"
	@echo ""
	@echo "Usage:"
	@echo "  make all       - Build everything (default)."
	@echo "  make build     - Compile Rust code to Wasm."
	@echo "  make bindgen   - Generate web bindings from the Wasm file."
	@echo "  make clean     - Remove all build artifacts."
	@echo ""

