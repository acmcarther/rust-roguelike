#![feature(core, custom_attribute, plugin)]
#![plugin(gfx_macros)]

extern crate game_window;
extern crate game_logic;

extern crate cgmath;
extern crate gfx_device_gl;
extern crate gfx;

use game_window::{Context, WindowDependencies};
use game_logic::LogicState;

use std::iter::IteratorExt;
use std::iter::count;
use std::num::Float;

use gfx::traits::*;
use gfx_device_gl::GlResources;

pub type Frame    = gfx::render::target::Frame<GlResources>;
pub type GLDevice = gfx_device_gl::GlDevice;
pub type Program  = gfx::device::handle::Program<GlResources>;
pub type Renderer = gfx::render::Renderer<GlResources, gfx_device_gl::CommandBuffer>;

#[vertex_format]
#[derive(Copy)]
struct Vertex {
    #[name = "a_Pos"]
    pos: [f32; 2],

    #[name = "a_Color"]
    color: [f32; 3],
}

static VERTEX_SRC: &'static [u8] = b" #version 120
    attribute vec2 a_Pos;
    attribute vec3 a_Color;
    varying vec4 v_Color;
    void main() {
        v_Color = vec4(a_Color, 1.0);
        gl_Position = vec4(a_Pos, 0.0, 1.0);
    }
";

static FRAGMENT_SRC: &'static [u8] = b"
    #version 120
    varying vec4 v_Color;
    void main() {
        gl_FragColor = v_Color;
    }
";


pub struct GfxDependencies {
  frame: Frame,
  device: GLDevice,
  program: Program,
  renderer: Renderer
}

impl GfxDependencies{
  pub fn new(frame: Frame, device: GLDevice, program: Program, renderer: Renderer)
    -> GfxDependencies{
    GfxDependencies { frame: frame, device: device, program: program, renderer: renderer }
  }
}

pub fn init_dependencies(window_deps: &mut WindowDependencies) -> GfxDependencies {
  let dimensions = window_deps.dimensions();
  let frame = gfx::Frame::new(dimensions.x, dimensions.y);
  let mut device = gfx_device_gl::GlDevice::new(|s| window_deps.window().get_proc_address(s));
  let program = device.link_program(VERTEX_SRC, FRAGMENT_SRC)
                      .ok().expect("Failed to link program");
  let renderer = device.create_renderer();
  GfxDependencies::new(frame, device, program, renderer)
}

pub fn render(logic_state: &LogicState, gfx: &mut GfxDependencies, window_deps: &mut WindowDependencies) {

  // Future render pipeline
  //-> generate_render_data
  //-> map convert_to_batch
  //-> bundle
  //-> submit_to_device
  //-> finalize

  let clear_data = gfx::ClearData {
      color: [0.3, 0.3, 0.3, 1.0],
      depth: 1.0,
      stencil: 0,
  };

  let vertex_data = generate_vertices(logic_state.edge_count());
  let mesh = gfx.device.create_mesh(&vertex_data.as_slice());
  let slice = mesh.to_slice(gfx::PrimitiveType::TriangleFan);
  let state = gfx::DrawState::new();

  gfx.renderer.reset();
  gfx.renderer.clear(clear_data, gfx::COLOR, &gfx.frame);
  gfx.renderer.draw(&gfx::batch::bind(&state, &mesh, slice.clone(), &gfx.program, &None), &gfx.frame).unwrap();
  gfx.device.submit(gfx.renderer.as_buffer());

  window_deps.swap_buffers();
  gfx.device.after_frame();
}

// TODO: Remove below
// Temporary rendering helpers to demo gfx

fn generate_vertices(edges: usize) -> Vec<Vertex> {
  let float_edges = edges as f32;
  let size = 1.0;
  let polygon = std::iter::count(0,1)
    .map(|idx| generate_vertex(size, (((idx as f32) - 1.0) / float_edges)))
    .take(edges).collect();

  polygon
}

fn generate_vertex(size: f32, perc: f32) -> Vertex {
  let pi = 3.14159;
  let sqrt_2 = (2.0).sqrt();

  let theta = 2.0*pi*(perc);
  let x = size * theta.cos();
  let y = size * theta.sin();
  let coords: [f32; 2] = [x, y];

  let r = 2.0 - distance(coords, [0.0, 2.0]);
  let g = 2.0 - distance(coords, [-sqrt_2, -sqrt_2]);
  let b = 2.0 - distance(coords, [sqrt_2, -sqrt_2]);
  let colors: [f32; 3] = [r, g, b];
  Vertex { pos: coords, color: colors }
}

fn distance(p1: [f32; 2], p2: [f32; 2]) -> f32 {
  let x_sq = (p1[0] - p2[0]).powi(2);
  let y_sq = (p1[1] - p2[1]).powi(2);
  let result = (x_sq + y_sq).sqrt();
  result
}

