#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use eframe::egui::vec2;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let editor = rust_text_editor::TextEditor::default();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(700., 500.)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(Box::new(editor), native_options);
}
