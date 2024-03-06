pub use crate::downloader;
pub use crate::icon;
pub use eframe::egui::{
    CentralPanel, Color32, Context, CursorIcon, IconData, OpenUrl, RichText, Ui, ViewportBuilder,
    ViewportCommand, Window,
};
pub use eframe::{egui, run_native, App, Error, Frame, NativeOptions};

// #[derive(PartialEq)]
// pub enum Format {
//     Video,
//     Playlist,
// }

#[derive(PartialEq)]
pub enum Status {
    None,
    Error,
    Progress,
    Done,
}

#[derive(PartialEq)]
pub enum Browser {
    // None,
    Safari,
    Chrome,
    Edge,
    Firefox,
    Brave,
    Opera,
    Vivaldi,
    Chromium,
}

pub struct APP {
    url: String,
    // format: Format,
    browser: Browser,
    status: Status,
    // is_playlist: bool,
}

impl Default for APP {
    fn default() -> Self {
        Self {
            url: String::from(
                "https://www.youtube.com/playlist?list=PLwpsV7kne6SmTia9UvZUhPV2IPDKFG9iN",
            ),
            // is_playlist: false,
            // format: Format::Video,
            browser: Browser::Chrome,
            status: Status::None,
        }
    }
}

impl App for APP {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("LINK").strong().underline())
                        .on_hover_cursor(CursorIcon::Help)
                        .on_hover_text_at_pointer("Copy link of target YouTube video or playlist");
                    ui.text_edit_singleline(&mut self.url);
                });

                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("BROWSER").strong().underline())
                        .on_hover_cursor(CursorIcon::Help)
                        .on_hover_text_at_pointer(
                            "Select a browser on which you are connected to YouTube",
                        );
                    // ui.selectable_value(&mut self.browser, Browser::None, "None")
                    //     .on_hover_cursor(CursorIcon::PointingHand);
                    ui.selectable_value(&mut self.browser, Browser::Safari, "Safari")
                        .on_hover_cursor(CursorIcon::PointingHand);
                    ui.selectable_value(&mut self.browser, Browser::Chrome, "Chrome")
                        .on_hover_cursor(CursorIcon::PointingHand);
                    ui.selectable_value(&mut self.browser, Browser::Edge, "Edge")
                        .on_hover_cursor(CursorIcon::PointingHand);
                    ui.selectable_value(&mut self.browser, Browser::Firefox, "Firefox")
                        .on_hover_cursor(CursorIcon::PointingHand);
                    ui.selectable_value(&mut self.browser, Browser::Brave, "Brave")
                        .on_hover_cursor(CursorIcon::PointingHand);
                    ui.selectable_value(&mut self.browser, Browser::Opera, "Opera")
                        .on_hover_cursor(CursorIcon::PointingHand);
                    ui.selectable_value(&mut self.browser, Browser::Vivaldi, "Vivaldi")
                        .on_hover_cursor(CursorIcon::PointingHand);
                    ui.selectable_value(&mut self.browser, Browser::Chromium, "Chromium")
                        .on_hover_cursor(CursorIcon::PointingHand);
                });

                // ui.separator();

                // ui.horizontal_wrapped(|ui| {
                //     ui.label("Format");
                //     ui.selectable_value(&mut self.format, Format::Video, "Single Video");
                //     ui.selectable_value(&mut self.format, Format::Playlist, "Playlist");
                // });

                // ui.separator();
            });

            // ui.separator();

            // if ui.link(self.url.clone()).clicked() {
            //     ui.ctx().open_url(OpenUrl {
            //         url: self.url.clone(),
            //         new_tab: true,
            //     })
            // }

            ui.separator();

            ui.centered_and_justified(|ui| {
                if ui
                    .button(
                        RichText::new(match self.status {
                            Status::None => "DOWNLOAD",
                            Status::Error => "ERROR!",
                            Status::Progress => "DOWNLOADING...",
                            Status::Done => "DONE!",
                        })
                        .strong()
                        .color(match self.status {
                            Status::None => Color32::LIGHT_BLUE,
                            Status::Error => Color32::LIGHT_RED,
                            Status::Progress => Color32::LIGHT_YELLOW,
                            Status::Done => Color32::LIGHT_GREEN,
                        }),
                    )
                    .on_hover_cursor(CursorIcon::PointingHand)
                    // .on_hover_text_at_pointer("Download all displayed videos in selected formats")
                    .clicked()
                {
                    self.status = Status::Progress;
                    ui.ctx().set_cursor_icon(CursorIcon::Wait);
                    downloader::run(self.url.clone(), &mut self.browser, |error: bool| {
                        ui.ctx().set_cursor_icon(CursorIcon::Default);
                        self.status = if error { Status::Error } else { Status::Done }
                    });
                }
            });
        });
    }
}

pub fn options() -> NativeOptions {
    NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([335.0, 150.0])
            .with_min_inner_size([335.0, 150.0])
            .with_max_inner_size([335.0, 150.0])
            .with_icon(IconData {
                rgba: icon::get(),
                height: 128,
                width: 128,
            }),
        ..Default::default()
    }
}
