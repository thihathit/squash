use std::path::PathBuf;

use gpui::{
    Context, FontWeight, IntoElement, ObjectFit, Render, Window, div, img, prelude::*, svg,
};

use crate::{
    gpui_components::ProgressCircle,
    theme::{ThemeValues, get_theme},
    types::PathFormatter,
    utilities::human_size,
};

pub struct Preview {
    theme: ThemeValues,
    path: PathBuf,
    original_bytes: u64,
    compressed_bytes: u64,
    sequence: bool,
    is_processing: bool,
    is_error: bool,

    pub formatter: PathFormatter,
}

impl Preview {
    pub fn new(cx: &mut Context<Self>, path: PathBuf, is_processing: bool, index: usize) -> Self {
        let theme = get_theme(cx);
        let formatter = PathFormatter::new(path.to_owned());
        let original_bytes = formatter.file_bytes();
        let compressed_bytes = original_bytes.to_owned();
        let sequence = (index % 2) == 0;

        Self {
            formatter,
            is_processing,
            original_bytes,
            compressed_bytes,
            sequence,
            theme,
            path,
            is_error: false,
        }
    }

    pub fn set_processing(&mut self, val: bool, cx: &mut Context<Self>) {
        self.is_processing = val;
        cx.notify();
    }

    pub fn set_error(&mut self, val: bool, cx: &mut Context<Self>) {
        self.is_error = val;
        cx.notify();
    }

    pub fn set_compressed_bytes(&mut self, val: u64, cx: &mut Context<Self>) {
        self.compressed_bytes = val;
        cx.notify();
    }

    fn name(&self) -> String {
        self.formatter.file_name()
    }

    fn original_size(&self) -> String {
        self.formatter.size()
    }

    fn compressed_size(&self) -> String {
        human_size(self.compressed_bytes).to_uppercase()
    }

    fn is_smaller(&self) -> bool {
        self.compressed_bytes < self.original_bytes
    }

    fn savings(&self) -> String {
        // cheap guard
        if self.original_bytes == 0 {
            return "0%".into();
        }

        let ratio = self.compressed_bytes * 100 / self.original_bytes;

        // Direction
        if ratio >= 100 {
            let increase = ratio - 100;
            format!("+{}%", increase)
        } else {
            format!("-{}%", 100 - ratio)
        }
    }
}

impl Render for Preview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let name = self.name();

        let is_error = self.is_error.to_owned();
        let is_processing = self.is_processing.to_owned();
        let is_smaller = self.is_smaller();
        let original_size = self.original_size();
        let size = original_size.clone();
        let compressed_size = self.compressed_size();
        let savings = self.savings();
        let bg = match self.sequence {
            false => self.theme.file_bg_sequence_1.to_owned(),
            _ => self.theme.transparent.to_owned(),
        };
        let src = self.path.to_owned();

        let img_bg = self.theme.file_bg_sequence_1.to_owned();
        let right_text_color = self.theme.secondary_text_color.to_owned();
        let before_size_color = self.theme.warning_text_color.to_owned();
        let savings_size_color = match self.is_smaller() {
            true => self.theme.success_text_color.to_owned(),
            false => self.theme.danger_text_color.to_owned(),
        };
        let progress_color = self.theme.progress_color;
        let status_danger = self.theme.danger_text_color.to_owned();
        let status_success = self.theme.success_text_color.to_owned();
        let status_warning = self.theme.warning_text_color.to_owned();

        let icon = svg().size_full();

        let status_indicator =
            div()
                .size_3()
                .flex_shrink_0()
                .map(|el| match (is_processing, is_error, is_smaller) {
                    (true, _, _) => el.child(
                        ProgressCircle::new("circle")
                            .size_full()
                            .color(progress_color)
                            .loading(true),
                    ),
                    (false, false, true) => {
                        el.child(icon.text_color(status_success).path("compress-success.svg"))
                    }
                    (false, false, false) => {
                        el.child(icon.text_color(status_warning).path("compress-warning.svg"))
                    }
                    _ => el.child(icon.text_color(status_danger).path("compress-danger.svg")),
                });

        let thumbnail = div()
            .w_8()
            .h_8()
            .flex_shrink_0()
            .rounded_md()
            .bg(img_bg)
            .overflow_hidden()
            .child(img(src).size_full().object_fit(ObjectFit::Cover));

        let filename = div()
            .flex_1()
            .min_w_0()
            .overflow_x_hidden()
            .whitespace_nowrap()
            .text_ellipsis()
            .child(name);

        let compression_result = div()
            .flex_shrink_0()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .map(|el| match !is_processing {
                true => el
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .child(original_size)
                    .child("→")
                    .child(div().text_color(before_size_color).child(compressed_size))
                    .child("·")
                    .child(div().text_color(savings_size_color).child(savings)),
                false => el.child(size),
            });

        let left_side = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .child(status_indicator)
            .child(thumbnail)
            .child(filename);

        let right_side = div().text_color(right_text_color).child(compression_result);

        let container = div()
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .px_4()
            .py_2()
            .bg(bg)
            .font_weight(FontWeight::MEDIUM)
            .text_xs()
            .child(left_side)
            .child(right_side);

        container
    }
}
