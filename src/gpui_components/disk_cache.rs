use std::{
    hash::{Hash, Hasher},
    io::{self, Cursor, Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::SystemTime,
};
use std::collections::hash_map::DefaultHasher;

use gpui::{
    prelude::FluentBuilder, App, Corners, ElementId, IntoElement, ObjectFit, Refineable,
    RenderImage, RenderOnce, StyleRefinement, Styled, Window, canvas,
};
use image::{
    AnimationDecoder, Delay, Frame, ImageBuffer, ImageFormat,
    codecs::{gif::GifDecoder, webp::WebPDecoder},
};
use smallvec::SmallVec;

const CACHE_MAGIC: &[u8; 8] = b"SBMCDKCF";
const CACHE_VERSION: u32 = 1;

/// A store that writes `.sbmc` cache files into a dedicated directory.
///
/// ```ignore
/// let store = SbmcStore::new("./.sbmc_cache");
/// // later, in a render method:
/// cached_img(path, &store)
/// ```
#[derive(Clone)]
pub struct SbmcStore {
    cache_dir: PathBuf,
}

impl SbmcStore {
    /// Create a store. The directory is created (with missing parents) immediately.
    pub fn new(cache_dir: impl Into<PathBuf>) -> Self {
        let cache_dir = cache_dir.into();
        let _ = std::fs::create_dir_all(&cache_dir);
        Self { cache_dir }
    }
}

/// Build a [`CachedImg`] whose decoded bytes are cached inside `store`'s directory.
pub fn cached_img(path: impl Into<PathBuf>, store: &SbmcStore) -> CachedImg {
    let path = path.into();
    let id = ElementId::Name(path.to_string_lossy().to_string().into());
    CachedImg {
        id,
        path,
        cache_dir: Some(store.cache_dir.clone()),
        style: StyleRefinement::default(),
        object_fit: ObjectFit::Contain,
    }
}

/// A custom image element backed by `.sbmc` disk cache.
/// Produced by [`cached_img`].
///
/// Styling, object-fit, and rounded corners all work the same as `img()`.
/// The difference: decoded bytes are persisted on disk, and RAM is freed on unmount.
#[derive(IntoElement)]
pub struct CachedImg {
    id: ElementId,
    path: PathBuf,
    cache_dir: Option<PathBuf>,
    style: StyleRefinement,
    object_fit: ObjectFit,
}

impl CachedImg {
    /// Set the object-fit strategy.
    pub fn object_fit(mut self, fit: ObjectFit) -> Self {
        self.object_fit = fit;
        self
    }

}

impl Styled for CachedImg {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

enum LoadState {
    NotStarted,
    Loading,
    Loaded(Arc<RenderImage>),
    Failed,
}

impl RenderOnce for CachedImg {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let entity = window.use_keyed_state(self.id.clone(), cx, |_, _| {
            Arc::new(Mutex::new(LoadState::NotStarted))
        });
        let state = entity.read(cx).clone();

        if matches!(*state.lock().unwrap(), LoadState::NotStarted) {
            *state.lock().unwrap() = LoadState::Loading;

            let load_path = self.path.clone();
            let cache_dir = self.cache_dir.clone();
            let shared = state.clone();
            rayon::spawn(move || {
                let image = load_cached(&load_path, cache_dir.as_deref());
                let mut guard = shared.lock().unwrap();
                *guard = match image {
                    Some(img) => LoadState::Loaded(img),
                    None => LoadState::Failed,
                };
            });
        }

        let object_fit = self.object_fit;

        canvas(
            move |_, _, _| state.clone(),
            move |bounds, state, window, _cx| {
                match &*state.lock().unwrap() {
                    LoadState::Loaded(ri) => {
                        let image_size = ri.size(0);
                        let fitted = object_fit.get_bounds(bounds, image_size);
                        let _ = window.paint_image(
                            fitted,
                            Corners::default(),
                            ri.clone(),
                            0,
                            false,
                        );
                    }
                    LoadState::Loading => {
                        window.refresh();
                    }
                    _ => {}
                }
            },
        )
        .map(|mut this| {
            this.style().refine(&self.style);
            this
        })
    }
}

// ---------------------------------------------------------------------------
// Loading / decoding / caching
// ---------------------------------------------------------------------------

fn cache_path_for(source: &Path, cache_dir: Option<&Path>) -> PathBuf {
    match cache_dir {
        Some(dir) => {
            let mut hasher = DefaultHasher::new();
            source.hash(&mut hasher);
            let hash = hasher.finish();
            dir.join(format!("{:016x}.sbmc", hash))
        }
        None => {
            let mut s = source.as_os_str().to_os_string();
            s.push(".sbmc");
            PathBuf::from(s)
        }
    }
}

fn load_cached(path: &Path, cache_dir: Option<&Path>) -> Option<Arc<RenderImage>> {
    let cache_path = cache_path_for(path, cache_dir);

    // ── try .sbmc disk cache ────────────────────────────────────────
    if cache_path.exists() {
        match read_cache(&cache_path, path) {
            Ok(Some(image)) => return Some(image),
            Ok(None) => { /* stale */ }
            Err(_) => { let _ = std::fs::remove_file(&cache_path); }
        }
    }

    // ── decode from source ──────────────────────────────────────────
    let image = match decode_image_from_path(path) {
        Ok(img) => img,
        Err(_) => return None,
    };

    // ── write .sbmc ─────────────────────────────────────────────────
    if let Err(_) = write_cache(&cache_path, path, &image) {
        // non-fatal
    }

    Some(image)
}

fn decode_image_from_path(path: &Path) -> Result<Arc<RenderImage>, gpui::ImageCacheError> {
    let bytes = std::fs::read(path)?;
    if let Ok(format) = image::guess_format(&bytes) {
        let data = match format {
            ImageFormat::Gif => decode_gif(&bytes)?,
            ImageFormat::WebP => decode_webp(&bytes)?,
            _ => {
                let mut pixels =
                    image::load_from_memory_with_format(&bytes, format)
                        .map_err(|e| gpui::ImageCacheError::Image(Arc::new(e)))?
                        .into_rgba8();
                for chunk in pixels.chunks_exact_mut(4) {
                    chunk.swap(0, 2);
                }
                SmallVec::from_elem(Frame::new(pixels), 1)
            }
        };
        Ok(Arc::new(RenderImage::new(data)))
    } else {
        Err(gpui::ImageCacheError::Other(Arc::new(anyhow::anyhow!(
            "unsupported image format: {}",
            path.display()
        ))))
    }
}

fn decode_gif(bytes: &[u8]) -> Result<SmallVec<[Frame; 1]>, gpui::ImageCacheError> {
    let decoder = GifDecoder::new(Cursor::new(bytes))?;
    let mut frames = SmallVec::new();
    for frame in decoder.into_frames() {
        match frame {
            Ok(mut f) => {
                for chunk in f.buffer_mut().chunks_exact_mut(4) {
                    chunk.swap(0, 2);
                }
                frames.push(f);
            }
            Err(_) => {}
        }
    }
    if frames.is_empty() {
        return Err(gpui::ImageCacheError::Other(Arc::new(anyhow::anyhow!(
            "no valid GIF frames"
        ))));
    }
    Ok(frames)
}

fn decode_webp(bytes: &[u8]) -> Result<SmallVec<[Frame; 1]>, gpui::ImageCacheError> {
    let mut decoder = WebPDecoder::new(Cursor::new(bytes))?;
    if decoder.has_animation() {
        let _ = decoder.set_background_color(image::Rgba([0, 0, 0, 0]));
        let mut frames = SmallVec::new();
        for frame in decoder.into_frames() {
            match frame {
                Ok(mut f) => {
                    for chunk in f.buffer_mut().chunks_exact_mut(4) {
                        chunk.swap(0, 2);
                    }
                    frames.push(f);
                }
                Err(_) => {}
            }
        }
        if frames.is_empty() {
            return Err(gpui::ImageCacheError::Other(Arc::new(anyhow::anyhow!(
                "no valid WebP frames"
            ))));
        }
        Ok(frames)
    } else {
        let mut pixels =
            image::DynamicImage::from_decoder(decoder)
                .map_err(|e| gpui::ImageCacheError::Image(Arc::new(e)))?
                .into_rgba8();
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2);
        }
        Ok(SmallVec::from_elem(Frame::new(pixels), 1))
    }
}

