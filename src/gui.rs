pub extern crate eframe;
pub use eframe::egui::{CentralPanel, Context, ViewportBuilder, ViewportCommand, Window};
pub use eframe::{egui, run_native, App, Error, Frame, NativeOptions};

#[derive(Default)]
pub struct GUI {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
}

impl App for GUI {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Try to close the window");
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }

        if self.show_confirmation_dialog {
            Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                });
        }
    }
}

pub fn run() -> Result<(), Error> {
    run_native(
        "YouTube Downloader GUI",
        NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size([640.0, 480.0]),
            ..Default::default()
        },
        Box::new(|_cc| Box::<GUI>::default()),
    )
}
