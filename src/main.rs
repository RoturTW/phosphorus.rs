use std::fs;
use std::path::PathBuf;
use std::time::Instant;
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
    
    let src = fs::read_to_string(path)
        .map_err(|e| format!("{}: {}", path.display(), e))?;
    
    Ok(src)
}

fn app() {
    let mut doc = Document::new();
    doc.rwl_instance.parse(read_file(&PathBuf::from("./assets/rwl/test.rwl")).unwrap());
    doc.rwl_instance.instance();
    //println!("{doc:?}");
    
    let mut gl_ctx = shared::graphics::init();
    
    let theme = Theme::default();
    
    let mut last_size = Vec2(0.0,0.0);
    while gl_ctx.running() {
        let width = gl_ctx.width() as f32;
        let height = gl_ctx.height() as f32;
        
        let mut handle = gl_ctx.begin_drawing();
        
        handle.clear_background(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        });
        
        if last_size != Vec2(width, height) {
            //let update_time = Instant::now();
            doc.rwl_instance.update((&mut handle, &theme), &Area {
                a: Vec2(0.0, 0.0),
                b: Vec2(width, height)
            });
            last_size = Vec2(width, height);
            //println!("update took {:?}", update_time.elapsed());
        }
        
        //let render_time = Instant::now();
        doc.rwl_instance.render(&mut handle);
        //println!("render took {:?}", render_time.elapsed());
    }
}

fn main() {
    app();
}
