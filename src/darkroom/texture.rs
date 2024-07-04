use image::{DynamicImage, GenericImageView};
use miniquad::{self as mq, TextureParams};

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
        let id = mq_ctx.new_texture(
            mq::TextureAccess::Static,
            mq::TextureSource::Bytes(data.as_bytes()),
            TextureParams {
                width,
                height,
                format: mq::TextureFormat::RGBA8,
                ..Default::default()
            },
        );

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
