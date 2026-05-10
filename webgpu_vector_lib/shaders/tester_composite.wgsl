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

    let near = texel * 1.5;
    let far = texel * 4.0;

    var glow = textureSample(glow_texture, glow_sampler, input.uv).rgb * 0.28;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(near.x, 0.0)).rgb * 0.1;
    glow += textureSample(glow_texture, glow_sampler, input.uv - vec2<f32>(near.x, 0.0)).rgb * 0.1;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(0.0, near.y)).rgb * 0.1;
    glow += textureSample(glow_texture, glow_sampler, input.uv - vec2<f32>(0.0, near.y)).rgb * 0.1;
    glow += textureSample(glow_texture, glow_sampler, input.uv + near).rgb * 0.055;
    glow += textureSample(glow_texture, glow_sampler, input.uv - near).rgb * 0.055;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(near.x, -near.y)).rgb * 0.055;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(-near.x, near.y)).rgb * 0.055;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(far.x, 0.0)).rgb * 0.035;
    glow += textureSample(glow_texture, glow_sampler, input.uv - vec2<f32>(far.x, 0.0)).rgb * 0.035;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(0.0, far.y)).rgb * 0.035;
    glow += textureSample(glow_texture, glow_sampler, input.uv - vec2<f32>(0.0, far.y)).rgb * 0.035;
    glow += textureSample(glow_texture, glow_sampler, input.uv + far).rgb * 0.02;
    glow += textureSample(glow_texture, glow_sampler, input.uv - far).rgb * 0.02;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(far.x, -far.y)).rgb * 0.02;
    glow += textureSample(glow_texture, glow_sampler, input.uv + vec2<f32>(-far.x, far.y)).rgb * 0.02;

    let scanline = 0.86 + 0.14 * step(0.5, fract(input.position.y * 0.5));
    let vignette = 1.0 - smoothstep(0.25, 0.92, distance(input.uv, vec2<f32>(0.5, 0.5)));
    let color = glow * 1.65 * scanline * (0.78 + 0.22 * vignette);

    return vec4<f32>(color, 1.0);
}
