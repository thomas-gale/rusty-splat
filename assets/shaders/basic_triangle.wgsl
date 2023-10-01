#import bevy_pbr::utils

// We will use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
  return vec4(in.uv, 0.0, 1.0);
}
