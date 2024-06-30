use crate::{
    renderer::Renderer,
    texture::{Texture, TextureType},
    uniform::{FragmentUniform, VertexUniform},
};

use eframe::{epaint, wgpu};
use egui::{load::SizedTexture, TextureId, Vec2};
use image::GenericImageView;
use std::env;

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
    frag_uniform: FragmentUniform,
    vert_uniform: VertexUniform,

    /// How much to rotate the image, in degrees
    rotation_angle: i32,

    /// How much to zoom in / out
    zoom_factor: f32,
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
            frag_uniform: FragmentUniform::default(),
            vert_uniform: VertexUniform::default(),
            rotation_angle: 0,
            zoom_factor: 1.0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Apply filters to the current image
        if let Some(output_texture) = self.output_texture.as_ref() {
            let wgpu = frame.wgpu_render_state().unwrap();
            self.renderer
                .prepare(&wgpu.queue, self.frag_uniform, self.vert_uniform);
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

        egui::SidePanel::right("right_panel")
            .exact_width(180.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("contrast");
                    ui.add(
                        egui::Slider::new(&mut self.frag_uniform.contrast, 0.9..=1.1)
                            .trailing_fill(true),
                    );

                    ui.label("brightness");
                    ui.add(
                        egui::Slider::new(&mut self.frag_uniform.brightness, -0.25..=0.25)
                            .trailing_fill(true),
                    );

                    ui.label("saturation");
                    ui.add(
                        egui::Slider::new(&mut self.frag_uniform.saturation, 0.0..=2.0)
                            .trailing_fill(true),
                    );
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(output_texture) = self.output_texture.as_ref().as_mut() {
                let id = self.output_texture_id.get_or_insert_with(|| {
                    let wgpu = frame.wgpu_render_state().unwrap();
                    let mut renderer = wgpu.renderer.write();
                    renderer.register_native_texture(
                        &wgpu.device,
                        &output_texture.view,
                        wgpu::FilterMode::Linear,
                    )
                });

                egui::TopBottomPanel::top("image_controls").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("↺").clicked() {
                            self.rotation_angle += 90;
                        }
                        if ui.button("↻").clicked() {
                            self.rotation_angle -= 90;
                        }

                        if ui.button("-").clicked() {
                            self.zoom_factor -= 0.125;
                        }
                        if ui.button("+").clicked() {
                            self.zoom_factor += 0.125;
                        }
                    });
                });

                let (mut width, mut height) = output_texture.size;
                if self.rotation_angle % 180 == 90 {
                    (width, height) = (height, width);
                }

                egui::TopBottomPanel::bottom("image_info").show_inside(ui, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(format!("{} x {} px", width, height));
                    });
                });

                ui.centered_and_justified(|ui| {
                    let size = Vec2::new(width as f32, height as f32);
                    let img = egui::Image::new((id.to_owned(), size))
                        .maintain_aspect_ratio(true)
                        .shrink_to_fit();

                    ui.add(img);
                });
            }
        });
    }
}
