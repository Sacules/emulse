use crate::renderer::Renderer;

use egui::{load::SizedTexture, ColorImage, Image, TextureHandle};
use image::DynamicImage;

pub struct App {
    renderer: Option<Renderer>,
    current_image: Option<DynamicImage>,
    current_texture: Option<TextureHandle>,
    contrast: i32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let img = image::open("test/film3.tif").unwrap();

        Self {
            //renderer: Renderer::new(cc.wgpu_render_state.as_ref().unwrap()),
            renderer: None,
            current_image: Some(img.clone()),
            current_texture: None,
            contrast: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            let img = self.current_image.as_mut().unwrap();

            let tex = self.current_texture.get_or_insert_with(|| {
                // load only once
                ctx.load_texture(
                    "test",
                    ColorImage::from_rgba_unmultiplied(
                        [img.width() as usize, img.height() as usize],
                        img.clone().into_rgba8().as_flat_samples().as_slice(),
                    ),
                    Default::default(),
                )
            });

            ui.add(Image::from_texture(SizedTexture::from_handle(tex)));

            ui.add(egui::Slider::new(&mut self.contrast, -20..=20));
        });
    }
}
