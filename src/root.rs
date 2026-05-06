use crate::{
    dropzone::DropZone,
    theme::{ThemeValues, get_theme},
    topbar::TopBar,
};
use gpui::{Context, Entity, Window, div, prelude::*};

pub struct Root {
    dropzone: Entity<DropZone>,
    topbar: Entity<TopBar>,
    theme: ThemeValues,
}

impl Root {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let theme = get_theme(cx);
        let dropzone = cx.new(|new_cx| DropZone::new(new_cx));
        let topbar = cx.new(|new_cx| TopBar::new(new_cx));

        Self {
            dropzone,
            topbar,
            theme,
        }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("Root")
            .size_full()
            .flex()
            .flex_col()
            .bg(self.theme.app_bg)
            .text_color(self.theme.base_text_color)
            .child(self.topbar.to_owned())
            .child(self.dropzone.to_owned())
    }
}
