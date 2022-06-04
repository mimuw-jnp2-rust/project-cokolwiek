// The thread responsible for recording user audio and sending it through to stt.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub enum GuiOrders {
    Record,
    Stop,
    Exit,
}

fn recorder(gui_receiver: Receiver<GuiOrders>, stter_sender: Sender<Option<Vec<i16>>>) {
    let host = cpal::default_host();
    let dev = host.default_input_device().expect("No input device!");
    let config = dev
        .supported_input_configs()
        .expect("Failed to find any config")
        .find(|c| c.sample_format() == cpal::SampleFormat::I16)
        .expect("Failed to find required input device config ie. i16 (??)")
        .with_max_sample_rate();

    let err_fn = move |err| {
        eprintln!("Error on stream: {}", err);
    };

    let stt_sender = Arc::new(Mutex::new(stter_sender));
    let stt_sender2 = stt_sender.clone();
    // todo: stt sender should get mutexed as it should send both here and in the
    // bottom most loop!
    let record_callback = move |data: &[i16], _: &_| {
        let data = data.to_vec();
        stt_sender
            .lock()
            .expect("Poisoned mutex!")
            .send(Some(data))
            .expect("Failed to send data to stter!");
        // stter_sender.send(Some()
    };

    let stt_sender = stt_sender2;

    let stream = dev
        .build_input_stream(&config.into(), record_callback, err_fn)
        .expect("Failed to construct an input stream!");

    loop {
        match gui_receiver
            .recv()
            .expect("Failed to receive messages from gui!")
        {
            GuiOrders::Record => stream.play().expect("Failed to start recording!"),
            GuiOrders::Stop => {
                stream.pause().expect("Failed to stop recording!");
                stt_sender
                    .lock()
                    .expect("Mutex poisoned!")
                    .send(None)
                    .expect("Failed to send None to the stter!");
            }
            GuiOrders::Exit => {
                // todo: we should send something more complex here
                stt_sender
                    .lock()
                    .expect("Mutex poisoned!")
                    .send(None)
                    .expect("Failed to send None to the stter!");
                return;
            }
        }
    }
}
