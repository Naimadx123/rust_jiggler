#![windows_subsystem = "windows"]

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use enigo::{Coordinate, Enigo, Mouse, Settings};

use eframe::egui;
use eframe::glow::Context;

struct JigglerApp {
    running: bool,
    should_run: Arc<AtomicBool>,
}

impl JigglerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            running: false,
            should_run: Arc::new(AtomicBool::new(false)),
        }
    }

    fn start_jiggler(&mut self) {
        if self.running {
            return;
        }

        self.running = true;
        self.should_run.store(true, Ordering::SeqCst);

        let flag = self.should_run.clone();

        thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            let interval = Duration::from_secs(30);

            while flag.load(Ordering::SeqCst) {
                if let Err(e) = enigo.move_mouse(1, 0, Coordinate::Rel) {
                    eprintln!("Cannot move the mouse: {e}");
                }
                thread::sleep(Duration::from_millis(50));
                if let Err(e) = enigo.move_mouse(-1, 0, Coordinate::Rel) {
                    eprintln!("Cannot move the mouse: {e}");
                }

                let mut elapsed = Duration::from_secs(0);
                while elapsed < interval && flag.load(Ordering::SeqCst) {
                    let step = Duration::from_millis(200);
                    thread::sleep(step);
                    elapsed += step;
                }
            }
        });
    }

    fn stop_jiggler(&mut self) {
        self.should_run.store(false, Ordering::SeqCst);
        self.running = false;
    }
}

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

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mouse Jiggler",
        options,
        Box::new(|cc| Ok(Box::new(JigglerApp::new(cc)))),
    )
}
