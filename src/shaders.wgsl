struct VertexOutput {
	@builtin(position) pos: vec4<f32>,
}

@vertex
fn vs_main() -> VertexOutput {
	var out: VertexOutput;
	out.pos = vec4<f32>(0.0, 0.0, 0.0, 0.0);

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
