use egui::{load::SizedTexture, ColorImage, Image, TextureHandle};

pub struct TemplateApp {
    current_texture: Option<TextureHandle>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            current_texture: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.heading("Emulse");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.heading("Library");
                    ui.heading("Process");
                });
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                let _ = ui.button("Import");
                ui.label("Library");
                ui.label("Tags");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let img = image::open("test/film2.tif").unwrap();

            let tex = self.current_texture.get_or_insert_with(|| {
                // load only once
                ctx.load_texture(
                    "test",
                    ColorImage::from_rgba_unmultiplied(
                        [img.width() as usize, img.height() as usize],
                        img.into_rgba8().as_flat_samples().as_slice(),
                    ),
                    Default::default(),
                )
            });

            ui.add(Image::from_texture(SizedTexture::from_handle(tex)));
        });
    }
}
