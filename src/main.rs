// Hide the console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod frame;

use eframe::egui;
use frame::JigglerApp;

fn main() -> eframe::Result<()> {
    // Configuration for the native window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 200.0]),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Mouse Jiggler",
        options,
        Box::new(|cc| Ok(Box::new(JigglerApp::new(cc)))),
    )
}
