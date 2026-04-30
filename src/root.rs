use gpui::{Context, Window, WindowAppearance, div, prelude::*};

use crate::theme::APP_THEME;

pub struct Root {}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let appearance = cx.window_appearance();

        let theme = match appearance {
            WindowAppearance::Dark => APP_THEME.dark,
            WindowAppearance::Light => APP_THEME.light,
            _ => APP_THEME.default,
        };

        div()
            .size_full()
            .text_color(theme.base_text_color)
            .bg(theme.base_color)
    }
}
