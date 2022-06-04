// The thread responsible for recording user audio and sending it through to stt.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

pub enum GuiOrders {
    Record,
    Stop,
}

fn recorder(gui_receiver: Receiver<GuiOrders>, stter_sender: Sender<Option<Vec<i16>>>) {
    let host = cpal::default_host();
    let dev = host.default_input_device().expect("No input device!");

    let err_fn = move |err| {
        eprintln!("Error on stream: {}", err);
    };

    let record_callback = move |data: &[i16], _: &_| {
        let data = data.to_vec();
    };

    // todo: config??? we want i16 only do we not, select a specific device for that?
}
