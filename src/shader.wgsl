struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

// Vertex shader

struct Camera {
    input_values: vec4<f32>,
    view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    if (model.position.z == 0.0) {
        // x-axis scalable billboard
        let world_position = model_matrix * vec4<f32>(0.0, model.position.y, 0.0, 1.0);
        var out: VertexOutput;
        out.tex_coords = model.tex_coords;
        out.clip_position = camera.view_proj * world_position + vec4((model.position.x / camera.input_values.x) * 2.66, 0.0, 0.0, 0.0);


        // scalable billboard
        // let world_position = model_matrix * vec4<f32>(0.0, 0.0, 0.0, 1.0);
        // var out: VertexOutput;
        // out.tex_coords = model.tex_coords;
        // out.clip_position = camera.view_proj * world_position + vec4((model.position.x / camera.input_values.x) * 2.0, model.position.y * 2.0, 0.0, 0.0);


        // unscalable billboard
        // let world_position = model_matrix * vec4<f32>(0.0, 0.0, 0.0, 1.0);
        // var out: VertexOutput;
        // out.tex_coords = model.tex_coords;
        // let clipposition: vec4<f32> = camera.view_proj * world_position + vec4((model.position.x / camera.input_values.x) * 2.0, model.position.y * 2.0, 0.0, 0.0);
        // let clipposition: vec4<f32> = camera.view_proj * world_position + vec4((model.position.x / camera.input_values.x) * 2.0 * clipposition.z * 0.3, model.position.y * 2.0 * clipposition.z * 0.3, 0.0, 0.0);
        // let clipposition: vec4<f32> = vec4(clipposition.xy, 1.0, clipposition.w);
        // out.clip_position = clipposition;
        return out;
    } else {
        let world_position = model_matrix * vec4<f32>(model.position, 1.0);
        var out: VertexOutput;
        out.tex_coords = model.tex_coords;
        out.clip_position = camera.view_proj * world_position;
        return out;
    }
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
