// Here we have the thread responsible for converting speech to text.

use coqui_stt::{Model, Stream};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

pub enum DecodedSpeech {
    Intermediate(String),
    Final(String),
}

const MODEL_DIR: &str = "en-model";

fn stter(recorder_receiver: Receiver<Option<Vec<i16>>>, gui_sender: Sender<DecodedSpeech>) {
    let (model_name, scorer_name) = get_model_scorer_names();
    let mut m = Model::new(model_name.to_str().expect("invalid utf-8 found in path")).unwrap();
    // Enable external scorer if found in the model folder.
    if let Some(scorer) = scorer_name {
        let scorer = scorer.to_str().expect("invalid utf-8 found in path");
        println!("Using external scorer `{}`", scorer);
        m.enable_external_scorer(scorer).unwrap();
    }

    let model = Arc::new(m);
    // Todo: perhaps this thread should exit sometimes?
    // And it shouldn't be Option that gets received but rather Option or exit?
    // if exit then au revoi the thread innit.
    loop {
        let mut stream =
            Stream::from_model(Arc::clone(&model)).expect("Model creation failed miserably");

        loop {
            let maybe_audio = recorder_receiver
                .recv()
                .expect("Audio receival failed miserably");
            match maybe_audio {
                Some(audio) => {
                    stream.feed_audio(&audio[..]);
                    let intermediate = stream.intermediate_decode();
                    if intermediate.is_ok() {
                        gui_sender
                            .send(DecodedSpeech::Intermediate(intermediate.unwrap()))
                            .expect("Sending of decoded speech faied miserably.");
                    }
                }
                None => {
                    let final_s = stream.finish_stream();
                    if final_s.is_ok() {
                        gui_sender
                            .send(DecodedSpeech::Final(final_s.unwrap()))
                            .expect("Sending of decoded speech failed miserably");
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
