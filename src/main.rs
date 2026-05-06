#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod assets;
mod compression;
mod dropzone;
mod gpui_components;
mod preview;
mod root;
mod theme;
mod topbar;
mod types;

#[expect(dead_code)]
mod utilities;

use app::app;

fn main() {
    app();
}
