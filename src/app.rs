use crate::{
    renderer::Renderer,
    texture::{Texture, TextureType},
    uniform::{FragmentUniform, VertexUniform},
};

use eframe::wgpu;
use egui::TextureId;
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

                let (width, height) = output_texture.size;

                egui::TopBottomPanel::top("image_controls").show_inside(ui, |ui| {
                    ui.horizontal_centered(|ui| {
                        if ui.button("↺").clicked() {
                            self.output_texture.as_mut().unwrap().size = (height, width);
                            self.rotation_angle += 90;
                            self.vert_uniform.matrix =
                                VertexUniform::rotate(self.rotation_angle).into();
                        }
                        if ui.button("↻").clicked() {
                            self.output_texture.as_mut().unwrap().size = (height, width);
                            self.rotation_angle -= 90;
                            self.vert_uniform.matrix =
                                VertexUniform::rotate(self.rotation_angle).into();
                        }
                    });
                });

                egui::TopBottomPanel::bottom("image_info").show_inside(ui, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(format!("{} x {} px", width, height));
                    });
                });

                let panel_area = ui.available_size();

                let scale_x = panel_area.x / width as f32;
                let scale_y = panel_area.y / height as f32;
                let scale = scale_x.min(scale_y);
                let margin_bottom = 16.0;

                ui.centered_and_justified(|ui| {
                    ui.image((
                        id.to_owned(),
                        (width as f32 * scale, height as f32 * scale - margin_bottom).into(),
                    ));
                });
            }
        });
    }
}
