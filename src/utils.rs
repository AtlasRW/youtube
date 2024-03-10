use regex::Regex;
use rfd::FileDialog;
use std::{fs::remove_file, path::PathBuf};

pub fn pick_folder() -> PathBuf {
    let result = FileDialog::new().pick_folder();

    match result {
        Some(path) => path,
        _ => PathBuf::from("."),
    }
}

pub fn is_valid_url(url: &mut String) -> bool {
    Regex::new(r"^(http(s?)\:\/\/|~/|/)?([a-zA-Z]{1}([\w\-]+\.)+([\w]{2,5}))(:[\d]{1,5})?/?(\w+\.[\w]{3,4})?((\?\w+=\w+)?(&\w+=\w+)*)?")
    .unwrap()
    .is_match(url.as_str())
}

pub fn clean(path: PathBuf) {
    let _ = remove_file(path);
}

pub fn folder_to_string(pathbuf: &mut PathBuf) -> String {
    let path = pathbuf.to_str().unwrap();
    String::from(if path == "." { "Select a folder" } else { path })
}
