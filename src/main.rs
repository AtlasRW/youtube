#![windows_subsystem = "windows"]

pub mod app;
pub mod components;
pub mod downloader;
pub mod threads;
pub mod types;
pub mod utils;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    eframe::run_native(
        "YouTube Downloader",
        app::options(),
        Box::new(|_| Box::<app::APP>::default()),
    )
}
