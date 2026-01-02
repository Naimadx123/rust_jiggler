use eframe::glow::Context;
use crate::frame::JigglerApp;

impl eframe::App for JigglerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mouse Jiggler");
            ui.separator();

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
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

    fn on_exit(&mut self, _gl: Option<&Context>) {
        self.stop_jiggler();
    }
}