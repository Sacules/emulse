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
	out.pos = vec4<f32>(model.pos, 1.0);

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
	invert: u32,
	temperature: f32,
}

@group(2) @binding(0)
var<uniform> frag_uniform: FragmentUniform;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	var tex = textureSample(texture_in, sampler_in, in.tex_coords);
	if (bool(frag_uniform.invert)) {
		tex = invert(tex);
	}

	return saturation(frag_uniform.saturation) *
		   brightness(frag_uniform.brightness) *
		   contrast(frag_uniform.contrast) *
		   //vec4<f32>(whiteBalance(frag_uniform.temperature), 1.0) *
		   tex;
}

fn invert(val: vec4<f32>) -> vec4<f32> {
	return vec4<f32>(1.0 - val.x, 1.0 - val.y, 1.0 - val.z, 1.0);
}

// Valid from 1000 to 40000 K (and additionally 0 for pure full white)
fn whiteBalance(temperature: f32) -> vec3<f32> {
    // Values from: http://blenderartists.org/forum/showthread.php?270332-OSL-Goodness&p=2268693&viewfull=1#post2268693
    var m: mat3x3<f32>;

    if (temperature >= 6500.0) {
  	    m = mat3x3<f32>(vec3(0.0, -2902.1955373783176, -8257.7997278925690),
			            vec3(0.0, 1669.5803561666639, 2575.2827530017594),
		                vec3(1.0, 1.3302673723350029, 1.8993753891711275));
    } else {
    	m = mat3x3<f32>(vec3(1745.0425298314172, 1216.6168361476490, -8257.7997278925690),
                        vec3(-2666.3474220535695, -2173.1012343082230, 2575.2827530017594),
	                    vec3(0.55995389139931482, 0.70381203140554553, 1.8993753891711275));
    }

  return mix(clamp(vec3<f32>(m[0] / (vec3<f32>(clamp(temperature, 1000.0, 40000.0)) + m[1]) + m[2]), vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0), smoothstep(1000.0, 0.0, temperature));
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

// from: https://www.w3.org/WAI/GL/wiki/Relative_luminance
const kSRGBLuminanceFactors = vec3f(0.2126, 0.7152, 0.0722);
