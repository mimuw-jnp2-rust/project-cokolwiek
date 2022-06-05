// The thread responsible for recording user audio and sending it through to stt.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

// todo: this should be redesigned actually
// the stter should always start with a totally new and clean stream i think
// and i am not sure if this is what happens as on subsequent it works
// differently?...
pub type AudioMessage = Option<Option<Vec<i16>>>;

// todo: this should be moved to the gui module for consistency (consistency ie
// each module defines what it sends)
pub enum GuiOrders {
    Record,
    Stop,
    Exit,
}

// todo mutex for a bool to check whether we should send or not as the
// pause method mya not be reliable?

pub fn recorder(gui_receiver: Receiver<GuiOrders>, stter_sender: Sender<AudioMessage>) {
    let host = cpal::default_host();
    let dev = host.default_input_device().expect("No input device!");
    let config = dev
        .supported_input_configs()
        .expect("Failed to find any config")
        .find(|c| c.sample_format() == cpal::SampleFormat::I16)
        .expect("Failed to find required input device config ie. i16 (??)");

    eprintln!("[recorder] smapele rate is from {} to {}",
              config.min_sample_rate().0, config.max_sample_rate().0);

    eprintln!("[recorder] there are {} channels", config.channels());
    // let sr = config.max_sample_rate();
    // todo: perhaps this should be rather set according to what the model says?
    // though obv the coqui models all run on 16 kHz so maybe hardcode it
    // or make a const u32 for that?
    let sr = cpal::SampleRate(16000);
    eprintln!("[recorder] Creating a stream with sample rate = {} Hz", sr.0);
    let config = config.with_sample_rate(sr);

    let err_fn = move |err| {
        eprintln!("Error on stream: {}", err);
    };

    let should_send = Arc::new(Mutex::new(false));
    let should_send2 = should_send.clone();
    let stt_sender = Arc::new(Mutex::new(stter_sender));
    let stt_sender2 = stt_sender.clone();
    let record_callback = move |data: &[i16], _: &_| {
        if *should_send.lock().expect("Mutex error") {
            let data = data.to_vec();
            stt_sender
                .lock()
                .expect("Poisoned mutex!")
                .send(Some(Some(data)))
                .expect("Failed to send data to stter!");
        }
    };

    let stt_sender = stt_sender2;
    let should_send = should_send2;

    let stream = dev
        .build_input_stream(&config.into(), record_callback, err_fn)
        .expect("Failed to construct an input stream!");
    stream.pause().expect("faield to pause upon creation?");

    loop {
        match gui_receiver
            .recv()
            .expect("Failed to receive messages from gui!")
        {
            GuiOrders::Record => {
                {
                    *should_send.lock().expect("Mutex failure") = true;
                }
                stream.play().expect("Failed to start recording!");
            },
            GuiOrders::Stop => {
                eprintln!("[recorder] Sending Some(None)) to the stter!");
                stream.pause().expect("Failed to stop recording!");
                {
                    *should_send.lock().expect("Mutex failure") = false;
                }
                // todo: wait here? until some time passes? so all audio is cleared?
                stt_sender
                    .lock()
                    .expect("Mutex poisoned!")
                    .send(Some(None))
                    .expect("Failed to send None to the stter!");
                eprintln!("[recorder] Sent Some(None)!");
            }
            GuiOrders::Exit => {
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
