use std::path::PathBuf;

use gpui::{
    Context, DragMoveEvent, Entity, ExternalPaths, FontWeight, TextAlign, Window, div, prelude::*,
};

use crate::{
    compression::VALID_EXTENSIONS,
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
        let preview = cx.new(|new_cx| Preview::new(new_cx, path.clone(), 0, index));
        let file = FileInfo {
            path: path.clone(),
            preview,
        };

        self.state.files.push(file);
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
        let is_dragging = self.state.has_active_drag;

        let zone = match self.state.files.is_empty() {
            true => div()
                .flex()
                .p_3()
                .size_full()
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
                let previews = self.state.files.iter().map(|file| &file.preview);

                div().children(previews.cloned())
            }
        };

        let bg = self.theme.dropzone_active;

        div()
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
