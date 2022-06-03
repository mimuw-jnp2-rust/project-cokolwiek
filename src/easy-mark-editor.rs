use eframe::egui::{text_edit::CCursorRange, *};
use eframe::{egui, epi};
use rfd::{FileDialog, MessageDialog};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct TextEditor {
    code: String,
    show_rendered: bool,
    file_path: Option<PathBuf>,
    should_exit: bool,
    is_exiting: bool,
}

impl PartialEq for TextEditor {
    fn eq(&self, other: &Self) -> bool {
        (&self.code, self.show_rendered) == (&other.code, other.show_rendered)
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        Self {
            code: String::new(),
            show_rendered: true,
            file_path: None,
            should_exit: false,
            is_exiting: false,
        }
    }
}

impl epi::App for TextEditor {
    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        self.should_exit
    }

    fn name(&self) -> &str {
        "Rust text editor"
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &epi::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            if ui.button(format!("{:^17}", "Quit")).clicked() {
                eprintln!("Quitting via the 'Quit' button");
                frame.quit();
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
                eprintln!("Quitting via ctrl-q");
                frame.quit();
            }

            if self.is_exiting {
                self.should_exit = self.quit();
                if self.should_exit {
                    frame.quit();
                } else {
                    self.is_exiting = false;
                }
            }
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

impl TextEditor {
    fn open(&mut self) -> Result<(), std::io::Error> {
        let path = FileDialog::new()
            .set_directory("~/")
            .pick_file()
            .unwrap_or_default();

        eprintln!("path: {}", path.display());

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        self.code.clear();
        match file.read_to_string(&mut self.code) {
            Ok(_) => self.file_path = Some(path),
            Err(err) => return Err(err),
        }
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

        eprintln!("path: {}", path.display());

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        match file.write_all(self.code.as_bytes()) {
            Ok(_) => self.file_path = Some(path),
            Err(err) => return Err(err),
        }

        Ok(())
    }

    // true if editor can exit (save not requested or succeed)
    fn quit(&mut self) -> bool {
        let mess = MessageDialog::new()
            .set_title("Quit")
            .set_description("Do you want to save before quitting?")
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
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("controls").show(ui, |ui| {
            ui.checkbox(&mut self.show_rendered, "Show rendered");
            ui.end_row();
            egui::reset_button(ui, self);
            ui.end_row();
        });

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
                    .desired_rows(100),
            )
        };

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
