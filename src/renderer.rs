use eframe::{
    egui_wgpu,
    wgpu::{self, include_wgsl},
};

use crate::texture::Texture;
use crate::vertex::Vertex;

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: Option<wgpu::BindGroup>,
}

impl Renderer {
    pub fn new(wgpu: &egui_wgpu::RenderState) -> Self {
        let layout = wgpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Image renderer pipeline layout"),
                bind_group_layouts: &[&Texture::get_bind_group_layout(&wgpu.device)],
                push_constant_ranges: &[],
            });

        let vertex = wgpu
            .device
            .create_shader_module(include_wgsl!("shader/image_vert.wgsl"));

        let fragment = wgpu
            .device
            .create_shader_module(include_wgsl!("shader/image_frag.wgsl"));

        let pipeline = wgpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Image render pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex,
                    entry_point: "main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fragment,
                    entry_point: "main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        Self {
            pipeline,
            texture_bind_group: None,
        }
    }

    pub fn render(
        &mut self,
        wgpu: &egui_wgpu::RenderState,
        input_texture: &Texture,
        output_texture: &Texture,
    ) {
        let texture_bind_group = self.texture_bind_group.get_or_insert_with(|| {
            wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Texture bind group"),
                layout: &Texture::get_bind_group_layout(&wgpu.device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            input_texture.view.as_ref().unwrap(),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            input_texture.sampler.as_ref().unwrap(),
                        ),
                    },
                ],
            })
        });

        let mut encoder = wgpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Image renderer encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Image renderer render pass descriptor"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: output_texture.view.as_ref().unwrap(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                timestamp_writes: None,
                depth_stencil_attachment: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, texture_bind_group, &[]);
        }

        let command_buffer = encoder.finish();
        wgpu.queue.submit([command_buffer]);
    }
}
