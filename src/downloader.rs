pub use crate::gui::Browser;
pub use std::{fs::remove_file, path::PathBuf};
pub use youtube_dl::{download_yt_dlp, model::*, YoutubeDl};

#[tokio::main]
pub async fn run(url: String, browser: &mut Browser, on_result: impl FnOnce(bool) -> ()) {
    let youtube_dl = download_yt_dlp(".").await;
    match youtube_dl {
        Ok(mut path) => download(url, browser, &mut path, on_result),
        Err(_) => on_result(true),
    };
}

pub fn download(
    url: String,
    browser: &mut Browser,
    path: &mut PathBuf,
    on_result: impl FnOnce(bool) -> (),
) {
    let result = YoutubeDl::new(url)
        .extra_arg(format!(
            "--cookies-from-browser={:}",
            match browser {
                Browser::Safari => "safari",
                Browser::Chrome => "chrome",
                Browser::Edge => "edge",
                Browser::Firefox => "firefox",
                Browser::Brave => "brave",
                Browser::Opera => "opera",
                Browser::Vivaldi => "vivaldi",
                Browser::Chromium => "chromium",
            }
        ))
        .output_template("%(title)s.%(ext)s")
        .youtube_dl_path(path.clone())
        .format("best[ext=mp4]")
        .download_to(".");

    match result {
        Ok(_) => on_result(false),
        Err(_) => on_result(true),
    }

    clean(path);
}

pub fn clean(path: &mut PathBuf) {
    let result = remove_file(path);

    match result {
        Ok(_) => (),
        Err(_) => (),
    }
}
