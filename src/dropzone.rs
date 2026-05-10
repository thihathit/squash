use std::{ops::Range, path::PathBuf, sync::Arc};

use gpui::{
    Context, DragMoveEvent, Entity, ExternalPaths, FontWeight, TextAlign, Window, div, prelude::*,
    uniform_list,
};

use futures::channel::oneshot::channel;

use crate::{
    compression::{VALID_EXTENSIONS, compress_image},
    gpui_components::CachedImgStore,
    preview::{Preview, PreviewData},
    theme::{ThemeValues, get_theme},
    types::PathFormatter,
};

struct FileInfo {
    path: PathBuf,
    preview: Entity<Preview>,
}

struct State {
    files: Vec<FileInfo>,
    has_active_drag: bool,
}

pub struct DropZone {
    img_cache: Arc<CachedImgStore>,
    theme: ThemeValues,
    state: State,
}

impl Drop for DropZone {
    fn drop(&mut self) {
        let _ = self.img_cache.clear();
    }
}

impl DropZone {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let theme = get_theme(cx);
        let files = Vec::new();

        let state = State {
            has_active_drag: false,
            files,
        };

        Self {
            img_cache: Arc::new(CachedImgStore::new(
                dirs::cache_dir()
                    .unwrap_or_else(|| std::env::temp_dir())
                    .join("squash"),
            )),
            state,
            theme,
        }
    }

    fn set_active_drag(&mut self, val: bool, cx: &mut Context<Self>) {
        self.state.has_active_drag = val;

        cx.notify();
    }

    fn push_file(&mut self, path: &PathBuf, index: usize, cx: &mut Context<Self>) {
        let formatter = PathFormatter::new(path.to_owned());
        let preview = cx.new(|new_cx| {
            Preview::new(
                PreviewData {
                    img_cache: self.img_cache.clone(),
                    index,
                    is_processing: true,
                    is_error: false,
                    path: path.to_owned(),
                    name: formatter.file_name(),
                    compressed_bytes: formatter.file_bytes(),
                    original_bytes: formatter.file_bytes(),
                },
                new_cx,
            )
        });

        let preview_entity = preview.to_owned();
        let image_path = path.to_owned();

        let thread = cx.spawn(async move |_this, thread_cx| {
            // We use a "oneshot" channel to bridge the Thread -> Async gap
            let (tx, rx) = channel();

            // Offload the single-threaded function to Rayon's work-stealing pool
            rayon::spawn(move || {
                let result = compress_image(image_path).ok();

                tx.send(result).ok();
            });

            // Wait for Rayon to finish. This YIELDS the thread.
            let result = rx.await;

            match result {
                Ok(Some(byte)) => preview_entity.update(thread_cx, |this, this_cx| {
                    this.compressed_bytes = byte;
                    this.is_processing = false;
                    this_cx.notify();
                }),
                // Accounts for:
                // - thread crash
                // - failed to compress
                _ => preview_entity.update(thread_cx, |this, this_cx| {
                    this.is_error = true;
                    this_cx.notify();
                }),
            }
        });

        thread.detach();

        self.state.files.push(FileInfo {
            path: path.clone(),
            preview,
        });
        cx.notify();
    }

    fn filter_paths(&self, paths: &[PathBuf]) -> Vec<PathBuf> {
        paths
            .iter()
            .filter_map(|path| {
                let valid_extension =
                    path.extension()
                        .and_then(|ext| ext.to_str())
                        .is_some_and(|ext| {
                            VALID_EXTENSIONS
                                .iter()
                                .any(|&v| ext.eq_ignore_ascii_case(v))
                        });

                let is_already_recorded = self.state.files.iter().any(|file| &file.path == path);

                let is_valid = path.is_file() && valid_extension && !is_already_recorded;

                match is_valid {
                    true => Some(path.clone()),
                    false => None,
                }
            })
            .collect()
    }
}

impl Render for DropZone {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg = self.theme.dropzone_active;
        let is_dragging = self.state.has_active_drag;
        let state_files = &self.state.files;

        let zone = div()
            .id("DropZone:zone")
            .size_full()
            .map(|el| match state_files.is_empty() {
                true => el
                    .flex()
                    .p_3()
                    .justify_center()
                    .items_center()
                    .gap_1()
                    .text_lg()
                    .text_align(TextAlign::Center)
                    .font_weight(FontWeight::MEDIUM)
                    .child(match is_dragging {
                        true => "Drop your files here",
                        false => "Drag & Drop your files here",
                    }),
                false => {
                    let item_count = state_files.iter().len();

                    let previews = uniform_list(
                        "TopBar:file-list",
                        item_count,
                        cx.processor(|this, range: Range<usize>, _, _| {
                            range
                                .map(|i| this.state.files[i].preview.to_owned())
                                .collect()
                        }),
                    )
                    .size_full()
                    .into_any_element();

                    el.child(previews)
                }
            });

        div()
            .id("DropZone")
            .size_full()
            .bg(self.theme.transparent)
            .drag_over::<ExternalPaths>(move |style, _, _, _| style.bg(bg))
            .on_drag_move(
                cx.listener(|this, _: &DragMoveEvent<ExternalPaths>, _window, cx| {
                    this.set_active_drag(cx.has_active_drag(), cx)
                }),
            )
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
                let valid_paths = this.filter_paths(paths.paths());

                for (index, path) in valid_paths.iter().enumerate() {
                    this.push_file(path, index, cx);
                }

                this.set_active_drag(false, cx);
            }))
            .child(zone)
    }
}
