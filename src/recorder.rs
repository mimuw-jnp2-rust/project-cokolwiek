// The thread responsible for recording user audio and sending it to stt.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub enum AudioMessage {
    Audio(Vec<i16>),
    EndOf,
    Exit,
}

pub enum GuiOrders {
    Record,
    Stop,
    Exit,
}

pub fn recorder(gui_receiver: Receiver<GuiOrders>, stter_sender: Sender<AudioMessage>) {
    let host = cpal::default_host();
    // todo: test this with a microphone from headphones
    let dev = host.default_input_device().expect("No input device!");
    let config = dev
        .supported_input_configs()
        .expect("Failed to find any config")
        .find(|c| c.sample_format() == cpal::SampleFormat::I16)
        .expect("Failed to find required input device config ie i16.");

    eprintln!(
        "[recorder] Avalaible sample rate is between {} and {} Hz.",
        config.min_sample_rate().0,
        config.max_sample_rate().0
    );

    eprintln!("[recorder] there are {} channel(s)", config.channels());
    // todo: make this work for stereo? do i really need to though?
    // stereo_to_mono function avalaible here:
    // https://github.com/tazz4843/coqui-stt/blob/master/examples/threads.rs
    // at the bottom
    assert!(config.channels() == 1);
    assert!(config.min_sample_rate().0 <= 16000 && 16000 <= config.max_sample_rate().0);

    // todo: perhaps this should be rather set according to what the model says?
    // though obv the coqui models all run on 16 kHz so maybe hardcode it
    // => or make a const u32 for that? (good idea)
    let sr = cpal::SampleRate(16000);
    eprintln!(
        "[recorder] Creating a stream with sample rate = {} Hz.",
        sr.0
    );
    let config = config.with_sample_rate(sr);

    let err_fn = move |err| eprintln!("Error on stream: {}", err);

    // shared data for the asynchronous callback fn
    let should_send = Arc::new(Mutex::new(false));
    let should_send2 = should_send.clone();

    let stt_sender = Arc::new(Mutex::new(stter_sender));
    let stt_sender2 = stt_sender.clone();

    let record_callback = move |data: &[i16], _: &_| {
        if *should_send.lock().expect("Poisoned should_send mutex!") {
            let data = data.to_vec();
            stt_sender
                .lock()
                .expect("Poisoned stt_sender mutex!")
                .send(AudioMessage::Audio(data))
                .expect("Failed to send data to stter!");
        }
    };

    let stt_sender = stt_sender2;
    let should_send = should_send2;

    let stream = dev
        .build_input_stream(&config.into(), record_callback, err_fn)
        .expect("Failed to construct an input stream!");

    // todo: perhaps pause error shouldn't be fatal, check on other machines
    stream.pause().expect("faield to pause upon creation?");

    loop {
        match gui_receiver
            .recv()
            .expect("Failed to receive messages from gui!")
        {
            GuiOrders::Record => {
                // sleep for a short while to make sure it starts smoothly
                std::thread::sleep(Duration::from_millis(75));
                {
                    *should_send.lock().expect("Poisoned should_send mutex!") = true;
                }
                stream.play().expect("Failed to start recording!");
            }
            GuiOrders::Stop => {
                stream.pause().expect("Failed to stop recording!");
                {
                    *should_send.lock().expect("Poisoned should_send mutex!") = false;
                }
                stt_sender
                    .lock()
                    .expect("Poisoned stt_sender mutex!")
                    .send(AudioMessage::EndOf)
                    .expect("Failed to send None to the stter!");
                eprintln!("[recorder] Sent EndOf to the stter.");
            }
            GuiOrders::Exit => {
                eprintln!("[recorder] Told to exit, doing so and notifying the stter.");
                stt_sender
                    .lock()
                    .expect("Poisoned stt_sender mutex!")
                    .send(AudioMessage::Exit)
                    .expect("Failed to send Exit to the stter!");
                return;
            }
        }
    }
}
