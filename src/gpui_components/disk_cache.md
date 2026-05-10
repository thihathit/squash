# Disk-backed image cache (`cached_img`)

Drop-in replacement for `img(path)` that keeps decoded image bytes off the heap
and frees all RAM on element unmount.

## Usage

```rust
use crate::gpui_components::cached_img;

cached_img(path)
    .size_full()
    .object_fit(ObjectFit::Cover)
```

Accepts `PathBuf` (or anything `Into<PathBuf>`). Supports `.rounded_*` + `overflow_hidden()`
on the parent `div()` for corner-radius clipping.

## How it works

### `.sbmc` cache file

When an image is decoded, BGRA pixel data is written to `<source_path>.sbmc`. On
subsequent loads the file is read directly — no decode, no heap allocation of decoded
bytes. Staleness is checked via source-file mtime.

### Loading off the main thread

The first time an element renders, a `rayon` task is spawned to do the actual work
(decode or `.sbmc` read). While the task runs the element shows a blank area and
calls `window.refresh()` each paint frame to keep the render loop alive. When the
task completes, the loaded `Arc<RenderImage>` is stored in a shared `Mutex` and
displayed on the next paint.

### RAM on unmount

The `Arc<Mutex<...>>` is held in a GPUI element-local `Entity` via `use_keyed_state`.
When the element leaves the tree (e.g. scrolled out of a `uniform_list`) GPUI drops
the entity, the `Arc` refcount drops to zero, and the decoded image memory is freed
immediately. A rayon task that happens to still be running holds its own `Arc` clone
and can safely finish, but the result is dropped as soon as the task ends.

## File format (`.sbmc`)

| Offset | Size | Field |
|--------|------|-------|
| 0 | 8 | Magic `SBMCDKCF` |
| 8 | 4 | Version (u32 LE) |
| 12 | 8 | Source mtime (i64 LE, unix seconds) |
| 20 | 4 | Frame count (u32 LE) |
| 24+ | per frame | width(u32), height(u32), delay_num(u32), delay_den(u32), data_len(u32), BGRA pixels |
