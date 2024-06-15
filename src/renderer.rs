use eframe::{egui_wgpu, wgpu};

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub fn new(wgpu: &egui_wgpu::RenderState) -> Self {
        unimplemented!();
    }
}
