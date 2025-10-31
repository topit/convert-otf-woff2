# --- Variables ---
# Project name
TARGET_NAME := convert_otf_woff2

# Build artifacts
WASM_INPUT_PATH := target/wasm32-unknown-unknown/release/$(TARGET_NAME).wasm
OUT_DIR := pkg

# --- Targets ---

.PHONY: all build bindgen clean help install-target

all: bindgen

# Install the wasm32-unknown-unknown target if not already installed
install-target:
	@echo "Installing wasm32-unknown-unknown target..."
	@rustup target add wasm32-unknown-unknown

# Build the Rust code to a Wasm module
build: install-target
	@echo "Building Rust code to Wasm..."
	@cargo build --target wasm32-unknown-unknown --release

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
	@echo "  make all          - Build everything (default)."
	@echo "  make install-target - Install wasm32-unknown-unknown target."
	@echo "  make build        - Compile Rust code to Wasm."
	@echo "  make bindgen      - Generate web bindings from the Wasm file."
	@echo "  make clean        - Remove all build artifacts."
	@echo ""

