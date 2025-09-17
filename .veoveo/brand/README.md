# VeoVeo Brand Assets and Rebranding Guide

This directory contains all the branding assets and instructions needed to rebrand Rerun as VeoVeo.

## Overview

VeoVeo is a branded fork of the Rerun project, maintaining the core functionality while establishing a unique visual identity and brand presence.

## Key Changes

### Changed Files

#### Images and Icons
- `crates/viewer/re_ui/data/icons/veoveo.png` - Main VeoVeo icon
- `crates/viewer/re_ui/data/icons/veoveo_menu.png` - Menu icon variant
- `crates/viewer/re_viewer/data/veoveo_splash.png` - Splash screen image
- `web_viewer/veoveo_logo_v1_full_gray_512.png` - Full logo variant
- `web_viewer/favicon.ico` - Web favicon
- `web_viewer/favicon.svg` - SVG favicon

#### Source Code Files
- `crates/viewer/re_ui/src/icons.rs` - Icon definitions
- `crates/viewer/re_viewer/src/ui/rerun_menu.rs` - Menu branding
- `crates/viewer/re_viewer/src/ui/top_panel.rs` - Top panel branding
- `crates/viewer/re_viewer/src/ui/welcome_screen/mod.rs` - Welcome screen module
- `crates/viewer/re_viewer/src/ui/welcome_screen/veoveo_splash.rs` - Splash screen implementation

#### Web Files
- `web_viewer/index.html` - Main web viewer page
- `web_viewer/sw.js` - Service worker

#### Documentation
- `README.md` - Added VeoVeo splash image reference

#### Build Configuration
- `examples/rust/custom_data_loader/Cargo.toml` - Example configuration

## Rebranding Guide

This guide explains how to apply VeoVeo branding to a fresh Rerun upstream pull.

### Files to Modify

#### 1. Icon Assets
Copy the following image files from `.veoveo/brand/images/` to their destinations:
- `veoveo.png` → `crates/viewer/re_ui/data/icons/veoveo.png`
- `veoveo_menu.png` → `crates/viewer/re_ui/data/icons/veoveo_menu.png`
- `veoveo_splash.png` → `crates/viewer/re_viewer/data/veoveo_splash.png`
- `favicon.ico` → `crates/viewer/re_web_viewer_server/web_viewer/favicon.ico`
- `favicon.svg` → `crates/viewer/re_web_viewer_server/web_viewer/favicon.svg`
- `veoveo_logo_v1_full_gray_512.png` → `crates/viewer/re_web_viewer_server/web_viewer/veoveo_logo_v1_full_gray_512.png`

#### 2. Source Code Changes

##### Icon Registration
In `crates/viewer/re_ui/src/icons.rs`, after line 119 with `RERUN_MENU`, add:
```rust
pub const VEOVEO_MENU: Icon = icon_from_path!("../data/icons/veoveo_menu.png");
```

##### Splash Screen Implementation
Create `crates/viewer/re_viewer/src/ui/welcome_screen/veoveo_splash.rs` with:
```rust
use re_ui::Icon;
use re_ui::UiExt as _;

pub const VEOVEO_SPLASH: Icon = Icon::new(
    "veoveo_splash",
    include_bytes!("../../../data/veoveo_splash.png"),
);

/// Show a minimal welcome section with just a centered image.
pub fn veoveo_splash_ui(ui: &mut egui::Ui) {
    ui.center("veoveo_splash_contents", |ui| {
        let image = VEOVEO_SPLASH.as_image();
        ui.add(image);
    });
}
```

##### Welcome Screen Module
In `crates/viewer/re_viewer/src/ui/welcome_screen/mod.rs`:
1. Add `mod veoveo_splash;` after line 3
2. Replace `use welcome_section::welcome_section_ui;` with `use veoveo_splash::veoveo_splash_ui;`
3. Replace the entire `.show(ui, |ui| { ... })` block (around line 55-67) with:
```rust
.show(ui, |ui| {
    // Just show the VeoVeo splash - no examples, no other UI
    veoveo_splash_ui(ui);
});
```

##### Menu Icon Update
In `crates/viewer/re_viewer/src/ui/rerun_menu.rs`, line 32, change:
```rust
let image = re_ui::icons::RERUN_MENU
```
to:
```rust
let image = re_ui::icons::VEOVEO_MENU
```

##### Top Panel Branding
In `crates/viewer/re_viewer/src/ui/top_panel.rs`:
1. Remove line 1: `use egui::NumExt as _;` (if present)
2. In function `website_link_ui` (around line 420), replace the function body with:
```rust
fn website_link_ui(ui: &mut egui::Ui) {
    // Use text for VeoVeo branding
    let url = "https://vertexstudio.co/";
    let response = ui
        .add(egui::Button::new("VeoVeo").frame(false))
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    if response.clicked() {
        ui.ctx().open_url(egui::output::OpenUrl {
            url: url.to_owned(),
            new_tab: true,
        });
    }
}
```

##### Remove Examples Section
In `crates/viewer/re_recording_panel/src/data.rs`, around line 136, replace:
```rust
let show_example_section = ctx
    .app_options()
    .include_rerun_examples_button_in_recordings_panel
    && !hide_examples
    || !example_apps.is_empty();
```
with:
```rust
// VeoVeo: Disable examples section entirely
let show_example_section = false;
```

#### 3. Web Viewer Changes

##### HTML Title
In `crates/viewer/re_web_viewer_server/web_viewer/index.html`, line 24, change:
```html
<title>Rerun Viewer</title>
```
to:
```html
<title>VeoVeo</title>
```

#### 4. Documentation

##### README
In the root `README.md`, add the VeoVeo splash AFTER the badges section (around line 13), inserting:
```html
<h1 align="center">
<img src="crates/viewer/re_viewer/data/veoveo_splash.png" alt="VeoVeo">
</h1>
```
Note: Keep the original Rerun banner and badges intact. The VeoVeo splash should be added as a third heading section after them.

### Applying the Rebrand

1. Get latest from upstream Rerun repository
2. Ensure Git LFS is installed and configured (brand images are stored in LFS)
3. Copy all images from `.veoveo/brand/images/` to their respective locations
4. Apply code changes from `.veoveo/brand/code-snippets/`
5. Update documentation as specified
6. Build and test the rebranded application

### Verification

After rebranding:
1. Check that VeoVeo splash screen appears on startup
2. Verify menu icons are updated
3. Confirm web viewer shows VeoVeo title and favicon
4. Test that all functionality remains intact

## Directory Structure

```
.veoveo/brand/
├── README.md           # This file
├── images/             # All VeoVeo branded image assets
│   ├── favicon.ico
│   ├── favicon.svg
│   ├── veoveo.png
│   ├── veoveo_logo_v1_full_gray_512.png
│   ├── veoveo_menu.png
│   └── veoveo_splash.png
└── code-snippets/      # Key code changes for rebranding
    ├── icons.rs
    ├── readme-splash.md
    ├── splash.rs
    └── web-title.html
```

## Brand Identity

VeoVeo maintains the technical capabilities of Rerun while establishing its own visual identity through:
- Custom logo and icon set
- Branded splash screens
- Modified UI components
- Web presence updates
