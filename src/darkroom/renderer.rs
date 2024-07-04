use cgmath::Vector2;
use miniquad as mq;

//use crate::darkroom::uniform::FragmentUniform;
use crate::darkroom::texture::Texture;
use crate::darkroom::vertex::Vertex;

pub struct Renderer {
    pipeline: mq::Pipeline,
    bindings: mq::Bindings,
}

impl Renderer {
    pub fn new(mq_ctx: &mut mq::Context) -> Self {
        let vertex_buffer = get_vertex_buffer(mq_ctx, -1.0, -1.0, 1.0, 1.0);

        #[rustfmt::skip]
        let indices: &[u16] = &[
        	0, 0,
         	0, 0
        ];
        let index_buffer = mq_ctx.new_buffer(
            mq::BufferType::IndexBuffer,
            mq::BufferUsage::Immutable,
            mq::BufferSource::slice(indices),
        );

        let bindings = mq::Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let shader = mq_ctx
            .new_shader(
                mq::ShaderSource::Glsl {
                    vertex: include_str!("shader_vert.glsl"),
                    fragment: include_str!("shader_frag.glsl"),
                },
                mq::ShaderMeta {
                    images: vec![],
                    uniforms: mq::UniformBlockLayout {
                        uniforms: vec![mq::UniformDesc::new("mvp", mq::UniformType::Mat4)],
                    },
                },
            )
            .unwrap();

        let pipeline = mq_ctx.new_pipeline(
            &[mq::BufferLayout {
                stride: indices.len() as i32,
                ..Default::default()
            }],
            &[
                mq::VertexAttribute::new("pos", mq::VertexFormat::Float3),
                mq::VertexAttribute::new("color0", mq::VertexFormat::Float4),
            ],
            shader,
            mq::PipelineParams {
                depth_test: mq::Comparison::LessOrEqual,
                depth_write: true,
                ..Default::default()
            },
        );

        Self { pipeline, bindings }
    }

    pub fn render(
        &mut self,
        mq_ctx: &mut mq::Context,
        input_texture: &Texture,
        output_texture: &Texture,
    ) -> egui::TextureId {
        let pass = mq_ctx.new_render_pass(input_texture.id, Some(output_texture.id));

        mq_ctx.begin_pass(Some(pass), mq::PassAction::clear_color(0.0, 0.0, 0.0, 0.0));
        mq_ctx.apply_pipeline(&self.pipeline);
        //mq_ctx.apply_uniforms(mq::UniformsSource::table(data));
        mq_ctx.draw(0, 6, 1);
        mq_ctx.end_render_pass();

        // create egui TextureId from Miniquad GL texture Id
        let raw_id = match unsafe { mq_ctx.texture_raw_id(output_texture.id) } {
            mq::RawId::OpenGl(id) => id as u64,
        };

        egui::TextureId::User(raw_id)
    }
}

fn get_vertex_buffer(
    mq_ctx: &mut mq::Context,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
) -> mq::BufferId {
    // Draw a rectangle
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

    mq_ctx.new_buffer(
        mq::BufferType::VertexBuffer,
        mq::BufferUsage::Immutable,
        mq::BufferSource::slice(shape.as_slice()),
    )
}
