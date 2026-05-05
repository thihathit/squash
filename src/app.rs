use gpui::{
    Bounds, WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowOptions, prelude::*,
    px, size,
};
use gpui_platform::application;

use crate::assets::Assets;

use super::root::Root;

pub fn app() {
    let mut app_instance = application();

    app_instance = app_instance.with_assets(Assets);

    app_instance.run(|app_cx| {
        let bounds = Bounds::centered(None, size(px(650.), px(400.0)), app_cx);
        let window = app_cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Blurred,
                window_decorations: Some(WindowDecorations::Client),
                titlebar: None,
                ..Default::default()
            },
            |_, window_cx| window_cx.new(|new_cx| Root::new(new_cx)),
        );

        window.unwrap();

        app_cx.activate(true);
    });
}
