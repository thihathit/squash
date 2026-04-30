use gpui::{
    Bounds, WindowBackgroundAppearance, WindowBounds, WindowDecorations,
    WindowOptions, prelude::*, px, size,
};
use gpui_platform::application;

use super::root::Root;

pub fn app() {
    application().run(|cx| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        let window = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Blurred,
                window_decorations: Some(WindowDecorations::Client),
                titlebar: None,
                ..Default::default()
            },
            |_, cx| cx.new(|_| Root {}),
        );

        window.unwrap();
    });
}
