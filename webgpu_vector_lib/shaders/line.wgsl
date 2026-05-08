@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var position = vec2<f32>(-0.75, -0.02);

    switch vertex_index {
        case 1u: {
            position = vec2<f32>(0.75, -0.02);
        }
        case 2u: {
            position = vec2<f32>(0.75, 0.02);
        }
        case 3u: {
            position = vec2<f32>(-0.75, -0.02);
        }
        case 4u: {
            position = vec2<f32>(0.75, 0.02);
        }
        case 5u: {
            position = vec2<f32>(-0.75, 0.02);
        }
        default: {}
    }

    return vec4<f32>(position, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);  // White lines
}
