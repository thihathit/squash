use crate::theme::{ThemeValues, get_logo, get_theme};
use gpui::{Context, Window, WindowControlArea, div, img, prelude::*, svg};

pub struct TopBar {
    theme: ThemeValues,
    logo: String,
}

impl TopBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let theme = get_theme(cx);
        let logo = get_logo(cx);

        Self { theme, logo }
    }
}

impl Render for TopBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let logo = div().pl_2p5().child(img(self.logo.to_owned()).w_5());

        let close = div()
            .id("TopBar:close")
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

        div()
            .id("TopBar")
            .w_full()
            .relative()
            .child(window_control)
            .child(top_bar_contents)
    }
}
