use std::env;

use crate::{
    renderer::Renderer,
    texture::{Texture, TextureType},
    uniform::Uniform,
};

use eframe::wgpu;
use egui::TextureId;
use image::GenericImageView;

/// The main object holding the app's state
pub struct App {
    /// A handle to the image processing renderer
    renderer: Renderer,

    /// The main texture loaded into the GPU for editing, not shown
    input_texture: Option<Texture>,

    /// The texture that's shown on screen after the render pass
    output_texture: Option<Texture>,
    output_texture_id: Option<TextureId>,

    /// A way to parametrize the shaders from the UI
    uniform: Uniform,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let args: Vec<String> = env::args().collect();
        let mut input_texture = None;
        let mut output_texture = None;

        // Always use wgpu, so this never fails
        let wgpu = cc.wgpu_render_state.as_ref().unwrap();

        if args.len() > 1 {
            match image::open(&args[1]) {
                Ok(data) => {
                    input_texture = Some(Texture::from_image(&wgpu.device, &wgpu.queue, &data));
                    output_texture = Some(Texture::new(
                        &wgpu.device,
                        data.dimensions(),
                        TextureType::Output,
                    ));
                }
                Err(_err) => {}
            };
        }

        Self {
            renderer: Renderer::new(wgpu),
            input_texture,
            output_texture,
            output_texture_id: None,
            uniform: Uniform::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Apply filters to the current image
        if let Some(output_texture) = self.output_texture.as_ref() {
            let wgpu = frame.wgpu_render_state().unwrap();
            self.renderer.prepare(&wgpu.queue, self.uniform);
            self.renderer
                .render(wgpu, self.input_texture.as_ref().unwrap(), output_texture);
        }

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
            ui.label("contrast");
            ui.add(egui::Slider::new(&mut self.uniform.contrast, 0.9..=1.1));

            ui.label("brightness");
            ui.add(egui::Slider::new(&mut self.uniform.brightness, -0.5..=0.5));

            ui.label("saturation");
            ui.add(egui::Slider::new(&mut self.uniform.saturation, 0.0..=2.0));

            if let Some(output_texture) = self.output_texture.as_ref() {
                let id = self.output_texture_id.get_or_insert_with(|| {
                    let wgpu = frame.wgpu_render_state().unwrap();
                    let mut renderer = wgpu.renderer.write();
                    renderer.register_native_texture(
                        &wgpu.device,
                        &output_texture.view,
                        wgpu::FilterMode::Linear,
                    )
                });

                ui.image((
                    id.to_owned(),
                    (output_texture.size.0 as f32, output_texture.size.1 as f32).into(),
                ));
            }
        });
    }
}
