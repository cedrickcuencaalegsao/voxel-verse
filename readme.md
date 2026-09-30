# VoxelVerse

A cross-platform voxel game built with **Rust** and **Bevy**.

VoxelVerse is designed to run on:

* Windows
* Linux
* macOS

## Tech Stack

* **Rust** — Edition 2024
* **Bevy** — 0.17
* **Noise** — procedural terrain generation
* **Serde** — serialization and deserialization
* **Bincode** — binary data serialization
* **Rayon** — parallel processing

---

# Requirements

Before building VoxelVerse, install the following:

| Requirement          | Purpose                               |
| -------------------- | ------------------------------------- |
| Rust                 | Programming language and compiler     |
| Cargo                | Rust package manager and build system |
| Git                  | Clone and manage the repository       |
| Platform build tools | Native compilation and linking        |

---

# Installation

## 1. Install Rust

VoxelVerse uses the stable Rust toolchain.

The recommended installation method is **rustup**.

Official Rust installation:

[Rust / rustup](https://rustup.rs/?utm_source=chatgpt.com)

### Windows

Open **PowerShell** and run:

```powershell
winget install Rustlang.Rustup
```

Alternatively, download `rustup-init.exe` from the official Rust website.

After installation, restart your terminal.

Verify:

```powershell
rustc --version
cargo --version
rustup --version
```

### Linux

Open a terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Select the default installation when prompted.

Then load Cargo:

```bash
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version
cargo --version
rustup --version
```

### macOS

Open Terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then:

```bash
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version
cargo --version
rustup --version
```

---

# 2. Install Platform Build Tools

Because Bevy uses native graphics and windowing functionality, your operating system needs its native development tools.

## Windows

Install **Visual Studio Build Tools**.

Make sure the following workload is installed:

```text
Desktop development with C++
```

Also install the Windows SDK.

[Visual Studio Downloads](https://visualstudio.microsoft.com/downloads/?utm_source=chatgpt.com)

After installation, restart your terminal.

---

## Linux

For Ubuntu/Debian:

```bash
sudo apt update
sudo apt install build-essential pkg-config
```

For Fedora:

```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install pkg-config
```

For Arch Linux:

```bash
sudo pacman -S base-devel pkg-config
```

Depending on your Linux distribution and graphics configuration, additional system libraries may be required by Bevy.

---

## macOS

Install Apple's Command Line Tools:

```bash
xcode-select --install
```

Verify:

```bash
xcode-select -p
```

If you have the full Xcode installation, you can also verify:

```bash
xcodebuild -version
```

---

# 3. Clone VoxelVerse

Clone the official repository:

```bash
git clone https://github.com/cedrickcuencaalegsao/voxel-verse.git
```

Enter the project:

```bash
cd voxel-verse
```

You should see:

```text
voxel-verse/
├── Cargo.toml
├── Cargo.lock
├── src/
└── .gitignore
```

---

# 4. Verify the Rust Toolchain

Run:

```bash
rustc --version
```

```bash
cargo --version
```

```bash
rustup show
```

VoxelVerse uses **Rust Edition 2024**.

The project dependencies are managed through `Cargo.toml` and the exact dependency resolution is recorded in `Cargo.lock`.

---

# 5. Build VoxelVerse

From the project directory:

```bash
cargo build
```

Cargo will automatically download and compile the project's dependencies.

This may take some time during the first build because Bevy and its dependencies are relatively large.

---

# 6. Run VoxelVerse

Run the game with:

```bash
cargo run
```

Cargo will compile the project and launch VoxelVerse.

For development, this is the recommended command:

```bash
cargo run
```

---

# 7. Run the Release Build

For optimized performance:

```bash
cargo run --release
```

Or build the optimized executable first:

```bash
cargo build --release
```

The executable will be generated inside:

```text
target/release/
```

### Windows

```text
target\release\voxel-verse.exe
```

### Linux

```text
target/release/voxel-verse
```

### macOS

```text
target/release/voxel-verse
```

---

# Windows

Complete Windows setup:

```powershell
git clone https://github.com/cedrickcuencaalegsao/voxel-verse.git
cd voxel-verse
cargo build
cargo run
```

For the optimized version:

```powershell
cargo run --release
```

---

# Linux

Complete Linux setup:

```bash
git clone https://github.com/cedrickcuencaalegsao/voxel-verse.git
cd voxel-verse
cargo build
cargo run
```

For the optimized version:

```bash
cargo run --release
```

---

# macOS

Complete macOS setup:

```bash
git clone https://github.com/cedrickcuencaalegsao/voxel-verse.git
cd voxel-verse
cargo build
cargo run
```

For the optimized version:

```bash
cargo run --release
```

---

# Development Commands

## Check the project

Use `cargo check` to verify the project without producing the final executable:

```bash
cargo check
```

This is usually faster than a complete build.

---

## Format the code

```bash
cargo fmt
```

Check formatting without changing files:

```bash
cargo fmt -- --check
```

---

## Run Clippy

```bash
cargo clippy
```

For stricter checking:

```bash
cargo clippy -- -D warnings
```

---

## Run Tests

```bash
cargo test
```

---

## Clean the Build

If you encounter strange build or dependency problems:

```bash
cargo clean
```

Then rebuild:

```bash
cargo build
```

---

# Project Dependencies

VoxelVerse currently uses the following Rust dependencies:

```toml
[dependencies]
bevy = { version = "0.17", features = [
    "bevy_core_pipeline",
    "bevy_render",
    "bevy_pbr",
    "bevy_winit",
    "bevy_window",
    "png",
] }

noise = "0.9"
serde = { version = "1", features = ["derive"] }
bincode = "1"
rayon = "1"
```

## Bevy

Bevy provides the game engine functionality, including:

* Rendering
* 3D graphics
* Window management
* Input
* ECS
* PBR rendering
* Game systems

## Noise

The `noise` crate is used for procedural generation and terrain-related systems.

## Serde

Serde provides serialization and deserialization support.

## Bincode

Bincode provides compact binary serialization.

## Rayon

Rayon provides data-parallel processing for CPU-intensive tasks.

---

# Build Profiles

VoxelVerse uses optimized development and release profiles.

## Development

```toml
[profile.dev]
opt-level = 1
```

This provides some optimization while maintaining faster development builds.

## Release

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

The release configuration enables:

* Optimization level 3
* Link-Time Optimization
* Single code generation unit

For normal development:

```bash
cargo run
```

For performance testing:

```bash
cargo run --release
```

---

# Cross-Platform Development

VoxelVerse is intended to support:

```text
Windows
Linux
macOS
```

When writing platform-independent code, prefer Rust's standard cross-platform APIs.

For filesystem paths, use:

```rust
std::path::Path
std::path::PathBuf
```

instead of hard-coded operating-system paths.

### Avoid

```rust
let path = "C:\\Users\\Player\\VoxelVerse";
```

### Prefer

```rust
use std::path::PathBuf;

let path = PathBuf::from("VoxelVerse");
```

---

# Platform-Specific Code

When operating-system-specific functionality is required, use Rust conditional compilation.

```rust
#[cfg(target_os = "windows")]
fn platform_name() {
    println!("Running on Windows");
}

#[cfg(target_os = "linux")]
fn platform_name() {
    println!("Running on Linux");
}

#[cfg(target_os = "macos")]
fn platform_name() {
    println!("Running on macOS");
}
```

Supported targets include:

```text
windows
linux
macos
```

---

# Troubleshooting

## `cargo: command not found`

Your terminal may not have Cargo in its `PATH`.

On Linux/macOS:

```bash
source "$HOME/.cargo/env"
```

Then:

```bash
cargo --version
```

On Windows, restart PowerShell or Command Prompt after installing Rust.

---

## `rustc: command not found`

Check rustup:

```bash
rustup --version
```

Then select the stable toolchain:

```bash
rustup default stable
```

Restart the terminal and verify:

```bash
rustc --version
```

---

## Build errors on Windows

Make sure **Visual Studio Build Tools** is installed with:

```text
Desktop development with C++
Windows SDK
```

---

## Build errors on macOS

Make sure Apple's Command Line Tools are installed:

```bash
xcode-select --install
```

Then verify:

```bash
xcode-select -p
```

---

## Build errors on Linux

Make sure the compiler and development tools are installed:

```bash
sudo apt update
sudo apt install build-essential pkg-config
```

Then try:

```bash
cargo clean
cargo build
```

---

# Recommended Development Workflow

After cloning the repository:

```bash
cd voxel-verse
```

Check the project:

```bash
cargo check
```

Format the code:

```bash
cargo fmt
```

Build:

```bash
cargo build
```

Run:

```bash
cargo run
```

Before committing changes:

```bash
cargo fmt -- --check
cargo check
cargo clippy
cargo test
```

For performance testing:

```bash
cargo run --release
```

---

# Quick Start

If Rust and the required platform tools are already installed:

```bash
git clone https://github.com/cedrickcuencaalegsao/voxel-verse.git
cd voxel-verse
cargo run
```

That's it.

For maximum performance:

```bash
cargo run --release
```

---

# Repository

**GitHub:**

[cedrickcuencaalegsao/voxel-verse](https://github.com/cedrickcuencaalegsao/voxel-verse?utm_source=chatgpt.com)

---

# License

License information will be added to the project as development progresses.
