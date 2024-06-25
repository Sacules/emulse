use cgmath::Vector2;
use eframe::{
    egui_wgpu,
    wgpu::{self, include_wgsl, util::DeviceExt},
};

use crate::texture::Texture;
use crate::uniform::Uniform;
use crate::vertex::Vertex;

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: Option<wgpu::BindGroup>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub fn new(wgpu: &egui_wgpu::RenderState) -> Self {
        let layout = wgpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Image renderer pipeline layout"),
                bind_group_layouts: &[
                    &Texture::get_bind_group_layout(&wgpu.device),
                    &Uniform::get_bind_group_layout(&wgpu.device),
                ],
                push_constant_ranges: &[],
            });

        let shaders = wgpu
            .device
            .create_shader_module(include_wgsl!("shaders.wgsl"));

        let pipeline = wgpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Image render pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shaders,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shaders,
                    entry_point: "fs_main",
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

        let uniform = Uniform::default();
        let uniform_buffer = wgpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex uniform buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let uniform_bind_group_layout = Uniform::get_bind_group_layout(&wgpu.device);
        let uniform_bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Fragment uniform bind group"),
        });

        Self {
            pipeline,
            texture_bind_group: None,
            uniform_buffer,
            uniform_bind_group,
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
                        resource: wgpu::BindingResource::TextureView(&input_texture.view),
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

        let vertex_buffer = get_vertex_buffer(&wgpu.device, -1.0, -1.0, 1.0, 1.0);

        let mut encoder = wgpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Image renderer encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Image renderer render pass descriptor"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output_texture.view,
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
            pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.draw(0..6, 0..1);
        }

        let command_buffer = encoder.finish();
        wgpu.queue.submit([command_buffer]);
    }
}

fn get_vertex_buffer(
    device: &wgpu::Device,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
) -> wgpu::Buffer {
    let texture_cords = (
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
    );
    let shape = [
        // Top triangle
        Vertex::new(start_x, end_y, texture_cords.0.x, texture_cords.0.y),
        Vertex::new(start_x, start_y, texture_cords.1.x, texture_cords.1.y),
        Vertex::new(end_x, start_y, texture_cords.3.x, texture_cords.3.y),
        // Bottom triangle
        Vertex::new(start_x, end_y, texture_cords.0.x, texture_cords.0.y),
        Vertex::new(end_x, end_y, texture_cords.2.x, texture_cords.2.y),
        Vertex::new(end_x, start_y, texture_cords.3.x, texture_cords.3.y),
    ];

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Image Vertex Buffer"),
        contents: bytemuck::cast_slice(shape.as_slice()),
        usage: wgpu::BufferUsages::VERTEX,
    })
}
