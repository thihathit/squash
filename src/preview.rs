use std::{path::PathBuf, sync::Arc};

use gpui::{Context, FontWeight, IntoElement, ObjectFit, Render, Window, div, prelude::*, svg};

use crate::{
    gpui_components::{CachedImgStore, ProgressCircle, cached_img},
    theme::{ThemeValues, get_theme},
    utilities::human_size,
};

#[derive(Debug, Clone)]
pub struct PreviewData {
    pub img_cache: Arc<CachedImgStore>,
    pub path: PathBuf,
    pub name: String,
    pub original_bytes: u64,
    pub compressed_bytes: u64,
    pub index: usize,
    pub is_processing: bool,
    pub is_error: bool,
}

pub struct Preview {
    img_cache: Arc<CachedImgStore>,
    theme: ThemeValues,
    name: String,
    path: PathBuf,
    original_bytes: u64,
    sequence: bool,
    pub compressed_bytes: u64,
    pub is_processing: bool,
    pub is_error: bool,
}

impl Preview {
    pub fn new(data: PreviewData, cx: &mut Context<Self>) -> Self {
        let theme = get_theme(cx);

        let PreviewData {
            img_cache,
            compressed_bytes,
            is_error,
            is_processing,
            original_bytes,
            path,
            index,
            name,
        } = data;

        let sequence = (index % 2) == 0;

        Self {
            img_cache,
            is_processing,
            original_bytes,
            compressed_bytes,
            sequence,
            theme,
            path,
            is_error,
            name,
        }
    }

    fn original_size(&self) -> String {
        human_size(self.original_bytes).to_uppercase()
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
        let name = self.name.to_owned();

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

        let img_cache: &CachedImgStore = self.img_cache.as_ref();
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

        let status_indicator = div()
            .id("Preview:status_indicator")
            .size_3()
            .flex_shrink_0()
            .map(|el| match (is_processing, is_error, is_smaller) {
                (true, _, _) => el.child(
                    ProgressCircle::new("Preview:progress")
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
            .id("Preview:thumbnail")
            .w_8()
            .h_8()
            .flex_shrink_0()
            .rounded_md()
            .bg(img_bg)
            .overflow_hidden()
            .child(
                cached_img(src, img_cache)
                    .size_full()
                    .object_fit(ObjectFit::Cover),
            );

        let filename = div()
            .id("Preview:filename")
            .flex_1()
            .min_w_0()
            .overflow_x_hidden()
            .whitespace_nowrap()
            .text_ellipsis()
            .child(name);

        let compression_result = div()
            .id("Preview:compression_result")
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
            .id("Preview:left")
            .flex_1()
            .min_w_0()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .child(status_indicator)
            .child(thumbnail)
            .child(filename);

        let right_side = div()
            .id("Preview:right")
            .text_color(right_text_color)
            .child(compression_result);

        let container = div()
            .id("Preview")
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
