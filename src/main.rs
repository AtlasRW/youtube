#![windows_subsystem = "windows"]

pub mod downloader;
pub mod gui;

fn main() -> Result<(), gui::Error> {
    env_logger::init();
    gui::run_native(
        "YouTube Downloader",
        gui::options(),
        Box::new(|_cc| Box::<gui::APP>::default()),
    )
}
