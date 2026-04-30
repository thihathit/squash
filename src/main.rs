mod app;
mod theme;

use gpui::{
    Application, Bounds, WindowBackgroundAppearance, WindowBounds, WindowOptions, prelude::*, px,
    size,
};

fn main() {
    Application::new().run(|cx| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Blurred,
                ..Default::default()
            },
            |_, cx| {
                let element = app::element();

                cx.new(|_| element)
            },
        )
        .unwrap();
    });
}
