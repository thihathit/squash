#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod assets;
mod compression;
mod constants;
mod dropzone;
mod gpui_components;
mod preview;
mod root;
mod theme;
mod topbar;

#[expect(dead_code)]
mod types;

#[expect(dead_code)]
mod utilities;

use app::app;

fn main() {
    app();
}
