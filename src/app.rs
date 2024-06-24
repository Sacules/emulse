use crate::{renderer::Renderer, texture::Texture};

use eframe::wgpu::{self};
use image::DynamicImage;

pub struct App {
    renderer: Renderer,
    current_image: Option<DynamicImage>,
    current_texture: Option<Texture>,
    frame_num: i32,
    contrast: i32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let img = image::open("test/film2.tif").unwrap();

        let wgpu = cc.wgpu_render_state.as_ref().unwrap();
        let tex = Texture::from_image(&wgpu.device, &wgpu.queue, &img);

        Self {
            renderer: Renderer::new(wgpu),
            current_image: Some(img.clone()),
            current_texture: Some(tex),
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
                let wgpu = frame.wgpu_render_state().unwrap();
                let mut renderer = wgpu.renderer.write();
                let id = renderer.register_native_texture(
                    &wgpu.device,
                    &current_texture.view,
                    wgpu::FilterMode::Linear,
                );

                ui.image((
                    id,
                    (current_texture.size.0 as f32, current_texture.size.1 as f32).into(),
                ));
            }
        });
    }
}
