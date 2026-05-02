use std::path::PathBuf;

use gpui::{
    Context, FontWeight, IntoElement, ObjectFit, Render, Window, div, img, prelude::*, rgb,
};

use crate::{
    theme::{ThemeValues, get_theme},
    types::PathFormatter,
    utilities::human_size,
};

pub struct Preview {
    theme: ThemeValues,
    path: PathBuf,
    formatter: PathFormatter,
    pub progress: i8,
    pub original_bytes: u64,
    pub compressed_bytes: u64,
    pub sequence: bool,
}

impl Preview {
    pub fn new(cx: &mut Context<Self>, path: PathBuf, progress: i8, index: usize) -> Self {
        let theme = get_theme(cx);
        let formatter = PathFormatter::new(path.clone());
        let original_bytes = formatter.file_bytes();
        let compressed_bytes = original_bytes.clone();
        let sequence = (index % 2) == 0;

        Self {
            formatter,
            progress,
            original_bytes,
            compressed_bytes,
            sequence,
            theme,
            path,
        }
    }

    fn name(&self) -> String {
        self.formatter.file_name()
    }

    fn is_done(&self) -> bool {
        self.progress >= 100
    }

    fn original_size(&self) -> String {
        self.formatter.size()
    }

    fn compressed_size(&self) -> String {
        human_size(self.compressed_bytes).to_uppercase()
    }

    fn is_smaller(&self) -> bool {
        self.original_bytes < self.compressed_bytes
    }

    fn savings(&self) -> String {
        let size = 100 - (self.compressed_bytes * 100 / self.original_bytes);

        format!("{}%", size)
    }

    pub fn set_progress(&mut self, val: i8, cx: &mut Context<Self>) {
        self.progress = val;
        cx.notify();
    }

    pub fn set_compressed_bytes(&mut self, val: u64, cx: &mut Context<Self>) {
        self.compressed_bytes = val;
        cx.notify();
    }
}

impl Render for Preview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let name = self.name();

        let is_done = self.is_done();
        let is_processing = !is_done;
        let original_size = self.original_size();
        let size = original_size.clone();
        let compressed_size = self.compressed_size();
        let savings = self.savings();
        let bg = match self.sequence {
            true => self.theme.file_bg_sequence_1,
            false => self.theme.transparent,
        };
        let src = self.path.to_owned();
        let img_bg = self.theme.dropzone_active;
        let right_text_color = self.theme.secondary_text_color;
        let before_size_color = self.theme.warning_text_color;
        let savings_size_color = match self.is_smaller() {
            true => self.theme.success_text_color,
            false => self.theme.danger_text_color,
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .px_4()
            .py_2()
            .gap_3()
            .bg(bg)
            .font_weight(FontWeight::MEDIUM)
            // left side
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    // status indicator
                    .child(div().w_4().h_4().rounded_full().border_1().map(|el| {
                        match (is_done, is_processing) {
                            (true, _) => el.bg(rgb(0x22c55e)).border_color(rgb(0x22c55e)), // green
                            (_, true) => el.bg(rgb(0x3b82f6)).border_color(rgb(0x3b82f6)), // blue
                            _ => el.border_color(rgb(0x52525b)), // gray ring
                        }
                    }))
                    // thumbnail
                    .child(
                        div()
                            .w_8()
                            .h_8()
                            .rounded_sm()
                            .bg(img_bg)
                            .overflow_hidden()
                            .child(img(src).size_full().object_fit(ObjectFit::Cover)),
                    )
                    // filename
                    .child(div().text_sm().child(name)),
            )
            // right side — size info
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .text_xs()
                    .text_color(right_text_color)
                    .child(match is_done {
                        true => div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .child(original_size)
                            .child("→")
                            .child(div().text_color(before_size_color).child(compressed_size))
                            .child("·")
                            .child(div().text_color(savings_size_color).child(savings)),
                        false => div().child(size),
                    }),
            )
    }
}
