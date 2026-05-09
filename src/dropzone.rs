use std::{ops::Range, path::PathBuf};

use gpui::{
    Context, Div, DragMoveEvent, Entity, ExternalPaths, FontWeight, MouseButton, Stateful,
    TextAlign, Window, div, prelude::*, svg, uniform_list,
};

use futures::channel::oneshot::channel;

use crate::{
    compression::{VALID_EXTENSIONS, compress_image},
    preview::Preview,
    theme::{ThemeValues, get_theme},
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
    theme: ThemeValues,
    state: State,
}

impl DropZone {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let theme = get_theme(cx);
        let files = Vec::new();

        let state = State {
            has_active_drag: false,
            files,
        };

        Self { state, theme }
    }

    fn set_active_drag(&mut self, val: bool, cx: &mut Context<Self>) {
        self.state.has_active_drag = val;

        cx.notify();
    }

    fn push_file(&mut self, path: &PathBuf, index: usize, cx: &mut Context<Self>) {
        let preview = cx.new(|new_cx| Preview::new(new_cx, path.to_owned(), true, index));

        let image_path = path.to_owned();

        let preview_entity = preview.clone();

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
                    this.set_compressed_bytes(byte, this_cx);
                    this.set_processing(false, this_cx)
                }),
                // Accounts for:
                // - thread crash
                // - failed to compress
                _ => {
                    preview_entity.update(thread_cx, |this, this_cx| this.set_error(true, this_cx))
                }
            }
        });
        thread.detach();

        let file = FileInfo {
            path: path.clone(),
            preview,
        };

        self.state.files.push(file);
        cx.notify();
    }

    fn clear_files(&mut self, cx: &mut Context<Self>) {
        self.state.files.clear();
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

    fn clear_button_wrapper(&self) -> Div {
        div().flex().w_full().py_4().justify_center()
    }

    fn clear_button(&self, id: String) -> Stateful<Div> {
        div()
            .id(id)
            .rounded_full()
            .p_4()
            .cursor_pointer()
            .shadow_md()
            .bg(self.theme.btn_1_bg_color.to_owned())
            .hover(|this| this.bg(self.theme.btn_1_hover_bg_color.to_owned()))
            .child(
                svg()
                    .size_5()
                    .text_color(self.theme.btn_1_text_color.to_owned())
                    .path("brush.svg"),
            )
    }
}

impl Render for DropZone {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg = self.theme.dropzone_active;
        let is_dragging = self.state.has_active_drag;
        let state_files = &self.state.files;

        let zone = div()
            .id("DropZone:zone")
            .flex_1()
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

        let clear_button_wrapper = self
            .clear_button_wrapper()
            .child(self.clear_button("DropZone:clear_button_wrapper".to_owned()))
            .invisible();
        let clear_button = self
            .clear_button_wrapper()
            .absolute()
            .bottom_0()
            .left_0()
            .child(
                self.clear_button("DropZone:clear_button".to_owned())
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, ev_cx| this.clear_files(ev_cx)),
                    ),
            );

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
            .relative()
            .flex()
            .flex_col()
            .child(zone)
            .child(clear_button_wrapper)
            .child(clear_button)
    }
}
