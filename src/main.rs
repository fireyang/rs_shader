#[macro_use]
extern crate glium;
// extern crate rs_shader;

// use rs_shader::*;
use glium::{glutin, Surface};
use std::path::Path;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const SCREEN: [Vertex; 4] = [
    Vertex{position: [ 0.0,  1.0]}, // Top right
    Vertex{position: [-1.0,  1.0]}, // Top left
    Vertex{position: [-1.0, -1.0]}, // Bottom left
    Vertex{position: [ 1.0, -1.0]}, // Bottom right
];

pub static DEFAULT_VERT_SRC_STR: &'static str = include_str!("../shaders/test.vs");

const SCREEN_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];


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

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();


    let mut closed = false;
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
                &uniform! { matrix: matrix },
                &Default::default(),
            )
            .unwrap();


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

extern crate image;
use image::GenericImage;

use glium::texture::RawImage2d;
// use glium::texture::ToClientFormat;

/*
fn load_texture<'a>(texpath: &'a Option<String>) -> glium::texture::RawImage2d<'a, T> {
    // let image = image::load(Cursor::new(&include_bytes!("../book/tuto-14-diffuse.jpg")[..]),
    // image::JPEG).unwrap().to_rgba();
    let im = image::open(&Path::new(&texpath.clone().unwrap())).unwrap();
    let image_dimensions = im.dimensions();
    let im = RawImage2d::from_raw_rgba_reversed(&im.into_raw(), image_dimensions);
    im
}*/
