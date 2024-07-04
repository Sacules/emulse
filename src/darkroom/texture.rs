use image::{DynamicImage, GenericImageView};
use miniquad::{self as mq, TextureKind, TextureParams};

/// A representation of an image in the GPU
pub struct Texture {
    /// The ID of the texture
    pub id: mq::TextureId,

    /// The dimensions
    pub size: (u32, u32),
}

impl Texture {
    /// Creates a new Texture with some data on it
    pub fn input(mq_ctx: &mut mq::Context, data: DynamicImage) -> Self {
        let (width, height) = data.dimensions();
        let data = data.to_rgba8();

        let id = mq_ctx.new_texture_from_rgba8(width as u16, height as u16, &data);

        Self {
            id,
            size: (width, height),
        }
    }

    pub fn output(mq_ctx: &mut mq::Context, size: (u32, u32)) -> Self {
        let (width, height) = size;
        let id = mq_ctx.new_render_texture(TextureParams {
            width,
            height,
            format: mq::TextureFormat::RGBA8,
            ..Default::default()
        });

        Self { id, size }
    }
}
