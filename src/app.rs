use std::env;

use crate::{renderer::Renderer, texture::Texture};

use eframe::wgpu;
use egui::TextureId;

/// The main object holding the app's state
pub struct App {
    /// A handle to the image processing renderer
    renderer: Renderer,

    /// The main texture loaded into the GPU for editing, not shown
    input_texture: Option<Texture>,
    current_texture_id: Option<TextureId>,

    /// The texture that's shown on screen after the render pass
    output_texture: Option<Texture>,
    output_texture_id: Option<TextureId>,

    /// Some image controls
    contrast: i32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let args: Vec<String> = env::args().collect();
        let mut tex = None;

        // Always use wgpu, so this never fails
        let wgpu = cc.wgpu_render_state.as_ref().unwrap();

        if args.len() > 1 {
            tex = match image::open(&args[1]) {
                Ok(data) => Some(Texture::from_image(&wgpu.device, &wgpu.queue, &data)),
                Err(_err) => None,
            };
        }

        Self {
            renderer: Renderer::new(wgpu),
            input_texture: tex,
            current_texture_id: None,
            output_texture: None,
            output_texture_id: None,
            contrast: 0,
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

            if let Some(output_texture) = self.output_texture.as_ref() {
                let id = self.current_texture_id.get_or_insert_with(|| {
                    let wgpu = frame.wgpu_render_state().unwrap();
                    let mut renderer = wgpu.renderer.write();
                    renderer.register_native_texture(
                        &wgpu.device,
                        output_texture.view.as_ref().unwrap(),
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
