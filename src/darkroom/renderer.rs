use miniquad as mq;

use crate::darkroom::vertex::Vertex;

use super::uniform::FragmentUniform;

#[derive(Copy, Clone)]
pub struct Renderer {
    pipeline: mq::Pipeline,
    vertex_buffer: mq::BufferId,
    index_buffer: mq::BufferId,
    render_pass: mq::RenderPass,
    input_texture_id: mq::TextureId,
}

impl Renderer {
    pub fn new(
        mq_ctx: &mut mq::Context,
        texture_id: mq::TextureId,
        dimensions: [usize; 2],
    ) -> Self {
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
                    uniforms: mq::UniformBlockLayout {
                        uniforms: vec![mq::UniformDesc::new("contrast", mq::UniformType::Float1)],
                    },
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
                depth_write: true,
                depth_test: mq::Comparison::LessOrEqual,
                ..Default::default()
            },
        );

        let output_texture = mq_ctx.new_render_texture(mq::TextureParams {
            width: dimensions[0] as u32,
            height: dimensions[1] as u32,
            format: mq::TextureFormat::RGBA8,
            ..Default::default()
        });

        let render_pass = mq_ctx.new_render_pass(output_texture, None);

        Self {
            render_pass,
            pipeline,
            vertex_buffer,
            index_buffer,
            input_texture_id: texture_id,
        }
    }

    pub fn render(&self, mq_ctx: &mut mq::Context, uniforms: FragmentUniform) -> egui::TextureId {
        let bindings = mq::Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images: vec![self.input_texture_id],
        };

        mq_ctx.begin_pass(
            Some(self.render_pass),
            mq::PassAction::clear_color(0.2, 0.0, 0.0, 1.0),
        );
        mq_ctx.apply_pipeline(&self.pipeline);
        mq_ctx.apply_bindings(&bindings);
        mq_ctx.apply_uniforms(mq::UniformsSource::table(&uniforms));
        mq_ctx.draw(0, 6, 1);
        mq_ctx.end_render_pass();

        // Retrieve output texture
        let attachments = mq_ctx.render_pass_color_attachments(self.render_pass);
        let raw_id = match unsafe { mq_ctx.texture_raw_id(attachments[0]) } {
            mq::RawId::OpenGl(id) => id as u64,
        };

        egui::TextureId::User(raw_id)
    }
}

fn get_vertex_buffer(mq_ctx: &mut mq::Context) -> mq::BufferId {
    // Draw a rectangle
    #[rustfmt::skip]
    let shape = [
    	Vertex { position: [ -1.0,  1.0 ],  tex_coords: [0.0, 1.0] },
        Vertex { position: [  1.0,  1.0 ],  tex_coords: [1.0, 1.0] },
        Vertex { position: [  1.0, -1.0 ],  tex_coords: [1.0, 0.0] },
        Vertex { position: [ -1.0, -1.0 ],  tex_coords: [0.0, 0.0] },
    ];

    mq_ctx.new_buffer(
        mq::BufferType::VertexBuffer,
        mq::BufferUsage::Immutable,
        mq::BufferSource::slice(shape.as_slice()),
    )
}
