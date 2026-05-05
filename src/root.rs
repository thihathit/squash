use crate::{
    dropzone::DropZone,
    theme::{ThemeValues, get_logo, get_theme},
};
use gpui::{Context, Entity, MouseButton, Window, div, img, prelude::*, svg};

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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let logo = div().pl_2p5().child(img(self.logo.to_owned()).w_5());

        let close = div()
            .px_2p5()
            .py_2()
            .child(
                svg()
                    .size_4()
                    .text_color(self.theme.white.to_owned())
                    .path("cross.svg"),
            )
            .cursor_pointer()
            .hover(|this| this.bg(self.theme.danger_text_color))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|_, _, _, ev_cx| ev_cx.quit()),
            );

        let top_bar = div()
            .w_full()
            .flex()
            .gap_1()
            .cursor_grab()
            .items_center()
            .justify_between()
            .bg(self.theme.status_bar_bg)
            .child(logo)
            .child(close);

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
