use crate::{
    dropzone::DropZone,
    theme::{ThemeValues, get_theme},
};
use gpui::{Context, Entity, Window, div, prelude::*};

pub struct Root {
    dropzone: Entity<DropZone>,
    theme: ThemeValues,
}

impl Root {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let theme = get_theme(cx);
        let dropzone = cx.new(|new_cx| DropZone::new(new_cx));

        Self { dropzone, theme }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(self.theme.app_bg)
            .text_color(self.theme.base_text_color)
            .child(self.dropzone.clone())
    }
}
