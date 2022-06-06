use eframe::egui::vec2;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use rust_shit_editor::recorder;
use rust_shit_editor::stter;

fn main() {
    let (gui_sender, stter_receiver) = channel::<stter::DecodedSpeech>();
    let (stter_sender, recorder_receiver) = channel::<recorder::AudioMessage>();
    let (recorder_sender, gui_receiver) = channel::<recorder::GuiOrders>();

    let stter_jh = thread::spawn(move || {
        stter::stter(recorder_receiver, gui_sender);
    });

    let recorder_jh = thread::spawn(move || {
        recorder::recorder(gui_receiver, stter_sender);
    });

    // todo: make this closure work
    // https://github.com/rust-lang/rust/issues/97362
    // ^ this shoyld fix that in fututre i guess??
    // let editor = move |cc| {
    //     let boexed: Box<dyn eframe::App> = Box::new(rust_shit_editor::TextEditor::new(
    //         cc,
    //         stter_receiver,
    //         recorder_sender,
    //     ));
    //     boexed
    // };

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(700., 500.)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "rust shit editor",
        native_options,
        Box::new(|cc| init_ed(cc, stter_receiver, recorder_sender, recorder_jh, stter_jh)),
    );
}

// this is here as the closure above does not work due to some rust clownery
// https://github.com/rust-lang/rust/issues/70263
fn init_ed(
    cc: &eframe::CreationContext<'_>,
    stter_receiver: Receiver<stter::DecodedSpeech>,
    recorder_sender: Sender<recorder::GuiOrders>,
    recorder_jh: thread::JoinHandle<()>,
    stter_jh: thread::JoinHandle<()>,
) -> Box<dyn eframe::App> {
    // https://stackoverflow.com/questions/43725433/why-cant-a-struct-be-assigned-to-a-binding-with-a-trait-it-implements
    let boexed: Box<dyn eframe::App> = Box::new(rust_shit_editor::TextEditor::new(
        cc,
        stter_receiver,
        recorder_sender,
        vec![recorder_jh, stter_jh],
    ));
    boexed
}
