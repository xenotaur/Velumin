struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) segment_start: vec2<f32>,
    @location(3) segment_end: vec2<f32>,
    @location(4) radius: f32,
    @location(5) core_width: f32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) local_position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) segment_start: vec2<f32>,
    @location(3) segment_end: vec2<f32>,
    @location(4) radius: f32,
    @location(5) core_width: f32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.local_position = input.position;
    output.color = input.color;
    output.segment_start = input.segment_start;
    output.segment_end = input.segment_end;
    output.radius = input.radius;
    output.core_width = input.core_width;
    return output;
}

fn distance_to_segment(point: vec2<f32>, start: vec2<f32>, end: vec2<f32>) -> f32 {
    let segment = end - start;
    let length_squared = max(dot(segment, segment), 0.000001);
    let projection = clamp(dot(point - start, segment) / length_squared, 0.0, 1.0);
    let closest = start + segment * projection;
    return distance(point, closest);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let distance_from_beam = distance_to_segment(
        input.local_position,
        input.segment_start,
        input.segment_end,
    );
    let core_radius = input.core_width * 0.35;
    let falloff = 1.0 - smoothstep(core_radius, input.radius, distance_from_beam);
    let emission = falloff * falloff;

    return vec4<f32>(input.color.rgb * emission, input.color.a * emission);
}
