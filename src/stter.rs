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

const MODEL_DIR: &str = "en-model";

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

    loop {
        let mut stream =
            Stream::from_model(Arc::clone(&model)).expect("Model creation failed miserably");

        loop {
            let recorder_msg = recorder_receiver
                .recv()
                .expect("Audio receival failed miserably");

            // We are told to shutdown so we do so.
            if recorder_msg.is_none() {
                return;
            }

            let maybe_audio = recorder_msg.unwrap();
            match maybe_audio {
                Some(audio) => {
                    eprintln!("[stter] Got Some(audio)");
                    // We got send some new audio to process.
                    stream.feed_audio(&audio[..]);
                    let intermediate = stream.intermediate_decode();
                    if intermediate.is_ok() {
                        gui_sender
                            .send(DecodedSpeech::Intermediate(intermediate.unwrap()))
                            .expect("Sending of decoded speech faied miserably.");
                    }
                }
                None => {
                    eprintln!("[stter] Got told to fuck off, finishing the stream then");
                    // We got a "end of recording" message.
                    let final_s = stream.finish_stream();
                    if final_s.is_ok() {
                        gui_sender
                            .send(DecodedSpeech::Final(final_s.unwrap()))
                            .expect("Sending of decoded speech failed miserably");
                    } else {
                        eprintln!("Failed to finish the stream: {}", final_s.unwrap_err());
                    }
                    break;
                }
            };
        }
    }
}

fn get_model_scorer_names() -> (Box<Path>, Option<Box<Path>>) {
    let dir_path = Path::new(MODEL_DIR);
    let mut model_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
    let mut scorer_name: Option<Box<Path>> = None;
    // search for model in model directory
    for file in dir_path
        .read_dir()
        .expect("Specified model dir is not a dir")
    {
        if let Ok(f) = file {
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
    }
    (model_name, scorer_name)
}