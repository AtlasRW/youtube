use crate::{
    threads::{self, *},
    types::*,
    utils,
};
use std::path::PathBuf;
use youtube_dl::{download_yt_dlp, YoutubeDl};

#[tokio::main]
pub async fn run(
    url: &mut String,
    browser: &mut Browser,
    folder: &mut PathBuf,
    status: &mut Async<Status>,
) {
    let mut _url = url.clone();
    let mut _browser = browser.clone();
    let mut _folder = folder.clone();
    let mut _status = status.clone();

    let youtube_dl = download_yt_dlp(".").await;

    // threads::execute(async move {
    // let result = download(_url, _browser, _folder).await;
    // match result {
    //     Ok(_) => _status.set(Status::Done),
    //     Err(_) => _status.set(Status::Error),
    // }
    // match youtube_dl {
    //     Ok(_) => _status.set(Status::Done),
    //     Err(_) => _status.set(Status::Error),
    // }
    // });
}

pub async fn download(
    url: String,
    browser: Browser,
    folder: PathBuf,
) -> Result<(), youtube_dl::Error> {
    let youtube_dl = download_yt_dlp(".").await;

    YoutubeDl::new(url.to_owned())
        .output_template("%(title)s.%(ext)s")
        .youtube_dl_path(youtube_dl.unwrap())
        .extra_arg(cookies_from_browser(browser))
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
