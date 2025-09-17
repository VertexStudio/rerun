# VeoVeo - Building and Running

## Overview

VeoVeo is a branded fork of Rerun, maintaining all the core functionality while establishing its own visual identity. This document explains how to build and run VeoVeo to see the custom branding in action.

## Prerequisites

1. **Rust toolchain** (1.85.0 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Pixi package manager** (recommended for consistent builds)
   ```bash
   curl -fsSL https://pixi.sh/install.sh | bash
   ```

3. **NASM** (optional but recommended for better video performance)
   - macOS: `brew install nasm`
   - Ubuntu/Debian: `sudo apt-get install nasm`
   - Windows: Download from https://www.nasm.us/

## Building and Running VeoVeo

### Quick Start (Recommended)

The simplest way to build and run VeoVeo:

```bash
pixi run rerun
```

This will build the viewer with all default features and launch it with the VeoVeo branding.

### Alternative Build Methods

1. **Direct Cargo Build**
   ```bash
   # Build and run the viewer CLI
   cargo run --package rerun-cli --no-default-features --features map_view,nasm,native_viewer
   
   # Or simply (if you have all dependencies)
   cargo run -p rerun-cli --release
   ```

2. **Run with Performance Debugging**
   ```bash
   pixi run rerun-perf-debug
   ```

3. **Build Only**
   ```bash
   # Build the viewer without running
   cargo build --package rerun-cli --release
   
   # The binary will be at: target/release/rerun
   ```

## Testing VeoVeo Branding

To see the VeoVeo branding in action:

1. **Launch the viewer**
   ```bash
   pixi run rerun
   ```
   You should see:
   - VeoVeo menu icon in the top bar
   - VeoVeo splash screen on startup
   - VeoVeo text/logo linking to https://vertexstudio.co/

2. **Run with example data**
   ```bash
   # Simple 3D points example
   cargo run -p minimal
   
   # DNA helix visualization
   cargo run -p dna
   
   # Clock example
   cargo run -p clock
   ```

3. **Load an existing recording**
   ```bash
   pixi run rerun path/to/recording.rrd
   ```

## Web Viewer

To test the web viewer with VeoVeo branding:

1. **Build the web viewer**
   ```bash
   pixi run rerun-build-web-release
   ```

2. **Serve the web viewer**
   ```bash
   pixi run rerun-web-release
   ```

3. Open http://localhost:9090 in your browser

## Troubleshooting

### Build Errors

1. **Rust version**: Ensure you have Rust 1.85.0 or later
   ```bash
   rustc --version
   ```

2. **Missing dependencies**: Install build essentials
   - macOS: Install Xcode Command Line Tools
   - Linux: `sudo apt-get install build-essential cmake`

3. **Architecture issues on Apple Silicon**
   ```bash
   # Check your architecture
   rustc -vV | grep host
   # Should show: host: aarch64-apple-darwin
   ```

### Performance Issues

If the viewer is slow:
1. Ensure you're building in release mode (`--release` flag)
2. Enable the `nasm` feature for better video decoding
3. Check if you're running with software rendering (look for warning in viewer)

## VeoVeo Branding Assets

The VeoVeo branding includes:
- **Icons**: `veoveo.png`, `veoveo_menu.png`
- **Splash Screen**: `veoveo_splash.png`
- **Web Assets**: Custom favicon and logo
- **UI Updates**: Modified menus and panels

All branding assets are stored in:
- `.veoveo/brand/images/` - Original brand assets
- Various locations in the codebase after rebranding

## Development Workflow

1. **Make changes to code**
2. **Build and test**
   ```bash
   pixi run rerun
   ```
3. **Run specific tests**
   ```bash
   cargo test -p rerun-cli
   ```

## Additional Resources

- Original Rerun documentation: https://www.rerun.io/docs
- Build instructions: See `BUILD.md` in the repository root
- Architecture details: See `ARCHITECTURE.md`

## Notes

- The pixi environment ensures consistent dependencies across different machines
- Release builds are significantly faster than debug builds
- The viewer supports both native and web deployments
- All Rerun functionality is preserved in VeoVeo