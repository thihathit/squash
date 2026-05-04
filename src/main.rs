#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod compression;
mod dropzone;
mod gpui_components;
mod preview;
mod root;
mod theme;
mod types;
mod utilities;

use app::app;

fn main() {
    app();
}
