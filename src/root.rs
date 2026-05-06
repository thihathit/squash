use crate::{
    dropzone::DropZone,
    theme::{ThemeValues, get_logo, get_theme},
};
use gpui::{Context, Entity, Window, WindowControlArea, div, img, prelude::*, svg};

pub struct Root {
    dropzone: Entity<DropZone>,
    theme: ThemeValues,
    logo: String,
}

impl Root {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let theme = get_theme(cx);
        let logo = get_logo(cx);
        let dropzone = cx.new(|new_cx| DropZone::new(new_cx));

        Self {
            dropzone,
            theme,
            logo,
        }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let logo = div().pl_2p5().child(img(self.logo.to_owned()).w_5());

        let close = div()
            .id("top-bar-close")
            .px_2p5()
            .py_2()
            .child(
                svg()
                    .size_4()
                    .text_color(self.theme.white.to_owned())
                    .path("cross.svg"),
            )
            .occlude()
            .cursor_pointer()
            .hover(|this| this.bg(self.theme.danger_text_color))
            .window_control_area(WindowControlArea::Close);

        let window_control = div()
            .size_full()
            .absolute()
            .top_0()
            .left_0()
            .cursor_grab()
            .bg(self.theme.status_bar_bg)
            .window_control_area(WindowControlArea::Drag);

        let top_bar_contents = div()
            .w_full()
            .flex()
            .gap_1()
            .items_center()
            .justify_between()
            .child(logo)
            .child(close);

        let top_bar = div()
            .w_full()
            .relative()
            .child(window_control)
            .child(top_bar_contents);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(self.theme.app_bg)
            .text_color(self.theme.base_text_color)
            .child(top_bar)
            .child(self.dropzone.clone())
    }
}