// ---------------------------------------------------------------------------
// .sbmc file format
// ---------------------------------------------------------------------------

fn read_cache(path: &Path, source: &Path) -> Result<Option<Arc<RenderImage>>, gpui::ImageCacheError> {
    let mut file = std::fs::File::open(path).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    let mut r = io::BufReader::new(&mut file);

    let mut magic = [0u8; 8];
    r.read_exact(&mut magic).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    if &magic != CACHE_MAGIC {
        return Err(gpui::ImageCacheError::Other(Arc::new(anyhow::anyhow!("bad .sbmc magic"))));
    }

    let ver = read_u32_le(&mut r).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    if ver != CACHE_VERSION {
        return Err(gpui::ImageCacheError::Other(Arc::new(anyhow::anyhow!("unsupported .sbmc version"))));
    }

    let cached_mtime = read_i64_le(&mut r).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    match source_mtime(source) {
        Some(m) if m == cached_mtime => {}
        _ => return Ok(None),
    }

    let frame_count = read_u32_le(&mut r).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    let mut frames: SmallVec<[Frame; 1]> = SmallVec::new();

    for _ in 0..frame_count {
        let w = read_u32_le(&mut r).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
        let h = read_u32_le(&mut r).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
        let dn = read_u32_le(&mut r).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
        let dd = read_u32_le(&mut r).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
        let len = read_u32_le(&mut r).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;

        let mut pixels = vec![0u8; len as usize];
        r.read_exact(&mut pixels).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;

        let buf = ImageBuffer::from_raw(w, h, pixels).ok_or_else(|| {
            gpui::ImageCacheError::Other(Arc::new(anyhow::anyhow!("bad .sbmc frame: {w}x{h}")))
        })?;

        frames.push(Frame::from_parts(buf, 0, 0, Delay::from_numer_denom_ms(dn, dd)));
    }

    Ok(Some(Arc::new(RenderImage::new(frames))))
}

