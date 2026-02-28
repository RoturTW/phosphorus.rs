use colored::Colorize;
use std::env;
use std::path::PathBuf;
use crate::rtr::RTRInstance;
use crate::shared::area::Area;
use crate::shared::color::Color;
use crate::shared::document::Document;
use crate::shared::fs::read_file;
use crate::shared::theme::Theme;
use crate::shared::vec::Vec2;

mod rtr;
mod rwl;
mod shared;

pub(crate) use shared::logging::{print_log, LogKind, LogSource, Log};
use crate::rtr::ast::node::EventTarget;

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "phosphorus maybe :3".to_string(),
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

async fn app() {
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
        
        gl_ctx.finish_frame().await;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let test = env::var("TEST").unwrap_or_else(|_| "NONE".to_string());
    
    match test.as_str() {
        "RTR" => {
            let mut inst = RTRInstance::new();
            
            inst.parse(&read_file(&PathBuf::from("./assets/rtr/test.rtr")).unwrap());
            
            //println!("{inst:?}");
            
            let out = inst.run_event_target(&EventTarget::Global {
                name: String::from("onload")
            });
            
            if let Err(err) = out {
                print_error!(LogSource::Rtr, "Err: {}", err);
            }
        }
        
        "NONE" => {
            app().await;
        }
        
        _ => {
            print_error!(LogSource::None, "{}", format!("unknown test '{test}'").bright_red());
        }
    }
}