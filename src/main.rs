use std::fs;
use std::path::PathBuf;
use crate::shared::area::Area;
use crate::shared::color::Color;
use crate::shared::document::Document;
use crate::shared::theme::Theme;
use crate::shared::vec::Vec2;

mod rwl;
mod shared;

fn read_file(path: &PathBuf) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("file not found: {}", path.display()));
    }
    fs::read_to_string(path)
        .map_err(|e| format!("{}: {}", path.display(), e))
}

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "phosphorus maybe :3".to_string(),
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut doc = Document::new();
    doc.rwl_instance.parse(&read_file(&PathBuf::from("./assets/rwl/test.rwl")).unwrap());
    doc.rwl_instance.instance();
    
    let mut gl_ctx = shared::graphics::GLCtx::new().await;
    let theme = Theme::default();
    let mut last_size = Vec2(0.0, 0.0);
    
    loop {
        if !gl_ctx.running() { break; }
        
        let width = gl_ctx.width();
        let height = gl_ctx.height();
        
        let mut handle = gl_ctx.begin_drawing();
        
        handle.clear_background(Color { r: 0, g: 0, b: 0, a: 255 });
        
        if last_size != Vec2(width, height) {
            doc.rwl_instance.update((&mut handle, &theme), &Area {
                a: Vec2(0.0, 0.0),
                b: Vec2(width, height),
            });
            last_size = Vec2(width, height);
        }
        
        doc.rwl_instance.render(&mut handle);
        
        macroquad::window::next_frame().await;
    }
}