use crate::renderer::Renderer;

use egui::load::SizedTexture;
use image::DynamicImage;

pub struct App {
    renderer: Renderer,
    current_image: Option<DynamicImage>,
    current_texture: Option<egui::TextureHandle>,
    frame_num: i32,
    contrast: i32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let img = image::open("test/film3.tif").unwrap();

        let wgpu = cc.wgpu_render_state.as_ref().unwrap();

        Self {
            renderer: Renderer::new(wgpu),
            current_image: Some(img.clone()),
            current_texture: None,
            contrast: 0,
            frame_num: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
            ui.label("whatever lol");
            ui.add(egui::Slider::new(&mut self.contrast, -20..=20));

            if let Some(current_texture) = self.current_texture.as_ref() {
                let img = egui::Image::from_texture(SizedTexture::from_handle(current_texture));
                img.paint_at(
                    ui,
                    egui::Rect {
                        min: (0.0, 0.0).into(),
                        max: (200.0, 200.0).into(),
                    },
                )
            }
        });
    }
}
