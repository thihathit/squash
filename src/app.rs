use gpui::{Context, SharedString, Window, WindowAppearance, div, prelude::*};

use crate::theme::APP_THEME;

pub struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let appearance = cx.window_appearance();

        let theme = match appearance {
            WindowAppearance::Dark => APP_THEME.dark,
            WindowAppearance::Light => APP_THEME.light,
            _ => APP_THEME.default,
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_3()
            .bg(theme.base_color)
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .text_xl()
            .text_color(theme.base_text_color)
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().size_8().bg(gpui::red()))
                    .child(div().size_8().bg(gpui::green()))
                    .child(div().size_8().bg(gpui::blue()))
                    .child(div().size_8().bg(gpui::yellow()))
                    .child(div().size_8().bg(gpui::black()))
                    .child(div().size_8().bg(gpui::white())),
            )
    }
}

pub fn element() -> impl Render {
    HelloWorld {
        text: "World".into(),
    }
}
