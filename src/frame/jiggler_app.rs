use eframe::glow::Context;
use crate::frame::JigglerApp;

impl eframe::App for JigglerApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mouse Jiggler");
            ui.separator();

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                // Toggle button for starting/stopping the jiggler
                let button_text = if self.running { "STOP" } else { "START" };
                if ui.add(egui::Button::new(button_text).min_size(egui::vec2(100.0, 40.0))).clicked() {
                    if self.running {
                        self.stop_jiggler();
                    } else {
                        self.start_jiggler();
                    }
                }
            });

            ui.add_space(20.0);
            ui.label("Close the window to stop the jiggler.");
        });
    }

    /// Called when the application is closing.
    fn on_exit(&mut self, _gl: Option<&Context>) {
        // Ensure the jiggler thread is stopped when exiting
        self.stop_jiggler();
    }
}