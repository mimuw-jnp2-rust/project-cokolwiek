use eframe::egui::vec2;
use std::sync::mpsc::{Receiver, Sender, channel};

use rust_shit_editor::recorder;
use rust_shit_editor::stter;

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

    eframe::run_native("rust shit editor", native_options,
                       Box::new(|cc| init_ed(cc, stter_receiver, recorder_sender)));

    // todo: the code below would have been unreachable thus it is advised that
    // we add the JoinHandles for these two threads as fields in our app and
    // join there upon quit
    // _recorder_th.join().expect("Failed to join the recorder thread!");
    // _stter_th.join().expect("Failed to join the stter thread!");
}

// this is here as the closure above does not work due to some rust clownery
// https://github.com/rust-lang/rust/issues/70263
fn init_ed(cc: &eframe::CreationContext<'_>,
           stter_receiver: Receiver<stter::DecodedSpeech>,
           recorder_sender: Sender<recorder::GuiOrders>) -> Box<dyn eframe::App> {
    // https://stackoverflow.com/questions/43725433/why-cant-a-struct-be-assigned-to-a-binding-with-a-trait-it-implements
    let boexed: Box<dyn eframe::App> = Box::new(rust_shit_editor::TextEditor::new(
        cc,
        stter_receiver,
        recorder_sender,
    ));
    boexed
}
