#![windows_subsystem = "windows"]
pub mod gui;

fn main() -> Result<(), gui::Error> {
    env_logger::init();
    gui::run()
}

// pub use serde_json::json;
// pub use youtube_dl::{model::*, YoutubeDl};

// fn main() {
//     let output = YoutubeDl::new("https://www.youtube.com/watch?v=WuGaY_TclSs")
//         .all_formats(true)
//         .socket_timeout("15")
//         .format("625")
//         .run()
//         .unwrap();

//     let video = output.into_single_video().unwrap();
//     let formats = video.formats.unwrap();
//     for i in formats {
//         let ext = i.ext.as_ref().unwrap();
//         let resolution = i.resolution.as_ref().unwrap();
//         if ext == "mp4" && resolution != "audio only" {
//             println!("Format ID : {:#?}", i.format_id.as_ref().unwrap());
//             println!("Format : {:#?}", i.format.as_ref().unwrap());
//             println!("Resolution : {:#?}", i.resolution.as_ref().unwrap());
//             println!("{:#?}", i);
//         }
//     }
// }
