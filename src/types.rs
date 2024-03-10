use eframe::egui::Color32;
use std::fmt::Debug;
use strum::EnumIter;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    None,
    Error,
    Progress,
    Done,
}

#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
pub enum Browser {
    None,
    Safari,
    Chrome,
    Edge,
    Firefox,
    Brave,
    Opera,
    Vivaldi,
    Chromium,
}

pub trait Colorable {
    fn as_color(&self) -> Color32;
}

pub trait Textable {
    fn as_text(&self) -> &'static str;
}

pub trait Capitalizable {
    fn as_capitalized(&self) -> &'static str;
}

pub trait Lowerable {
    fn as_lower(&self) -> &'static str;
}

pub trait GetterSetter<T> {
    fn new(value: T) -> Self;
    fn get(&mut self) -> T;
    fn set(&mut self, value: T);
}

impl Colorable for Status {
    fn as_color(&self) -> Color32 {
        match self {
            Status::None => Color32::LIGHT_BLUE,
            Status::Error => Color32::LIGHT_RED,
            Status::Progress => Color32::LIGHT_YELLOW,
            Status::Done => Color32::LIGHT_GREEN,
        }
    }
}

impl Textable for Status {
    fn as_text(&self) -> &'static str {
        match self {
            Status::None => "DOWNLOAD",
            Status::Error => "ERROR",
            Status::Progress => "DOWNLOADING...",
            Status::Done => "DONE",
        }
    }
}

impl Capitalizable for Browser {
    fn as_capitalized(&self) -> &'static str {
        match self {
            Browser::None => "None",
            Browser::Safari => "Safari",
            Browser::Chrome => "Chrome",
            Browser::Edge => "Edge",
            Browser::Firefox => "Firefox",
            Browser::Brave => "Brave",
            Browser::Opera => "Opera",
            Browser::Vivaldi => "Vivaldi",
            Browser::Chromium => "Chromium",
        }
    }
}

impl Lowerable for Browser {
    fn as_lower(&self) -> &'static str {
        match self {
            Browser::None => "none",
            Browser::Safari => "safari",
            Browser::Chrome => "chrome",
            Browser::Edge => "edge",
            Browser::Firefox => "firefox",
            Browser::Brave => "brave",
            Browser::Opera => "opera",
            Browser::Vivaldi => "vivaldi",
            Browser::Chromium => "chromium",
        }
    }
}
