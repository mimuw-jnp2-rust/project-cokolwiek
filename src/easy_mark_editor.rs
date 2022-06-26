use eframe::egui;
use eframe::egui::{text_edit::CCursorRange, *};
use log::trace;
use rfd::{FileDialog, MessageDialog};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use crate::recorder::GuiOrders;
use crate::stter::DecodedSpeech;

pub struct TextEditor {
    code: String,
    show_rendered: bool,
    file_path: Option<PathBuf>,
    has_changed: bool,
    // Quit only when sure we want to quit.
    should_exit: bool,
    is_exiting: bool,
    // Channels for passing messages regarding voice-writing.
    stter_receiver: Receiver<DecodedSpeech>,
    recorder_sender: Sender<GuiOrders>,
    // Coordinating waiting for decoded speech.
    is_recording: bool,
    is_stopping: bool,
    // Keep that when receiving intermediate results.
    backup_code: String,
    // Join handles for all threads communicating with the editor.
    jhandles: Vec<JoinHandle<()>>,
}

impl PartialEq for TextEditor {
    fn eq(&self, other: &Self) -> bool {
        (&self.code, self.show_rendered) == (&other.code, other.show_rendered)
    }
}

impl eframe::App for TextEditor {
    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        self.should_exit
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            if ui.button(format!("{:^17}", "Quit")).clicked() {
                trace!("Quitting via the 'Quit' button");
                frame.quit();
            }

            // This part of ui handles stt-ing.
            if self.is_recording && self.is_stopping {
                ui.spinner();
            } else {
                let dictate_bttn = if !self.is_recording && !self.is_stopping {
                    "Dictate"
                } else {
                    "Stop"
                };

                if ui.button(format!("{:^16}", dictate_bttn)).clicked() {
                    if !self.is_recording {
                        self.start_recording();
                    } else if !self.is_stopping {
                        trace!("Asking the recorder to stop recording.");
                        self.recorder_sender
                            .send(GuiOrders::Stop)
                            .expect("Failed to send recording-stopping message!");
                        self.is_stopping = true;
                    }
                }
            }

            if ui.button(format!("{:^13}", "Open file")).clicked() && matches!(self.open(), Err(_))
            {
                MessageDialog::new()
                    .set_title("File is opening")
                    .set_description("Failed to open the file")
                    .set_buttons(rfd::MessageButtons::Ok)
                    .show();
            }

            if ui.button(format!("{:^15}", "Save file")).clicked()
                && matches!(self.save(false), Err(_))
            {
                MessageDialog::new()
                    .set_title("File is saving")
                    .set_description("Failed to save the file")
                    .set_buttons(rfd::MessageButtons::Ok)
                    .show();
            }

            if ui.button(format!("{:^13}", "Save file as")).clicked()
                && matches!(self.save(true), Err(_))
            {
                MessageDialog::new()
                    .set_title("Saving file as")
                    .set_description("Failed to save the file")
                    .set_buttons(rfd::MessageButtons::Ok)
                    .show();
            }

            if ui.input_mut().consume_key(egui::Modifiers::COMMAND, Key::Q) {
                trace!("Quitting via ctrl-q");
                frame.quit();
            }

            // This is here to prevent the default response to clicking the X button
            // and check if we can safely exit.
            if self.is_exiting {
                self.should_exit = self.quit();
                if self.should_exit {
                    self.recorder_sender
                        .send(GuiOrders::Exit)
                        .expect("Failed to send Exit to recorder!");

                    let mut jhandles = vec![];
                    std::mem::swap(&mut jhandles, &mut self.jhandles);
                    for jh in jhandles {
                        jh.join().expect("Failure upon joining a thread!");
                    }

                    // Here we actually quit for real hence all of the above.
                    trace!("End of times.");
                    frame.quit();
                } else {
                    self.is_exiting = false;
                }
            }
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });

        if self.is_stopping {
            self.end_recording();
        } else if self.is_recording {
            self.manage_recording();
            // need to check for incoming results quite often hence the request
            ctx.request_repaint();
        }
    }
}

