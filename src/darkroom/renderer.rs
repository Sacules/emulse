use miniquad::{self as mq, TextureId};

//use crate::darkroom::uniform::FragmentUniform;
use crate::darkroom::texture::Texture;
use crate::darkroom::vertex::Vertex;

pub struct Renderer {
    pipeline: mq::Pipeline,
    vertex_buffer: mq::BufferId,
    index_buffer: mq::BufferId,
}

impl Renderer {
    pub fn new(mq_ctx: &mut mq::Context) -> Self {
        let vertex_buffer = get_vertex_buffer(mq_ctx);

        #[rustfmt::skip]
        let indices: &[u16] = &[
        	0, 1, 2, 0, 2, 3,
        ];
        let index_buffer = mq_ctx.new_buffer(
            mq::BufferType::IndexBuffer,
            mq::BufferUsage::Immutable,
            mq::BufferSource::slice(indices),
        );

        let shader = mq_ctx
            .new_shader(
                mq::ShaderSource::Glsl {
                    vertex: include_str!("shader_vert.glsl"),
                    fragment: include_str!("shader_frag.glsl"),
                },
                mq::ShaderMeta {
                    images: vec!["tex".to_string()],
                    uniforms: mq::UniformBlockLayout { uniforms: vec![] },
                },
            )
            .unwrap();

        let pipeline = mq_ctx.new_pipeline(
            &[mq::BufferLayout {
                ..Default::default()
            }],
            &[
                mq::VertexAttribute::new("position", mq::VertexFormat::Float2),
                mq::VertexAttribute::new("tex_coords", mq::VertexFormat::Float2),
            ],
            shader,
            mq::PipelineParams {
                depth_write: false,
                primitive_type: mq::PrimitiveType::Triangles,
                cull_face: mq::CullFace::Nothing,
                ..Default::default()
            },
        );

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn render(&mut self, mq_ctx: &mut mq::Context, input_texture_id: mq::TextureId) {
        let bindings = mq::Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images: vec![input_texture_id],
        };

        let pass = mq_ctx.new_render_pass(input_texture_id, None);
        mq_ctx.begin_pass(Some(pass), mq::PassAction::clear_color(0.2, 0.0, 0.0, 1.0));
        mq_ctx.apply_pipeline(&self.pipeline);
        mq_ctx.apply_bindings(&bindings);
        mq_ctx.apply_uniforms(mq::UniformsSource::table(&input_texture_id));
        mq_ctx.draw(0, 6, 1);
        mq_ctx.end_render_pass();

        // create egui TextureId from Miniquad GL texture Id
        /*
        let raw_id = match unsafe { mq_ctx.texture_raw_id(output_texture.id) } {
            mq::RawId::OpenGl(id) => id as u64,
        };

        egui::TextureId::User(raw_id)
         */
    }
}

fn get_vertex_buffer(mq_ctx: &mut mq::Context) -> mq::BufferId {
    // Draw a rectangle
    #[rustfmt::skip]
    let shape = [
        Vertex { position: [ -0.5, -0.5 ],  tex_coords: [0.0, 1.0] },
        Vertex { position: [  0.5, -0.5 ],  tex_coords: [1.0, 1.0] },
        Vertex { position: [  0.5,  0.5 ],  tex_coords: [1.0, 0.0] },
        Vertex { position: [  -0.5,  0.5 ], tex_coords: [0.0, 0.0] },
    ];

    mq_ctx.new_buffer(
        mq::BufferType::VertexBuffer,
        mq::BufferUsage::Immutable,
        mq::BufferSource::slice(shape.as_slice()),
    )
}
