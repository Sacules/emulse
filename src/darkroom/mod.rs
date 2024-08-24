#![allow(clippy::new_without_default)]

pub mod renderer;
pub mod texture;
pub mod uniform;
pub mod vertex;

use crate::darkroom::{renderer::Renderer, uniform::FragmentUniform};

use cgmath::{Angle, Rad};
use egui::Vec2;
use miniquad as mq;

pub struct Darkroom {
    /// A handle to the image processing renderer
    renderer: Renderer,

    /// A way to parametrize the shaders from the UI
    frag_uniform: FragmentUniform,

    /// The size of the image
    input_texture_dimensions: (f32, f32),

    /// The texture that's shown on screen after the render pass
    output_texture_id: egui::TextureId,

    /// How much to rotate the image, in degrees
    rotation_angle: Rad<f32>,

    /// How much to zoom in / out
    zoom_factor: f32,
}

impl Darkroom {
    pub fn new(mq_ctx: &mut mq::Context, texture_handle: egui::TextureHandle) -> Self {
        let dimensions = texture_handle.size();
        let id = texture_handle.id();

        Self {
            renderer: Renderer::new(mq_ctx, egui_to_mq_texture_id(id), dimensions),
            frag_uniform: FragmentUniform::default(),
            input_texture_dimensions: (dimensions[0] as f32, dimensions[1] as f32),
            output_texture_id: id,
            rotation_angle: Rad(0.0),
            zoom_factor: 1.0,
        }
    }

    pub fn update(&mut self, mq_ctx: &mut mq::Context) {
        // Apply filters to the current image
        self.output_texture_id = self.renderer.render(mq_ctx, self.frag_uniform);
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right_panel")
            .exact_width(180.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("contrast");
                    ui.add(
                        egui::Slider::new(&mut self.frag_uniform.contrast, -30.0..=30.0)
                            .step_by(1.0)
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

                    let mut invert = self.frag_uniform.invert != 0;
                    ui.add(egui::Checkbox::new(&mut invert, "Invert"));
                    self.frag_uniform.invert = invert as u32;
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("image_controls").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("↺").clicked() {
                        self.rotation_angle -= Rad::turn_div_4();
                    }
                    if ui.button("↻").clicked() {
                        self.rotation_angle += Rad::turn_div_4();
                    }

                    if ui.button("-").clicked() {
                        self.zoom_factor -= 0.125;
                    }
                    if ui.button("+").clicked() {
                        self.zoom_factor += 0.125;
                    }
                });
            });

            egui::TopBottomPanel::bottom("image_info").show_inside(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(format!(
                        "{} x {} px",
                        self.input_texture_dimensions.0, self.input_texture_dimensions.1
                    ));
                });
            });

            egui::ScrollArea::both().show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    let img = egui::Image::new((
                        self.output_texture_id.to_owned(),
                        self.input_texture_dimensions.into(),
                    ))
                    .rotate(self.rotation_angle.0, Vec2::splat(0.5))
                    .maintain_aspect_ratio(true)
                    .fit_to_fraction((self.zoom_factor, self.zoom_factor).into());

                    ui.add(img);
                });
            });
        });
    }
}

fn egui_to_mq_texture_id(from: egui::TextureId) -> mq::TextureId {
    match from {
        egui::TextureId::Managed(id) => {
            // For some reason OpenGL expects u32 for the texture IDs,
            // but egui uses u64 instead. Also, it's off by 2???
            let raw_id = mq::RawId::OpenGl((id + 2).try_into().expect("couldn't cast"));
            mq::TextureId::from_raw_id(raw_id)
        }
        _ => mq::TextureId::from_raw_id(mq::RawId::OpenGl(1)),
    }
}
