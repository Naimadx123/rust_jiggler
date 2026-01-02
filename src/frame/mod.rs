// Define the frame module components
pub mod jiggler_app;
mod impl_jiggler_app;

// Re-export JigglerApp for easier access
pub use impl_jiggler_app::JigglerApp;