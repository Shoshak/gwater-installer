use std::{
    path::{Path, PathBuf},
    thread::JoinHandle,
};

mod consts;
mod downloader;
mod remover;

#[derive(Default)]
pub struct GApp {
    garrysmod_path: Option<PathBuf>,
    download_thread: Option<JoinHandle<bool>>,
    wrong_folder: bool,
    modules_deleted: bool,
    download_finished: bool,
}

impl eframe::App for GApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.heading("Locate '...steam/steamapps/common/GarrysMod' folder");
                    if ui.button(egui::RichText::new("Open").size(25.0)).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            if validate_path(&path) {
                                self.garrysmod_path = Some(path);
                                self.wrong_folder = false;
                            } else {
                                self.wrong_folder = true;
                            }
                        }
                    }

                    ui.separator();

                    ui.add_enabled_ui(
                        self.garrysmod_path.is_some()
                            && !self.wrong_folder
                            && !self.download_thread.as_ref().is_some(),
                        |ui| {
                            if ui.button(egui::RichText::new("Start").size(25.0)).clicked() {
                                self.download_finished = false;
                                let garryclonepath = self.garrysmod_path.clone().unwrap();
                                self.download_thread = Some(std::thread::spawn(move || {
                                    downloader::download_files(&garryclonepath);
                                    true
                                }));
                            }
                            if ui
                                .button(egui::RichText::new("Delete").size(25.0))
                                .clicked()
                            {
                                self.download_finished = false;
                                remover::remove_modules(self.garrysmod_path.as_ref().unwrap());
                                self.modules_deleted = true;
                            }
                        },
                    );

                    if self.download_thread.as_ref().is_some() {
                        if self.download_thread.as_ref().unwrap().is_finished() {
                            self.download_thread = None;
                            self.download_finished = true;
                        } else {
                            ui.heading("Download in progress...");
                        }
                    } else if self.download_finished {
                        ui.heading("Download finished! Enjoy :)");
                    } else if self.wrong_folder {
                        ui.heading("The folder you chose is not the root 'GarrysMod' folder!");
                    } else if self.modules_deleted {
                        ui.heading("GWater deleted :(");
                    } else if self.garrysmod_path.is_some() {
                        ui.heading("Folder seems correct, press start to begin!");
                    }
                },
            );
        });
    }
}

fn validate_path(path: &Path) -> bool {
    let mut test_path = path.clone().to_path_buf();
    test_path.push("garrysmod");
    if !test_path.is_dir() {
        return false;
    }
    test_path.push("lua/bin");
    if !test_path.is_dir() {
        return false;
    }
    return true;
}
