struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)       color        : vec3<f32>
};

@vertex
fn vs_main(vin: VertexInput) -> VertexOutput {
    var vout: VertexOutput;
    vout.clip_position = camera.view_proj * vec4<f32>(vin.position, 1.0);
    vout.color = (vin.position + vec3<f32>(1.0, 1.0, 1.0)) / 2.0;
    vout.color = vec3<f32>(1.0, vout.color[0], vout.color[1]);
    return vout;
}

@fragment
fn fs_main(fin: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(fin.color, 1.0);
}
