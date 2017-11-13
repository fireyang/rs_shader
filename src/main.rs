#[macro_use]
extern crate glium;
// extern crate rs_shader;

// use rs_shader::*;
use glium::{glutin, Surface};
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const SCREEN: [Vertex; 4] = [
    Vertex{position: [ 1.0,  1.0]}, // Top right
    Vertex{position: [-1.0,  1.0]}, // Top left
    Vertex{position: [-1.0, -1.0]}, // Bottom left
    Vertex{position: [ 1.0, -1.0]}, // Bottom right
];

pub static DEFAULT_VERT_SRC_STR: &'static str = include_str!("../shaders/test2.vs");

const SCREEN_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

// Fragment shader prefix
const PREFIX: &'static str = "
    #version 150 core

    uniform float     iGlobalTime;
    uniform float     iTime;
    uniform vec3      iResolution;
    uniform vec4      iMouse;
    uniform int       iFrame;
    uniform sampler2D iChannel0;
    uniform sampler2D iChannel1;
    uniform sampler2D iChannel2;
    uniform sampler2D iChannel3;

    in vec2 fragCoord;
    out vec4 fragColor;
";

// Fragment shader suffix
const SUFFIX: &'static str = "
    void main() {
        mainImage(fragColor, fragCoord);
    }
";

pub fn format_shader_src(src: &str) -> String {
    format!("{}\n{}\n{}", PREFIX, src, SUFFIX)
}

pub fn load_fragment_shader(file: &str) -> io::Result<String> {
    let mut frag_src_str = String::new();
    File::open(&Path::new(&file))?.read_to_string(
        &mut frag_src_str,
    )?;
    let s = format_shader_src(&frag_src_str);
    Ok(s)
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();


    //定义顶点
    let positions = glium::VertexBuffer::new(&display, &SCREEN).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &SCREEN_INDICES,
    ).unwrap();

    let vertex_shader_src = DEFAULT_VERT_SRC_STR;

    let file = "./shaders/test2.fs";
    let fragment_shader_src = load_fragment_shader(&file).unwrap();
    // let fragment_shader_src = fragment_shader_src.as_slice();

    let program =
        glium::Program::from_source(&display, vertex_shader_src, &fragment_shader_src, None)
            .unwrap();


    // let file = "textures/01-brickwall.jpg";
    // let f = Some(file);
    let f = None;
    let im = load_texture(&f);
    let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, im).unwrap();



    let mut closed = false;
    let mut time: f32 = 0.0;
    let start_time = Instant::now();
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        target
            .draw(
                &positions,
                &indices,
                &program,
                &uniform! { matrix: matrix , tex: &diffuse_texture, iTime: time},
                &Default::default(),
            )
            .unwrap();
        let elapsed = get_tm(&start_time);
        // println!("time: {}", elapsed);

        time = elapsed;

        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                }
            }
            _ => (),
        });
    }
}

fn get_tm(tm: &Instant) -> f32 {
    let elapsed = tm.elapsed();
    let elapsed_ms = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64;
    (elapsed_ms as f32) / 1000.0
}

extern crate image;
// use image::GenericImage;

use glium::texture::RawImage2d;
// use glium::texture::ToClientFormat;
pub static DEFAULT_TEXTURE0_BUF: &'static [u8] = include_bytes!("../textures/black.png");

fn load_texture<'a>(texpath: &'a Option<&str>) -> glium::texture::RawImage2d<'a, u8> {
    // let image = image::load(Cursor::new(&include_bytes!("../book/tuto-14-diffuse.jpg")[..]),
    // image::JPEG).unwrap().to_rgba();
    let im = if texpath.is_some() {
        image::open(&Path::new(&texpath.clone().unwrap()))
            .unwrap()
            .flipv()
            .to_rgba()
    } else {
        image::load_from_memory(DEFAULT_TEXTURE0_BUF)
            .unwrap()
            .flipv()
            .to_rgba()
    };
    // let im = image::open(&Path::new(&texpath.clone().unwrap())).unwrap().to_rgba();
    let image_dimensions = im.dimensions();
    let im = RawImage2d::from_raw_rgba_reversed(&im.into_raw(), image_dimensions);
    im
}
