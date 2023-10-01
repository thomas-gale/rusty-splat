#import bevy_pbr::utils
// *** Notes on the bevy_pbr::utils shader ***
// Source: https://raw.githubusercontent.com/bevyengine/bevy/v0.11.3/crates/bevy_pbr/src/render/utils.wgsl

// #define_import_path bevy_pbr::utils

// const PI: f32 = 3.141592653589793;
// const HALF_PI: f32 = 1.57079632679;
// const E: f32 = 2.718281828459045;

// fn hsv2rgb(hue: f32, saturation: f32, value: f32) -> vec3<f32> {
//     let rgb = clamp(
//         abs(
//             ((hue * 6.0 + vec3<f32>(0.0, 4.0, 2.0)) % 6.0) - 3.0
//         ) - 1.0,
//         vec3<f32>(0.0),
//         vec3<f32>(1.0)
//     );

//     return value * mix(vec3<f32>(1.0), rgb, vec3<f32>(saturation));
// }

// fn random1D(s: f32) -> f32 {
//     return fract(sin(s * 12.9898) * 43758.5453123);
// }

// // returns the (0-1, 0-1) position within the given viewport for the current buffer coords .
// // buffer coords can be obtained from `@builtin(position).xy`.
// // the view uniform struct contains the current camera viewport in `view.viewport`.
// // topleft = 0,0
// fn coords_to_viewport_uv(position: vec2<f32>, viewport: vec4<f32>) -> vec2<f32> {
//     return (position - viewport.xy) / viewport.zw;
// }


#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput
// **** Notes on the fullscreen vertex shader vertex shader ****
// Source: https://raw.githubusercontent.com/bevyengine/bevy/v0.11.3/crates/bevy_core_pipeline/src/fullscreen_vertex_shader/fullscreen.wgsl

// #define_import_path bevy_core_pipeline::fullscreen_vertex_shader

// struct FullscreenVertexOutput {
//     @builtin(position)
//     position: vec4<f32>,
//     @location(0)
//     uv: vec2<f32>,
// };

// // This vertex shader produces the following, when drawn using indices 0..3:
// //
// //  1 |  0-----x.....2
// //  0 |  |  s  |  . ´
// // -1 |  x_____x´
// // -2 |  :  .´
// // -3 |  1´
// //    +---------------
// //      -1  0  1  2  3
// //
// // The axes are clip-space x and y. The region marked s is the visible region.
// // The digits in the corners of the right-angled triangle are the vertex
// // indices.
// //
// // The top-left has UV 0,0, the bottom-left has 0,2, and the top-right has 2,0.
// // This means that the UV gets interpolated to 1,1 at the bottom-right corner
// // of the clip-space rectangle that is at 1,-1 in clip space.
// @vertex
// fn fullscreen_vertex_shader(@builtin(vertex_index) vertex_index: u32) -> FullscreenVertexOutput {
//     // See the explanation above for how this works
//     let uv = vec2<f32>(f32(vertex_index >> 1u), f32(vertex_index & 1u)) * 2.0;
//     let clip_position = vec4<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), 0.0, 1.0);

//     return FullscreenVertexOutput(clip_position, uv);
// }

// ***************************************************************************

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
  return vec4(in.uv, 1.0, 1.0);
}
