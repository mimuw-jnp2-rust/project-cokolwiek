// Here we have the thread responsible for converting speech to text.

use coqui_stt::{Model, Stream};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use crate::recorder::AudioMessage;

#[derive(Debug)]
pub enum DecodedSpeech {
    Intermediate(String),
    Final(String),
}

const DEFAULT_MODEL_DIR: &str = "en-model";

pub fn stter(recorder_receiver: Receiver<AudioMessage>, gui_sender: Sender<DecodedSpeech>) {
    let (model_name, scorer_name) = get_model_scorer_names();
    let mut m = Model::new(model_name.to_str().expect("invalid utf-8 found in path")).unwrap();
    // Enable external scorer if found in the model folder.
    if let Some(scorer) = scorer_name {
        let scorer = scorer.to_str().expect("invalid utf-8 found in path");
        println!("Using external scorer `{}`", scorer);
        m.enable_external_scorer(scorer).unwrap();
    }

    let model = Arc::new(m);
    let sr = model.get_sample_rate() as u32;
    eprintln!("Model's expected sample rate is {}", sr);

    loop {
        eprintln!("[stter] creating new stream...");
        // todo: i only create new stream each time, should i create new model?
        // there is some weird behaviour, all im sayin'
        // todo model.tostream?
        let mut stream =
            Stream::from_model(Arc::clone(&model)).expect("Model creation failed miserably");

        // we do not want intermediate decode after each sample as
        // they are sampled at a high rate and an average person says
        // something like 4-5 words per second
        let mut counter: u32 = 0;

        loop {
            let maybe_audio = recorder_receiver
                .recv()
                .expect("Audio receival failed miserably");

            match maybe_audio {
                AudioMessage::Audio(audio) => {
                    if counter == 0 {
                        eprintln!("[stter] Received first bit of new recording");
                    }
                    counter += 1;
                    // We got send some new audio to process.
                    stream.feed_audio(&audio[..]);

                    // Send only intermediate results just so often.
                    // todo: why 100?
                    if counter % 100 != 0 {
                        continue;
                    }

                    let intermediate = stream.intermediate_decode();
                    if let Ok(intermediate) = intermediate {
                        eprintln!(
                            "[stter] counter = {}, sending intermediate results: \"{}\"",
                            counter, intermediate
                        );

                        gui_sender
                            .send(DecodedSpeech::Intermediate(intermediate))
                            .expect("Sending of decoded speech failed miserably.");
                    }
                }
                AudioMessage::EndOf => {
                    eprintln!("[stter] Got told to end, finishing the stream then");
                    let final_s = stream.finish_stream();
                    if let Ok(final_s) = final_s {
                        gui_sender
                            .send(DecodedSpeech::Final(final_s))
                            .expect("Sending of decoded speech failed miserably");
                    } else {
                        eprintln!("Failed to finish the stream: {}", final_s.unwrap_err());
                    }
                    break;
                }
                AudioMessage::Exit => {
                    eprintln!("[stter] Exiting gracefully.");
                    return;
                }
            };
        }
    }
}

// todo: surely this can be done in smarter way without copying strings
// shouldn't the argv strings have 'static lifetimes anyway as our default?
fn get_model_dir() -> String {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.get(0) {
        Some(s) => s.into(),
        None => DEFAULT_MODEL_DIR.into(),
    }
}

// todo do those need to be boxes? copied without thinkin rly
fn get_model_scorer_names() -> (Box<Path>, Option<Box<Path>>) {
    let model_dir = get_model_dir();
    println!("Looking for a model in the {} directory", model_dir);
    let dir_path = Path::new(&model_dir);
    let mut model_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
    let mut scorer_name: Option<Box<Path>> = None;
    // search for model in model directory
    for f in dir_path
        .read_dir()
        .expect("Specified model dir is not a dir")
        .flatten()
    {
        let file_path = f.path();
        if file_path.is_file() {
            if let Some(ext) = file_path.extension() {
                if ext == "pb" || ext == "pbmm" || ext == "tflite" {
                    model_name = file_path.into_boxed_path();
                } else if ext == "scorer" {
                    scorer_name = Some(file_path.into_boxed_path());
                }
            }
        }
    }
    (model_name, scorer_name)
}
