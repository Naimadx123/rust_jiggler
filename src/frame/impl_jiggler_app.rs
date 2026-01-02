use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use enigo::{Coordinate, Enigo, Settings, Mouse};

/// Main application state
pub struct JigglerApp {
    /// Whether the jiggler is currently active in the UI
    pub running: bool,
    /// Shared flag to signal the background thread to stop
    pub should_run: Arc<AtomicBool>,
}

impl JigglerApp {
    /// Initialize a new instance of the application
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            running: false,
            should_run: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Starts the mouse jiggling background thread
    pub(crate) fn start_jiggler(&mut self) {
        if self.running {
            return;
        }

        self.running = true;
        self.should_run.store(true, Ordering::SeqCst);

        let flag = self.should_run.clone();

        // Spawn a background thread to move the mouse periodically
        thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            let interval = Duration::from_secs(30);

            while flag.load(Ordering::SeqCst) {
                // Move the mouse slightly to prevent the system from idling
                if let Err(e) = enigo.move_mouse(1, 0, Coordinate::Rel) {
                    eprintln!("Cannot move the mouse: {e}");
                }
                // Small delay between movements
                thread::sleep(Duration::from_millis(50));
                // Move back to original position
                if let Err(e) = enigo.move_mouse(-1, 0, Coordinate::Rel) {
                    eprintln!("Cannot move the mouse: {e}");
                }

                // Wait for the next interval while checking if we should stop
                let mut elapsed = Duration::from_secs(0);
                while elapsed < interval && flag.load(Ordering::SeqCst) {
                    let step = Duration::from_millis(200);
                    thread::sleep(step);
                    elapsed += step;
                }
            }
        });
    }

    /// Stops the mouse jiggling background thread
    pub(crate) fn stop_jiggler(&mut self) {
        self.should_run.store(false, Ordering::SeqCst);
        self.running = false;
    }
}