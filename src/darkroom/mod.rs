pub mod renderer;
pub mod texture;
pub mod uniform;
pub mod vertex;

use crate::darkroom::{
    renderer::Renderer,
    texture::Texture,
    uniform::{FragmentUniform, VertexUniform},
};

use cgmath::{Angle, Rad};
use egui::{TextureId, Vec2};
use image::GenericImageView;
use miniquad as mq;

/// The main object holding the app's state
pub struct Darkroom {
    /// Whether this view is ready for showing
    pub ready: bool,

    /// A handle to the image processing renderer
    pub renderer: Option<Renderer>,

    /// The main texture loaded into the GPU for editing, not shown
    input_texture: Option<Texture>,

    /// The texture that's shown on screen after the render pass
    output_texture: Option<Texture>,

    pub output_texture_id: Option<TextureId>,

    /// A way to parametrize the shaders from the UI
    frag_uniform: FragmentUniform,
    vert_uniform: VertexUniform,

    /// How much to rotate the image, in degrees
    rotation_angle: Rad<f32>,

    /// How much to zoom in / out
    zoom_factor: f32,
}

impl Darkroom {
    pub fn new() -> Self {
        Self {
            ready: false,
            renderer: None,
            input_texture: None,
            output_texture: None,
            output_texture_id: None,
            frag_uniform: FragmentUniform::default(),
            vert_uniform: VertexUniform::default(),
            rotation_angle: Rad(0.0),
            zoom_factor: 1.0,
        }
    }

    pub fn prepare(&mut self, filename: String, mq_ctx: &mut mq::Context) {
        let data = image::open(filename).unwrap();
        let dimensions = data.dimensions();

        let input_texture = Texture::input(mq_ctx, data);
        let output_texture = Texture::output(mq_ctx, dimensions);

        self.renderer = Some(Renderer::new(mq_ctx));
        self.input_texture = Some(input_texture);
        self.output_texture = Some(output_texture);
        self.ready = true;
    }

    pub fn update(&mut self, mq_ctx: &mut mq::Context) {
        // Apply filters to the current image
        self.renderer.as_mut().unwrap().render(
            mq_ctx,
            &self.input_texture.as_ref().unwrap(),
            &self.output_texture.as_ref().unwrap(),
        );
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
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

                    let mut invert = self.frag_uniform.invert != 0;
                    ui.add(egui::Checkbox::new(&mut invert, "Invert"));
                    self.frag_uniform.invert = invert as u32;

                    /*
                    ui.label("white balance");
                    ui.add(
                        egui::Slider::new(&mut self.frag_uniform.temperature, 0.0..=40_000.0)
                            .trailing_fill(true),
                    );
                    */
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (mut width, mut height) = (0, 0);
            if let Some(output_texture) = self.output_texture.as_ref() {
                (width, height) = output_texture.size;
            }

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

            let size = Vec2::new(width as f32, height as f32);

            egui::TopBottomPanel::bottom("image_info").show_inside(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(format!("{} x {} px", size.x, size.y));
                });
            });

            egui::ScrollArea::both().show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    if let Some(id) = self.output_texture_id {
                        let img = egui::Image::new((id.to_owned(), size))
                            .rotate(self.rotation_angle.0, Vec2::splat(0.5))
                            .maintain_aspect_ratio(true)
                            .fit_to_fraction((self.zoom_factor, self.zoom_factor).into());

                        ctx.debug_text(format!("{:?}", id));
                        ui.add(img);
                    }
                });
                ctx.texture_ui(ui);
            });
        });
    }
}
