use std::path::{Path, PathBuf};

mod downloader;

#[derive(Default)]
pub struct GApp {
    garrysmod_path: Option<PathBuf>,
    wrong_folder: bool,
    finished_downloading: bool,
}

impl eframe::App for GApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.heading("Locate '...steam/steamapps/common/GarrysMod' folder");
                    if ui.button("Open...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            if validate_path(&path) {
                                self.garrysmod_path = Some(path);
                                self.wrong_folder = false;
                            } else {
                                self.wrong_folder = true;
                            }
                        }
                    }
                    ui.add_enabled_ui(self.garrysmod_path.is_some() && !self.wrong_folder, |ui| {
                        if ui.button("Start downloading...").clicked() {
                            downloader::download_files(self.garrysmod_path.as_ref().unwrap());
                            self.finished_downloading = true;
                        }
                    });
                    ui.separator();
                    if self.finished_downloading {
                        ui.label("Download finished!");
                    } else if self.wrong_folder {
                        ui.label("The folder you chose does not have the necessary subfolders. Try again!");
                    } else if self.garrysmod_path.is_some() {
                        ui.label("Folder seems correct!");
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
