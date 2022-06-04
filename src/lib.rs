#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod easy_mark_editor;
pub mod easy_mark_parser;
mod easy_mark_viewer;

pub use easy_mark_editor::TextEditor;
pub use easy_mark_parser as parser;
pub use easy_mark_viewer::easy_mark;

mod stter;
mod recorder;