fn write_cache(path: &Path, source: &Path, image: &RenderImage) -> Result<(), gpui::ImageCacheError> {
    let mtime = source_mtime(source).unwrap_or(0);

    let file = std::fs::File::create(path).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    let mut w = io::BufWriter::new(file);

    w.write_all(CACHE_MAGIC).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    write_u32_le(&mut w, CACHE_VERSION).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    write_i64_le(&mut w, mtime).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;

    let frames = image.frame_count() as u32;
    write_u32_le(&mut w, frames).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;

    for fi in 0..image.frame_count() {
        let sz = image.size(fi);
        let (dn, dd) = image.delay(fi).numer_denom_ms();
        let bytes = image.as_bytes(fi).unwrap_or(&[]);

        write_u32_le(&mut w, sz.width.0.max(0) as u32).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
        write_u32_le(&mut w, sz.height.0.max(0) as u32).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
        write_u32_le(&mut w, dn).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
        write_u32_le(&mut w, dd).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
        write_u32_le(&mut w, bytes.len() as u32).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
        w.write_all(bytes).map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    }

    w.flush().map_err(|e| gpui::ImageCacheError::Io(Arc::new(e)))?;
    Ok(())
}

fn source_mtime(path: &Path) -> Option<i64> {
    std::fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
}

fn read_u32_le<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

fn read_i64_le<R: Read>(r: &mut R) -> io::Result<i64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(i64::from_le_bytes(b))
}

fn write_u32_le<W: Write>(w: &mut W, v: u32) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}

fn write_i64_le<W: Write>(w: &mut W, v: i64) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