impl TextEditor {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        stter_receiver: Receiver<DecodedSpeech>,
        recorder_sender: Sender<GuiOrders>,
        jhandles: Vec<JoinHandle<()>>,
    ) -> Self {
        Self {
            code: String::new(),
            show_rendered: true,
            file_path: None,
            has_changed: true,
            should_exit: false,
            is_exiting: false,
            stter_receiver,
            recorder_sender,
            is_recording: false,
            is_stopping: false,
            backup_code: String::new(),
            jhandles,
        }
    }

    fn open(&mut self) -> Result<(), std::io::Error> {
        let path = FileDialog::new()
            .set_directory("~/")
            .pick_file()
            .unwrap_or_default();

        trace!("path: {}", path.display());

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        self.code.clear();
        match file.read_to_string(&mut self.code) {
            Ok(_) => self.file_path = Some(path),
            Err(err) => return Err(err),
        }
        self.has_changed = false;
        Ok(())
    }

    fn save(&mut self, save_as: bool) -> Result<(), std::io::Error> {
        let path = match (&self.file_path, save_as) {
            (Some(p), false) => p.clone(),
            _ => FileDialog::new()
                .set_title("Save file")
                .set_directory("~/")
                .save_file()
                .unwrap_or_default(),
        };

        trace!("path: {}", path.display());

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        match file.write_all(self.code.as_bytes()) {
            Ok(_) => self.file_path = Some(path),
            Err(err) => return Err(err),
        }

        self.has_changed = false;
        Ok(())
    }

    // true if editor can exit (save not requested or succeed)
    fn quit(&mut self) -> bool {
        // Cannot quit while recording.
        if self.is_recording || self.is_stopping {
            return false;
        }
        // the editr asks for confirmation only if there are any unsaved changes
        if self.has_changed {
            let mess = MessageDialog::new()
                .set_title("Quit")
                .set_description("Do you want to save your changes??")
                .set_buttons(rfd::MessageButtons::YesNo)
                .show();

            if mess {
                match self.save(false) {
                    Ok(()) => true,
                    Err(_) => {
                        MessageDialog::new()
                            .set_title("Quitting")
                            .set_description("Failed to save the file")
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();
                        false
                    }
                }
            } else {
                true
            }
        } else {
            true
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("controls").show(ui, |ui| {
            ui.checkbox(&mut self.show_rendered, "Show rendered");
            ui.end_row();
        });

        // Display the file name if we have an assigned file
        if let Some(p) = &self.file_path {
            // in rust the path may be really f-ed up so it is all options
            if let Some(s) = p.file_name().and_then(|s| s.to_str()) {
                if self.has_changed {
                    ui.label(format!("{}*", s));
                } else {
                    ui.label(s.to_string());
                }
            }
        }

        ui.separator();

        if self.show_rendered {
            ui.columns(2, |columns| {
                ScrollArea::vertical()
                    .id_source("source")
                    .show(&mut columns[0], |ui| self.editor_ui(ui));
                ScrollArea::vertical()
                    .id_source("rendered")
                    .show(&mut columns[1], |ui| {
                        crate::easy_mark_viewer::easy_mark(ui, &self.code);
                    });
            });
        } else {
            ScrollArea::vertical()
                .id_source("source")
                .show(ui, |ui| self.editor_ui(ui));
        }
    }

    fn editor_ui(&mut self, ui: &mut egui::Ui) {
        let Self { code, .. } = self;

        let response = {
            ui.add(
                egui::TextEdit::multiline(code)
                    .lock_focus(true)
                    .hint_text("Type here...")
                    .desired_width(f32::INFINITY)
                    .font(egui::FontId::monospace(15.)) // for cursor height
                    .desired_rows(100)
                    .interactive(!self.is_recording), // not writable while recording
            )
        };

        // Unsaved changes!
        if response.changed() {
            self.has_changed = true;
        }

        if let Some(mut state) = TextEdit::load_state(ui.ctx(), response.id) {
            if let Some(mut ccursor_range) = state.ccursor_range() {
                let any_change = shortcuts(ui, code, &mut ccursor_range);
                if any_change {
                    state.set_ccursor_range(Some(ccursor_range));
                    state.store(ui.ctx(), response.id);
                }
            }
        }
    }

    fn start_recording(&mut self) {
        self.is_recording = true;
        self.recorder_sender
            .send(GuiOrders::Record)
            .expect("Failed to send a recording-starting message!");
        self.backup_code = self.code.clone();
    }

    // Both manage_recording and end_recording receive info from the stter in
    // a non-blocking fashion as we want the gui to still be refreshed often enough
    // and respond to our clicks etc.
    fn manage_recording(&mut self) {
        let speech = self.stter_receiver.try_recv();
        if let Err(err) = speech {
            match err {
                std::sync::mpsc::TryRecvError::Empty => return,
                std::sync::mpsc::TryRecvError::Disconnected => panic!("Failed to receive!"),
            };
        }
        let speech = speech.unwrap();

        match speech {
            DecodedSpeech::Intermediate(s) => {
                self.code = self.backup_code.clone();
                self.code.push_str(&s);
                self.code.push_str("...");
            }
            DecodedSpeech::Final(_) => panic!(
                "Editor logic error! We should only receive intermediate decoded text fragments now!"
            ),
        };
    }

    fn end_recording(&mut self) {
        let speech = self.stter_receiver.try_recv();
        if let Err(err) = speech {
            match err {
                std::sync::mpsc::TryRecvError::Empty => return,
                std::sync::mpsc::TryRecvError::Disconnected => panic!("Failed to receive!"),
            };
        }
        let speech = speech.unwrap();

        match speech {
            DecodedSpeech::Intermediate(s) => {
                self.code = self.backup_code.clone();
                self.code.push_str(&s);
                self.code.push_str("...");
            }
            DecodedSpeech::Final(s) => {
                self.code = self.backup_code.clone();
                self.backup_code = String::new();
                self.code.push_str(&s);
                // only now the recording is done and fully processed
                self.is_recording = false;
                self.is_stopping = false;
            }
        };
    }
}

