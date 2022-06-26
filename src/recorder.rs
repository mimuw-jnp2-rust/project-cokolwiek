// The thread responsible for recording user audio and sending it to stt.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use log::{trace, info, error};
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

const STT_SAMPLE_RATE: u32 = 16000;

pub fn recorder(gui_receiver: Receiver<GuiOrders>, stter_sender: Sender<AudioMessage>) {
    let host = cpal::default_host();
    let dev = host.default_input_device().expect("No input device!");
    let config = dev
        .supported_input_configs()
        .expect("Failed to find any config")
        .find(|c| c.sample_format() == cpal::SampleFormat::I16)
        .expect("Failed to find required input device config ie i16.");

    info!(
        "Avalaible sample rate is between {} and {} Hz.",
        config.min_sample_rate().0,
        config.max_sample_rate().0
    );

    info!("there are {} channel(s)", config.channels());
    // should i make this work for stereo? do i really need to though?
    // computer microphone usually is stereo but this should be tested more
    // thoroughly probably
    // stereo_to_mono function avalaible here:
    // https://github.com/tazz4843/coqui-stt/blob/master/examples/threads.rs
    // at the bottom
    assert!(config.channels() == 1);
    assert!(
        config.min_sample_rate().0 <= STT_SAMPLE_RATE
            && STT_SAMPLE_RATE <= config.max_sample_rate().0
    );

    let sr = cpal::SampleRate(STT_SAMPLE_RATE);
    let config = config.with_sample_rate(sr);

    let should_send = Arc::new(Mutex::new(false));

    let mut stream;

    loop {
        match gui_receiver
            .recv()
            .expect("Failed to receive messages from gui!")
        {
            GuiOrders::Record => {
                trace!("Starting the recording, creating new stream.");
                // Shared data for the asynchronous callback fn.
                let should_send2 = should_send.clone();
                let stter_sender2 = stter_sender.clone();

                let record_callback = move |data: &[i16], _: &_| {
                    if *should_send2.lock().expect("Poisoned should_send mutex!") {
                        let data = data.to_vec();
                        stter_sender2
                            .send(AudioMessage::Audio(data))
                            .expect("Failed to send audio data to stter!");
                    }
                };

                let err_fn = |err| error!("Error on stream: {}", err);

                let config = config.clone();
                stream = dev
                    .build_input_stream(&config.into(), record_callback, err_fn)
                    .expect("Failed to construct an input stream!");

                {
                    *should_send.lock().expect("Poisoned should_send mutex!") = true;
                }
                stream.play().expect("Failed to start recording!");
            }
            GuiOrders::Stop => {
                trace!("Being told to stop.");
                {
                    *should_send.lock().expect("Poisoned should_send mutex!") = false;
                }
                // sleep here so that the record_callback stops smoothly
                std::thread::sleep(Duration::from_millis(50));
                stter_sender
                    .send(AudioMessage::EndOf)
                    .expect("Failed to send EndOf to the stter!");
                trace!("Sent EndOf to the stter.");
            }
            GuiOrders::Exit => {
                trace!("Told to exit, doing so and notifying the stter.");
                stter_sender
                    .send(AudioMessage::Exit)
                    .expect("Failed to send Exit to the stter!");
                return;
            }
        }
    }
}
