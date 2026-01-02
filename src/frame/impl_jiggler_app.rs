use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use enigo::{Coordinate, Enigo, Settings, Mouse};

pub struct JigglerApp {
    pub running: bool,
    pub should_run: Arc<AtomicBool>,
}
impl JigglerApp {
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            running: false,
            should_run: Arc::new(AtomicBool::new(false)),
        }
    }

    pub(crate) fn start_jiggler(&mut self) {
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

    pub(crate) fn stop_jiggler(&mut self) {
        self.should_run.store(false, Ordering::SeqCst);
        self.running = false;
    }
}