use crate::{components, downloader, threads, types::*, utils};
use eframe::{
    egui::{Context, CursorIcon, IconData, ViewportBuilder},
    App, Frame, NativeOptions,
};
use std::path::PathBuf;

pub struct APP {
    url: String,
    folder: PathBuf,
    status: threads::Async<Status>,
    browser: Browser,
    progress: u8,
}

impl Default for APP {
    fn default() -> Self {
        Self {
            url: String::from(
                // "https://www.youtube.com/watch?v=EC-O35VVC8s",
                "https://www.youtube.com/playlist?list=PLwpsV7kne6SmTia9UvZUhPV2IPDKFG9iN",
            ),
            folder: PathBuf::from("/Users/raphael/danae/youtube"),
            status: threads::Async::new(Status::None),
            browser: Browser::Brave,
            progress: 0,
        }
        // Self {
        //     url: String::from("",
        //     ),
        //     folder: PathBuf::from("."),
        //     browser: Browser::None,
        //     status: Status::None,
        //     progress: 0,
        // }
    }
}

impl App for APP {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        components::central_panel(ctx, |ui| {
            components::space(ui, 10);
            components::row(ui, |ui| {
                components::title(ui, "LINK", "Copy link of target video or playlist");
                components::text_edit(ui, self.status.get() != Status::Progress, &mut self.url);
            });
            components::separator(ui, 20);
            components::row(ui, |ui| {
                components::title(
                    ui,
                    "BROWSER",
                    "Select a browser logged in to YouTube (optional)",
                );
                components::select(
                    ui,
                    self.status.get() != Status::Progress,
                    &mut self.browser,
                    |browser| browser.as_capitalized(),
                );
            });
            components::separator(ui, 20);
            components::row(ui, |ui| {
                components::title(ui, "FOLDER", "Select target download folder");
                components::browse_button(
                    ui,
                    "BROWSE",
                    "Select target download folder",
                    self.status.get() != Status::Progress,
                    |_| self.folder = utils::pick_folder(),
                );
                components::text_edit(ui, false, &mut utils::folder_to_string(&mut self.folder));
            });
            components::separator(ui, 20);
            components::download_button(
                ui,
                40,
                self.status.get().as_text(),
                self.status.get().as_color(),
                self.status.get() != Status::Progress
                    && utils::is_valid_url(&mut self.url)
                    && self.folder != PathBuf::from("."),
                |_| {
                    self.status.set(Status::Progress);
                    downloader::run(
                        &mut self.url,
                        &mut self.browser,
                        &mut self.folder,
                        &mut self.status,
                    );
                },
            );
            components::space(ui, 10);
            components::progress_bar(
                ctx,
                ui,
                self.status.get() == Status::Progress,
                &mut self.progress,
                50,
                self.status.get().as_color(),
            );
        });

        components::set_cursor(ctx, self.status.get() == Status::Progress, CursorIcon::Wait);
    }
}

pub fn options() -> NativeOptions {
    NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([335.0, 260.0])
            .with_icon(IconData {
                rgba: components::icon(),
                height: 128,
                width: 128,
            })
            .with_resizable(false),
        ..Default::default()
    }
}
