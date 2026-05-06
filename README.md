# squash

A blazing fast desktop image compression app built with Rust and [GPUI](https://gpui.rs).

Compress images by dragging them into the window. Native performance, zero fuss.

## Features

- Supports JPG, JPEG, PNG, GIF, WebP, and TIFF formats
- Native desktop GUI built with GPUI (Zed's framework)
- Parallel compression using Rayon
- Drag and drop support
- Real-time preview

## Installation

Download the latest release from the [releases page](https://github.com/thihathit/squash/releases/latest):

### macOS (Apple Silicon)

```bash
curl -L https://github.com/thihathit/squash/releases/latest/download/squash-macos-aarch64 -o squash
chmod +x squash
./squash
```

### macOS (Intel)

```bash
curl -L https://github.com/thihathit/squash/releases/latest/download/squash-macos-x86_64 -o squash
chmod +x squash
./squash
```

### Linux (x86_64)

```bash
curl -L https://github.com/thihathit/squash/releases/latest/download/squash-linux-x86_64 -o squash
chmod +x squash
./squash
```

### Windows

Download [squash-windows-x86_64.exe](https://github.com/thihathit/squash/releases/latest/download/squash-windows-x86_64.exe) and run it.

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