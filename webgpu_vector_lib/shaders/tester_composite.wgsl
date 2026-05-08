@group(0) @binding(0) var glow_texture: texture_2d<f32>;
@group(0) @binding(1) var glow_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    var uvs = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );

    var output: VertexOutput;
    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    output.uv = uvs[vertex_index];
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let dimensions = vec2<f32>(textureDimensions(glow_texture));
    let texel = 1.0 / dimensions;

    var glow = textureSample(glow_texture, glow_sampler, input.uv).rgb * 0.34;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(texel.x, 0.0)).rgb * 0.12;
    glow += textureSample(glow_texture, glow_sampler, input.uv - vec2<f32>(texel.x, 0.0)).rgb * 0.12;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(0.0, texel.y)).rgb * 0.12;
    glow += textureSample(glow_texture, glow_sampler, input.uv - vec2<f32>(0.0, texel.y)).rgb * 0.12;
    glow += textureSample(glow_texture, glow_sampler, input.uv + texel).rgb * 0.05;
    glow += textureSample(glow_texture, glow_sampler, input.uv - texel).rgb * 0.05;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(texel.x, -texel.y)).rgb * 0.05;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(-texel.x, texel.y)).rgb * 0.05;

    let scanline = 0.72 + 0.28 * step(0.5, fract(input.position.y * 0.5));
    let vignette = 1.0 - smoothstep(0.25, 0.92, distance(input.uv, vec2<f32>(0.5, 0.5)));
    let color = glow * 2.0 * scanline * (0.72 + 0.28 * vignette);

    return vec4<f32>(color, 1.0);
}
