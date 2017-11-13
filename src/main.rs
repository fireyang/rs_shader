#[macro_use]
extern crate glium;
// extern crate rs_shader;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;
mod ui;

// use rs_shader::*;
use glium::{glutin, Surface};
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::time::Instant;

use imgui::{ImGui, Ui};

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

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    use imgui_glium_renderer::Renderer;
    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    configure_keys(&mut imgui);


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
    let mut run_ui = ui::hello_world;
    let start_time = Instant::now();
    let mut last_frame = Instant::now();
    let mut mouse_state = MouseState::default();

    while !closed {

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => {
                use glium::glutin::WindowEvent::*;
                use glium::glutin::ElementState::Pressed;
                use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};
                match event {
                    Closed => closed = true,
                    KeyboardInput { input, .. } => {
                        use glium::glutin::VirtualKeyCode as Key;
                        let pressed = input.state == Pressed;
                        match input.virtual_keycode {
                            Some(Key::Tab) => imgui.set_key(0, pressed),
                            Some(Key::Left) => imgui.set_key(1, pressed),
                            Some(Key::Right) => imgui.set_key(2, pressed),
                            _ => {}
                        }
                    }
                    MouseMoved { position: (x, y), .. } => mouse_state.pos = (x as i32, y as i32),
                    MouseInput { state, button, .. } => {
                        match button {
                            MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
                            MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
                            MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
                            _ => {}
                        }
                    }
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } |
                    MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse_state.wheel = y,
                    ReceivedCharacter(c) => imgui.add_input_character(c),
                    _ => (),
                }
            }
            _ => (),
        });

        //draw ui
        update_mouse(&mut imgui, &mut mouse_state);

        let gl_window = display.gl_window();
        let size_points = gl_window.get_inner_size_points().unwrap();
        let size_pixels = gl_window.get_inner_size_pixels().unwrap();

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        let ui = imgui.frame(size_points, size_pixels, delta_s);
        if !run_ui(&ui) {
            break;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        renderer.render(&mut target, ui).expect("Rendering failed");

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        // target
        //     .draw(
        //         &positions,
        //         &indices,
        //         &program,
        //         &uniform! { matrix: matrix , tex: &diffuse_texture, iTime: time},
        //         &Default::default(),
        //     )
        //     .unwrap();
        let elapsed = get_tm(&start_time);
        // println!("time: {}", elapsed);

        time = elapsed;

        target.finish().unwrap();

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

fn configure_keys(imgui: &mut ImGui) {
    use imgui::ImGuiKey;

    imgui.set_imgui_key(ImGuiKey::Tab, 0);
    imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
    imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
    imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
    imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
    imgui.set_imgui_key(ImGuiKey::PageUp, 5);
    imgui.set_imgui_key(ImGuiKey::PageDown, 6);
    imgui.set_imgui_key(ImGuiKey::Home, 7);
    imgui.set_imgui_key(ImGuiKey::End, 8);
    imgui.set_imgui_key(ImGuiKey::Delete, 9);
    imgui.set_imgui_key(ImGuiKey::Backspace, 10);
    imgui.set_imgui_key(ImGuiKey::Enter, 11);
    imgui.set_imgui_key(ImGuiKey::Escape, 12);
    imgui.set_imgui_key(ImGuiKey::A, 13);
    imgui.set_imgui_key(ImGuiKey::C, 14);
    imgui.set_imgui_key(ImGuiKey::V, 15);
    imgui.set_imgui_key(ImGuiKey::X, 16);
    imgui.set_imgui_key(ImGuiKey::Y, 17);
    imgui.set_imgui_key(ImGuiKey::Z, 18);
}

fn update_mouse(imgui: &mut ImGui, mouse_state: &mut MouseState) {
    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(
        mouse_state.pos.0 as f32 / scale.0,
        mouse_state.pos.1 as f32 / scale.1,
    );
    imgui.set_mouse_down(
        &[
            mouse_state.pressed.0,
            mouse_state.pressed.1,
            mouse_state.pressed.2,
            false,
            false,
        ],
    );
    imgui.set_mouse_wheel(mouse_state.wheel / scale.1);
    mouse_state.wheel = 0.0;
}
