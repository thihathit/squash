# squash

[![Release](https://img.shields.io/github/v/release/thihathit/squash)](https://github.com/thihathit/squash/releases/latest)
[![License](https://img.shields.io/badge/license-MIT-blue)]()

A blazing fast desktop lossless image compression app built with Rust and [GPUI](https://gpui.rs).

Compress images by dragging them into the window. Native performance, zero fuss.

## Features

- **Lossless** compression for PNG, JPEG, WebP, and GIF formats
- Supports JPG, PNG, GIF, WebP, and TIFF
- Native desktop GUI built with GPUI (Zed's framework)
- Parallel compression using Rayon
- Drag and drop support
- Real-time preview with disk-cached rendering
- Custom title bar with window dragging
- Cross-platform (macOS, Linux, Windows)

## Installation

Download the latest release from the [releases page](https://github.com/thihathit/squash/releases/latest):

### macOS (Apple Silicon)

```bash
curl -L https://github.com/thihathit/squash/releases/latest/download/Squash_0.2.0_aarch64.dmg -o Squash_0.2.0_aarch64.dmg
open Squash_0.2.0_aarch64.dmg
```

### Linux (x86_64)

**AppImage:**
```bash
curl -L https://github.com/thihathit/squash/releases/latest/download/squash_0.2.0_x86_64.AppImage -o squash.AppImage
chmod +x squash.AppImage
./squash.AppImage
```

**Debian/Ubuntu:**
```bash
curl -L https://github.com/thihathit/squash/releases/latest/download/squash_0.2.0_amd64.deb -o squash_0.2.0_amd64.deb
sudo dpkg -i squash_0.2.0_amd64.deb
```

### Windows

Download and run [squash_0.2.0_x64-setup.exe](https://github.com/thihathit/squash/releases/latest/download/squash_0.2.0_x64-setup.exe).

## Usage

Launch the app, drag your images into the window, and they'll be compressed instantly.

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- GPUI dependencies (see [GPUI prerequisites](https://gpui.rs/))

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```