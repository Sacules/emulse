use cgmath::{Deg, Matrix4, SquareMatrix};
use eframe::wgpu;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FragmentUniform {
    pub contrast: f32,
    pub saturation: f32,
    pub brightness: f32,
}

impl Default for FragmentUniform {
    fn default() -> Self {
        Self {
            contrast: 1.0,
            saturation: 1.0,
            brightness: 0.0,
        }
    }
}

impl FragmentUniform {
    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Image fragment bind group"),
        })
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexUniform {
    pub matrix: [[f32; 4]; 4],
}

impl Default for VertexUniform {
    fn default() -> Self {
        Self {
            matrix: Matrix4::identity().into(),
        }
    }
}

impl VertexUniform {
    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Vertex uniform bind group"),
        })
    }

    pub fn rotate(angle: i32) -> Matrix4<f32> {
        Matrix4::from_angle_z(Deg(angle as f32))
    }
}
