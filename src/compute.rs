use eframe::{
    egui_wgpu,
    wgpu::{self, include_wgsl},
};
use std::{mem, sync::mpsc::channel};

use crate::texture::Texture;

const HISTOGRAM_NUM_BINS: u64 = 256;

pub struct Compute {
    pipeline: wgpu::ComputePipeline,
}

fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Compute bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    })
}

impl Compute {
    pub fn new(wgpu: &egui_wgpu::RenderState) -> Self {
        let layout = wgpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute pipeline layout"),
                bind_group_layouts: &[&bind_group_layout(&wgpu.device)],
                push_constant_ranges: &[],
            });

        let shaders = wgpu
            .device
            .create_shader_module(include_wgsl!("compute.wgsl"));

        let pipeline = wgpu
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Compute pipeline"),
                layout: Some(&layout),
                module: &shaders,
                entry_point: "cs_main",
            });

        Self { pipeline }
    }

    pub async fn histogram(&self, wgpu: &egui_wgpu::RenderState, texture: &Texture) {
        let histogram_buffer = wgpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Compute histogram buffer"),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
            size: HISTOGRAM_NUM_BINS * mem::size_of::<u32>() as u64,
            mapped_at_creation: false,
        });

        let result_buffer = wgpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Compute result buffer"),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            size: histogram_buffer.size(),
            mapped_at_creation: false,
        });

        let compute_bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute bind group"),
            layout: &bind_group_layout(&wgpu.device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        histogram_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
            ],
        });

        let mut encoder = wgpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Compute command encoder"),
            });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute pass descriptor"),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &compute_bind_group, &[]);
            pass.dispatch_workgroups(texture.size.0, texture.size.1, 1);
        }

        encoder.copy_buffer_to_buffer(
            &histogram_buffer,
            0,
            &result_buffer,
            0,
            result_buffer.size(),
        );

        let command_buffer = encoder.finish();
        wgpu.queue.submit([command_buffer]);

        let (sender, receiver) = channel();
        result_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                match sender.send(result) {
                    Ok(()) => {}
                    Err(err) => println!("couldn't send to receiver: {}", err),
                }
            });

        wgpu.device.poll(wgpu::MaintainBase::Wait);
        receiver
            .recv()
            .expect("communication failed")
            .expect("buffer readind failed");
    }
}
