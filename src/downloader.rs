use crate::{threads::*, types::*};
use std::path::PathBuf;
use youtube_dl::{download_yt_dlp, Error, YoutubeDl};

#[tokio::main]
pub async fn run(
    url: &mut String,
    browser: &mut Browser,
    folder: &mut PathBuf,
    status: &mut Async<Status>,
) {
    let result = download(url, browser, folder).await;
    status.set(match result {
        Ok(_) => Status::Done,
        Err(_) => Status::Error,
    });
}

pub async fn download(
    url: &mut String,
    browser: &mut Browser,
    folder: &mut PathBuf,
) -> Result<(), Error> {
    let youtube_dl = download_yt_dlp(".").await;

    YoutubeDl::new(url.to_owned())
        .output_template("%(title)s.%(ext)s")
        .youtube_dl_path(youtube_dl.unwrap())
        .extra_arg(cookies_from_browser(browser.to_owned()))
        .format("best[ext=mp4]")
        .download_to_async(folder)
        .await
}

pub fn cookies_from_browser(browser: Browser) -> String {
    match browser {
        Browser::None => String::from("--no-cookies-from-browser"),
        _ => format!("--cookies-from-browser={:}", browser.as_lower()),
    }
}
