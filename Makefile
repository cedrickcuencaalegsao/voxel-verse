PROJECT := voxel-verse

.PHONY: run dev check build release clean fmt fmt-check clippy test watch

# Run the project
run:
	cargo run

# Run in development mode with automatic restart on file changes
dev:
# 	cargo watch -x run
	cargo watch -s "pkill your_app_name; cargo run"

# Check the project without building an executable
check:
	cargo check

# Build debug version
build:
	cargo build

# Build optimized release version
release:
	cargo build --release

# Format Rust code
fmt:
	cargo fmt

# Check formatting without modifying files
fmt-check:
	cargo fmt -- --check

# Run Clippy
clippy:
	cargo clippy --all-targets --all-features

# Run tests
test:
	cargo test

# Automatically restart when Rust files change
watch:
	cargo watch -x run

# Remove build artifacts
clean:
	cargo clean
