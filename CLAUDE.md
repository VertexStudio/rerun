# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This repository contains Rerun (branded as VeoVeo), a multimodal data visualization and logging system. It's a large Rust workspace with Python bindings, C++ SDK, and web viewer components. The codebase uses Apache Arrow for data storage and egui/wgpu for visualization.

**Note**: The main branch is `veoveo` (not `main`).

## Quick Start Commands

### Most Common Tasks
- **Run viewer**: `pixi run rerun` (also accepts .rrd files as arguments)
- **Run release viewer**: `pixi run rerun-release`
- **Quick lint check**: `pixi run fast-lint` (checks only changed files against veoveo branch)
- **Full lint check**: `pixi run lint-all` (comprehensive, run before pushing)
- **Format all code**: `pixi run format`

### Build Commands

#### Viewer
- **Build viewer (debug)**: `pixi run rerun-build`
- **Build viewer (release)**: `pixi run rerun-build-release`
- **Build web viewer**: `pixi run rerun-web` (requires LLVM/clang 18+ on macOS)
- **Build web viewer (release)**: `pixi run rerun-web-release`
- **Build native + web**: `pixi run rerun-build-native-and-web`

#### SDKs
- **Python SDK (debug)**: `pixi run py-build`
- **Python SDK (release)**: `pixi run py-build-release`
- **Python wheel**: `pixi run py-build-wheel`
- **C++ SDK**: `pixi run -e cpp cpp-build-all`

## Development Workflow

### Before You Push - CI Checks

These checks MUST pass in CI:
1. **Rust formatting**: `pixi run rs-fmt --check` (or `pixi run rs-fmt` to auto-fix)
2. **Typo check**: `pixi run lint-typos`
3. **Large files check**: `pixi run check-large-files` (files >100KB must be in Git LFS)
4. **Misc formatting**: `pixi run misc-fmt --check` (or `pixi run misc-fmt` to auto-fix)

**Quick option**: `pixi run fast-lint` (for small changes)
**Safe option**: `pixi run lint-all` (comprehensive, recommended before push)

### Testing

- **Rust tests**: `cargo test -p <crate_name>`
- **Python tests**: `pixi run py-test`
- **C++ tests**: `pixi run -e cpp cpp-test`

### Code Quality Tools

#### Formatting (Auto-fix)
- **All languages**: `pixi run format`
- **Rust**: `pixi run rs-fmt`
- **Python**: `pixi run py-fmt`
- **C++**: `pixi run cpp-fmt`
- **YAML/JS/HTML**: `pixi run misc-fmt`
- **TOML**: `pixi run toml-fmt`

#### Linting (Check only)
- **Rust**: `pixi run rs-fmt --check`, `pixi run lint-rs-all`
- **Python**: `pixi run py-lint`, `pixi run py-fmt-check`
- **Markdown**: `pixi run mdlint`
- **Typos**: `pixi run lint-typos`
- **Custom rules**: `pixi run lint-rerun`
- **Generated code**: `pixi run codegen --check`

### Language-Specific Commands

#### Rust
- **Check compilation**: `pixi run rs-check`
- **Update snapshots**: `pixi run rs-update-snapshot-tests`
- **Run example**: `cargo run -p <example_name>` (e.g., `cargo run -p dna`)

#### Python
- **Lint**: `pixi run py-lint` (ruff, mypy, etc.)
- **Test**: `pixi run py-test`
- **Build wheel**: `pixi run py-build-wheel`

#### C++
- **Format**: `pixi run cpp-fmt`
- **Build all**: `pixi run -e cpp cpp-build-all`
- **Test**: `pixi run -e cpp cpp-test`

## Architecture

### Core Components

1. **Logging SDKs** (`rerun_py/`, `crates/top/re_sdk/`, `rerun_cpp/`)
   - Encode data using Apache Arrow
   - Log to `.rrd` files or stream over gRPC

2. **Viewer** (`crates/viewer/re_viewer/`)
   - Built with egui (immediate mode GUI) and wgpu
   - Runs native or WebAssembly
   - Entire GUI rebuilt each frame

3. **Data Storage** 
   - `re_chunk_store`: In-memory time series database
   - `re_entity_db`: High-level entity storage
   - Apache Arrow for data representation

4. **View System** (`crates/viewer/re_view_*/`)
   - Modular visualization types (3D, plots, images)
   - Each view type is a separate crate

### Key Design Patterns

- **Immediate Mode**: GUI and rendering pipeline rebuilt each frame
- **Entity-Component System**: Data as entities with time-varying components
- **Time-Aware**: All data timestamped, queryable at any point in time
- **Modular Views**: Composable visualization modules

## Important Notes

### Web Build on macOS
To build web viewer on macOS, you need LLVM/clang 18+ (Apple's clang has issues):
```bash
brew install llvm@18
export CC=$(brew --prefix llvm@18)/bin/clang
export CXX=$(brew --prefix llvm@18)/bin/clang++
export AR=$(brew --prefix llvm@18)/bin/llvm-ar
pixi run rerun-web
```

### Development Environment
- Enter dev environment: `pixi shell`
- Task definitions: `pixi.toml`
- Workspace config: `Cargo.toml`
- Stricter checks: `RERUN_STRICT=1`

### Key Directories
- `crates/`: Rust code
- `rerun_py/`: Python SDK
- `rerun_cpp/`: C++ SDK
- `examples/`: Example code
- `docs/`: Documentation
- `.veoveo/brand/`: VeoVeo branding assets and instructions