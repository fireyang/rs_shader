use imgui::*;
use imgui::{ImGui, Ui};
use imgui_glium_renderer::Renderer;

pub fn hello_world<'a>(ui: &Ui<'a>) -> bool {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos.0,
                mouse_pos.1
            ));
        });

    true
}

use std::cell::Cell;

pub struct UI{
    ui: Cell<ImGui>,
    t: Cell<u32>,
    // render: Renderer,
}

use glium::Display;

impl UI{
    pub fn new() -> UI{
        let im = ImGui::init();
        UI{
            ui: Cell::new(im),
            t: Cell::new(5),
        }
    }

    pub fn test(&self){
        let a = &self.t.get();
        // let mut c = a;
        println!("{:?}", a);
        // configure_keys(&mut a);
    }
    /*
    pub fn new(display: &Display) -> UI{
        let mut im = ImGui::init();
        im.set_ini_filename(None);
        configure_keys(&mut im);

        let mut renderer = Renderer::init(&mut im, display).expect("Failed to initialize renderer");
        UI{
            ui: &mut im,
            render: renderer,
        }
    }
    */

    // pub fn poll_event(&self, event){
    // }
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
