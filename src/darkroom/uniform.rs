use cgmath::{Deg, Matrix4, SquareMatrix};
use serde::{Deserialize, Serialize};

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FragmentUniform {
    pub contrast: f32,
    pub saturation: f32,
    pub brightness: f32,
    // GLSL doesn't support bools in uniforms so we'll have to trick it
    pub invert: u32,
    pub temperature: f32,
}

impl Default for FragmentUniform {
    fn default() -> Self {
        Self {
            contrast: 0.0,
            saturation: 1.0,
            brightness: 0.0,
            invert: 0,
            temperature: 5500.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    pub fn rotate(angle: i32) -> Matrix4<f32> {
        Matrix4::from_angle_z(Deg(angle as f32))
    }

    pub fn scale(factor: f32) -> Matrix4<f32> {
        Matrix4::from_scale(factor)
    }
}
