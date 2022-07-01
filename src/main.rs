use gwater_installer::GApp;

fn main() {
    let options = eframe::NativeOptions {
        max_window_size: Some(egui::vec2(500.0, 100.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Gwater Installer",
        options,
        Box::new(|_cc| Box::new(GApp::default())),
    );
}
