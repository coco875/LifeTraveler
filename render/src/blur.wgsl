// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var size_blur: i32 = 10;

    var color : vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    let dimensions = textureDimensions(t_diffuse);

    for (var x : i32 = -size_blur; x <= size_blur; x++) {
        for (var y : i32 = -size_blur; y <= size_blur; y++) {
            var offset: vec2<f32> = vec2<f32>(f32(x)/f32(dimensions.x), f32(y)/f32(dimensions.y));
            color += textureSample(t_diffuse, s_diffuse, in.tex_coords + offset);
        }
    }

    color /= f32(size_blur+2) * f32(size_blur+2);
    // var color : vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    return color;
}