// Compute
@group(0) @binding(0) var<storage, read_write> bins: array<atomic<u32>>;
@group(0) @binding(1) var ourTexture: texture_2d<f32>;

@compute @workgroup_size(1)
fn cs_main(@builtin(global_invocation_id) global_invocation_id: vec3u) {
	let numBins = f32(arrayLength(&bins));
	let lastBinIndex = u32(numBins - 1);

	let position = global_invocation_id.xy;
	let color = textureLoad(ourTexture, position, 0);
	let v = srgbLuminance(color.rgb);
	let bin = min(u32(v * numBins), lastBinIndex);
	atomicAdd(&bins[bin], 1u);
}

// from: https://www.w3.org/WAI/GL/wiki/Relative_luminance
const kSRGBLuminanceFactors = vec3f(0.2126, 0.7152, 0.0722);

fn srgbLuminance(color: vec3f) -> f32 {
  return saturate(dot(color, kSRGBLuminanceFactors));
}
