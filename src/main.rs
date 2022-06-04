#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::sync::mpsc::channel;
use eframe::egui::vec2;

use rust_shit_editor::stter;
use rust_shit_editor::recorder;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let (gui_sender, stter_receiver) = channel::<stter::DecodedSpeech>();
    let (stter_sender, recorder_receiver) = channel::<recorder::AudioMessage>();
    let (recorder_sender, gui_receiver) = channel::<recorder::GuiOrders>();
    // todo: join handles should be part of the TextEditor as well since eframe
    // run native makes everything below it unreachable as it is now it is
    // running those threads in a detached mode.
    let _stter_th = std::thread::spawn(move || {
        stter::stter(recorder_receiver, gui_sender);
    });

    let _recorder_th = std::thread::spawn(move || {
        recorder::recorder(gui_receiver, stter_sender);
    });

    let editor = rust_shit_editor::TextEditor::new(stter_receiver, recorder_sender);
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(700., 500.)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(Box::new(editor), native_options);

    // todo: the code below would have been unreachable thus it is advised that
    // we add the JoinHandles for these two threads as fields in our app and
    // join there upon quit
    // _recorder_th.join().expect("Failed to join the recorder thread!");
    // _stter_th.join().expect("Failed to join the stter thread!");
}
