use cgmath::{Deg, Matrix4, SquareMatrix};
use eframe::wgpu;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FragmentUniform {
    pub contrast: f32,
    pub saturation: f32,
    pub brightness: f32,
    // wgsl doesn't support bools in uniforms so we'll have to trick it
    pub invert: u32,
    pub temperature: f32,
}

impl Default for FragmentUniform {
    fn default() -> Self {
        Self {
            contrast: 1.0,
            saturation: 1.0,
            brightness: 0.0,
            invert: 0,
            temperature: 5500.0,
        }
    }
}

impl FragmentUniform {
    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
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

    pub fn scale(factor: f32) -> Matrix4<f32> {
        Matrix4::from_scale(factor)
    }
}
