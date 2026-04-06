use egui::Context;
use egui::FontId;
use egui::Image;
use egui::RichText;

use crate::Apod;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ApodWindow {
    pub apod_window_visible: bool,
    pub apod_full_window_visible: bool,
    pub apod_data: Option<Apod>,
}

impl Default for ApodWindow {
    fn default() -> Self {
        Self {
            apod_window_visible: false,
            apod_full_window_visible: false,
            apod_data: None,
        }
    }
}

impl ApodWindow {
    // Make the paramter self a &mut self to avoid errors with ui
    pub fn create_apod_window(&mut self, ctx: &Context) {
        let mut apod_window_visible = self.apod_window_visible; // Set a local variable to I don't have to use self with .open() below
        egui::Window::new("APOD (Astronomy Pic Of the Day)")
            .max_height(1000.0)
            .open(&mut apod_window_visible) // This doesn't need to be &mut self.apod_ui_apod_window_visible
            .show(ctx, |ui| {
                // APOD Window //
                egui::Frame::default().show(ui, |ui| {
                    match &self.apod_data {
                        Some(data) => {
                            ui.heading(
                                RichText::new(data.title.clone()).font(FontId::monospace(20.0)),
                            );
                            if ui
                                .add(egui::Button::image(
                                    egui::Image::from_uri(data.url.clone())
                                        .fit_to_original_size(1.0),
                                ))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.apod_full_window_visible = true;
                                //self.show_apod_full(true);
                            }
                            ui.label(format!(
                                "Copyright: {}",
                                data.copyright.clone().replace("\n", "")
                            ));
                            ui.heading(RichText::new("Description:").font(FontId::monospace(30.0)));
                            ui.separator();
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.label(
                                    RichText::new(data.explanation.clone())
                                        .font(FontId::monospace(17.0)),
                                );
                            });

                            if self.apod_full_window_visible {
                                self.apod_full_window(
                                    &egui::Image::from_uri(data.hdurl.clone()),
                                    &data.title.clone(),
                                    &data.copyright.clone(),
                                    &ctx,
                                );
                            }
                        }
                        None => match Apod::get_apod_data_blocking() {
                            Ok(apod) => self.apod_data = Some(apod),
                            Err(_e) => {
                                ui.label("Network Error");
                            }
                        },
                    }
                });
            }); // APOD //
        self.apod_window_visible = apod_window_visible;
    }

    pub fn apod_full_window(
        &mut self,
        img: &Image,
        image_name: &String,
        image_credit: &String,
        ctx: &egui::Context,
    ) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("apod_viewport"),
            egui::ViewportBuilder::default()
                .with_title(format!(
                    "{} (By {})",
                    &image_name,
                    &image_credit.replace("\n", "")
                ))
                .with_maximized(true),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.image(img.source(ctx));
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.apod_full_window_visible = false;
                }
            },
        );
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct NeowsWindow {
    pub neows_date: String,
    pub neows_invalid_input_window_visible: bool,
    pub neows_window_visible: bool,
}

impl Default for NeowsWindow {
    fn default() -> Self {
        Self {
            neows_date: String::default(),
            neows_invalid_input_window_visible: false,
            neows_window_visible: false,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct NIVLWindow {
    pub query: String,
    pub nivl_window_visible: bool,
}

impl Default for NIVLWindow {
    fn default() -> Self {
        Self {
            query: String::default(),
            nivl_window_visible: false,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AboutWindow {
    pub about_window_visible: bool,
}

impl Default for AboutWindow {
    fn default() -> Self {
        Self {
            about_window_visible: false,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiKeyWindow {
    pub api_key_window_visible: bool,
    pub key: String,
    pub key_set_label: String,
}

impl Default for ApiKeyWindow {
    fn default() -> Self {
        Self {
            api_key_window_visible: false,
            key: String::default(),
            key_set_label: String::default(),
        }
    }
}
