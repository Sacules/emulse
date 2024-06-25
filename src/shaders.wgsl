// Vertex
struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) pos: vec4<f32>,
	@location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
	var out: VertexOutput;
	out.tex_coords = model.tex_coords;
	out.pos = vec4<f32>(model.pos, 1.0);

	return out;
}


// Fragment
@group(0) @binding(0)
var texture_in: texture_2d<f32>;

@group(0) @binding(1)
var sampler_in: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	var tex = textureSample(texture_in, sampler_in, in.tex_coords);

	return desaturate(tex.rgb);
}

fn desaturate(color: vec3<f32>) -> vec4<f32> {
	var new_color = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
	return vec4<f32>(new_color, new_color, new_color, 1.0);
}
