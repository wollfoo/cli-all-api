# =============================================================================
# ProxyPal Build Makefile
# Makefile cho build automation - quản lý builds Linux và Windows
# =============================================================================

.PHONY: help build-linux build-windows build-all clean dist-clean install-deps check-deps

# Đường dẫn mặc định
DIST_DIR := dist
SCRIPTS_DIR := scripts

# -----------------------------------------------------------------------------
# Help - Hiển thị danh sách lệnh
# -----------------------------------------------------------------------------
help:
	@echo "ProxyPal Build System"
	@echo "===================="
	@echo ""
	@echo "Available targets:"
	@echo "  make build-linux    - Build Linux binary (x86_64-unknown-linux-gnu)"
	@echo "  make build-windows  - Build Windows binary (x86_64-pc-windows-gnu)"
	@echo "  make build-all      - Build cả Linux và Windows"
	@echo "  make clean          - Xóa dist/ directory"
	@echo "  make dist-clean     - Xóa dist/ và target/ directories"
	@echo "  make install-deps   - Cài đặt cross-compilation dependencies"
	@echo "  make check-deps     - Kiểm tra dependencies đã cài đặt"
	@echo ""
	@echo "Output directories:"
	@echo "  Linux:   dist/linux-x86_64/proxypal"
	@echo "  Windows: dist/windows-x86_64/proxypal.exe"

# -----------------------------------------------------------------------------
# Build Targets
# -----------------------------------------------------------------------------

## Build Linux binary
build-linux:
	@echo "Building Linux binary..."
	@chmod +x $(SCRIPTS_DIR)/build-linux.sh
	@$(SCRIPTS_DIR)/build-linux.sh

## Build Windows binary (cross-compile)
build-windows:
	@echo "Building Windows binary..."
	@chmod +x $(SCRIPTS_DIR)/build-windows.sh
	@$(SCRIPTS_DIR)/build-windows.sh

## Build tất cả platforms
build-all: build-linux build-windows
	@echo ""
	@echo "============================================"
	@echo "  All builds completed!"
	@echo "============================================"
	@echo ""
	@ls -la $(DIST_DIR)/*/

# -----------------------------------------------------------------------------
# Clean Targets
# -----------------------------------------------------------------------------

## Xóa dist/ directory
clean:
	@echo "Cleaning dist/ directory..."
	@rm -rf $(DIST_DIR)
	@echo "Done."

## Xóa cả dist/ và target/ directories
dist-clean: clean
	@echo "Cleaning target/ directories..."
	@rm -rf src-headless/target
	@echo "Done."

# -----------------------------------------------------------------------------
# Dependency Management
# -----------------------------------------------------------------------------

## Cài đặt cross-compilation dependencies (Ubuntu/Debian)
install-deps:
	@echo "Installing cross-compilation dependencies..."
	@echo "This requires sudo privileges."
	@sudo apt-get update
	@sudo apt-get install -y mingw-w64
	@rustup target add x86_64-unknown-linux-gnu
	@rustup target add x86_64-pc-windows-gnu
	@echo "Done. Dependencies installed."

## Kiểm tra dependencies
check-deps:
	@echo "Checking dependencies..."
	@echo ""
	@echo "Rust toolchain:"
	@rustc --version
	@cargo --version
	@echo ""
	@echo "Installed targets:"
	@rustup target list --installed
	@echo ""
	@echo "mingw-w64:"
	@which x86_64-w64-mingw32-gcc 2>/dev/null && echo "✓ mingw-w64 installed" || echo "✗ mingw-w64 NOT installed (required for Windows builds)"
	@echo ""
