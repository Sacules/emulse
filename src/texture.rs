use eframe::wgpu;
use image::GenericImageView;

/// A representation of an image in the GPU
pub struct Texture {
    /// A handle to the data
    pub tex: wgpu::Texture,

    /// A way to view into said data
    pub view: Option<wgpu::TextureView>,

    /// A way to fetch the data, like a single texel
    pub sampler: Option<wgpu::Sampler>,

    /// The dimensions
    pub size: (u32, u32),

    pub ty: TextureType,
}

pub enum TextureType {
    Input,
    Output,
}

impl Texture {
    pub fn new(device: &wgpu::Device, dimensions: (u32, u32), ty: TextureType) -> Self {
        let (width, height) = dimensions;
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let usage = match ty {
            TextureType::Input => {
                wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
            }
            TextureType::Output => {
                wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT
            }
        };

        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("image"),
            size,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb, // TODO: dynamically infer this
            usage,
            view_formats: &[],
            mip_level_count: 1,
        });

        Self {
            tex,
            view: None,
            sampler: None,
            ty,
            size: dimensions,
        }
    }

    /// Creates a texture from a parsed image
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
    ) -> Self {
        let (width, height) = img.dimensions();
        let bytes_per_pixel = std::mem::size_of::<image::Rgba<u8>>();
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let mut texture = Texture::new(device, img.dimensions(), TextureType::Input);

        let bytes = img.to_rgba8();

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture.tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &bytes,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some((width as usize * bytes_per_pixel) as u32),
                rows_per_image: Some(height),
            },
            size,
        );

        texture.view = Some(
            texture
                .tex
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture sampler descriptor"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,

            // Helps make a smoother, more natural zoom in/out
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,

            ..Default::default()
        });
        texture.sampler = Some(sampler);
        texture.ty = TextureType::Input;

        texture
    }

    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }
}
