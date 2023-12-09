struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) vertex_index: u32,
}

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(2) @binding(0) // 1.
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
    @builtin(vertex_index) vertex_index: u32,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    let i = vertex_index%u32(3);
    out.vertex_index = vertex_index;
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@group(0) @binding(0)
var<storage, read> vertexs: array<VertexInput>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // let color = textureSample(t_diffuse, s_diffuse, vec2<f32>(in.tex_coords.x, 1.0 - in.tex_coords.y));
    let color = vertexs[in.vertex_index].position;
    return vec4<f32>(color, 1.0);
}