fn shortcuts(ui: &Ui, code: &mut dyn TextBuffer, ccursor_range: &mut CCursorRange) -> bool {
    let mut any_change = false;
    for (key, surrounding) in [
        (Key::B, "*"), // *bold*
        (Key::D, "`"), // `code`
        (Key::I, "/"), // /italics/
        (Key::L, "$"), // $subscript$
        (Key::R, "^"), // ^superscript^
        (Key::S, "~"), // ~strikethrough~
        (Key::U, "_"), // _underline_
    ] {
        if ui.input_mut().consume_key(egui::Modifiers::COMMAND, key) {
            toggle_surrounding(code, ccursor_range, surrounding);
            any_change = true;
        };
    }
    any_change
}

/// E.g. toggle *strong* with `toggle_surrounding(&mut text, &mut cursor, "*")`
fn toggle_surrounding(
    code: &mut dyn TextBuffer,
    ccursor_range: &mut CCursorRange,
    surrounding: &str,
) {
    let [primary, secondary] = ccursor_range.sorted();

    let surrounding_ccount = surrounding.chars().count();

    let prefix_crange = primary.index.saturating_sub(surrounding_ccount)..primary.index;
    let suffix_crange = secondary.index..secondary.index.saturating_add(surrounding_ccount);
    let already_surrounded = code.char_range(prefix_crange.clone()) == surrounding
        && code.char_range(suffix_crange.clone()) == surrounding;

    if already_surrounded {
        code.delete_char_range(suffix_crange);
        code.delete_char_range(prefix_crange);
        ccursor_range.primary.index -= surrounding_ccount;
        ccursor_range.secondary.index -= surrounding_ccount;
    } else {
        code.insert_text(surrounding, secondary.index);
        let advance = code.insert_text(surrounding, primary.index);

        ccursor_range.primary.index += advance;
        ccursor_range.secondary.index += advance;
    }
}
