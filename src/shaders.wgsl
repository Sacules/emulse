// Vertex
struct VertexUniform {
	matrix: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> vert_uniform: VertexUniform;

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
	out.pos = vert_uniform.matrix * vec4<f32>(model.pos, 1.0);

	return out;
}


// Fragment
@group(0) @binding(0)
var texture_in: texture_2d<f32>;

@group(0) @binding(1)
var sampler_in: sampler;

struct FragmentUniform {
    contrast: f32,
	saturation: f32,
	brightness: f32,
}

@group(2) @binding(0)
var<uniform> frag_uniform: FragmentUniform;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	var tex = textureSample(texture_in, sampler_in, in.tex_coords);

	return saturation(frag_uniform.saturation) * brightness(frag_uniform.brightness) * contrast(frag_uniform.contrast) * tex;
}

fn saturation(val: f32) -> mat4x4<f32> {
	var luminance = vec3<f32>(0.3086, 0.6094, 0.0820);
	var oneMinusSat = 1.0 - val;
	var redVal = 1.0;
	var greenVal = 1.0;
	var blueVal = 1.0;

	var red = vec3<f32>(luminance.x * oneMinusSat);
	red+= vec3<f32>(val, 0, 0) * redVal;

	var green = vec3<f32>(luminance.y * oneMinusSat );
	green += vec3<f32>(0, val, 0) * greenVal;

	var blue = vec3<f32>(luminance.z * oneMinusSat);
	blue += vec3<f32>(0, 0, val) * blueVal;

	return mat4x4<f32>(vec4<f32>(red, 0),vec4<f32>(green,0),vec4<f32>(blue,0),vec4<f32>(0, 0, 0, 1));
}

fn contrast(val: f32) -> mat4x4<f32> {
   var t: f32 = (1.0 - val) / 2.0;

   return mat4x4<f32>(
		vec4<f32>(val, 0, 0, 0),
		vec4<f32>(0, val, 0, 0),
		vec4<f32>(0, 0, val, 0),
		vec4<f32>(t, t, t, 1)
	);
}

fn brightness(val: f32) -> mat4x4<f32> {
	return mat4x4<f32>(
		vec4<f32>(1, 0, 0, 0),
		vec4<f32>(0, 1, 0, 0),
		vec4<f32>(0, 0, 1, 0),
		vec4<f32>(val, val, val, 1)
	);
}

// Relative luminance
fn desaturate(color: vec3<f32>) -> vec4<f32> {
	var new_color = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
	return vec4<f32>(new_color, new_color, new_color, 1.0);
}
