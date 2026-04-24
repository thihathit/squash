# squash

A blazing fast image compression tool that compresses all images in the current directory using parallel threads.

## Features

- Supports JPG, JPEG, PNG, GIF, WebP, and TIFF formats
- Parallel processing using multiple threads
- Overwrites images in-place

## Installation

### Linux (x86_64)

```bash
curl -L https://github.com/thihathit/squash/releases/latest/download/squash-linux-x86_64 -o squash
chmod +x squash
```

### macOS (Apple Silicon)

```bash
curl -L https://github.com/thihathit/squash/releases/latest/download/squash-macos-aarch64 -o squash
chmod +x squash
```

### Windows

Download [squash-windows-x86_64.exe](https://github.com/thihathit/squash/releases/latest/download/squash-windows-x86_64.exe) and run in PowerShell or Command Prompt.

## Usage

```bash
# Run in the directory containing images
./squash
```

That's it. All supported image files in the current directory will be compressed in place.

## Development

```bash
cargo build --release
```