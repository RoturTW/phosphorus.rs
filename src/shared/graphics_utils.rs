use raylib::prelude::*;

#[derive(Debug, Clone)]
pub struct Rounding {
    tl: f32,
    tr: f32,
    bl: f32,
    br: f32
}

impl Default for Rounding {
    fn default() -> Self {
        0.0.into()
    }
}

impl From<f32> for Rounding {
    fn from(value: f32) -> Self {
        Rounding {
            tl: value,
            tr: value,
            bl: value,
            br: value
        }
    }
}

pub fn draw_rectangle_rounded_corners(
    handle: &mut RaylibDrawHandle,
    a: Vector2,
    b: Vector2,
    rounding: &Rounding,
    color: Color,
) {
    let x = a.x.min(b.x);
    let y = a.y.min(b.y);
    let width = (a.x - b.x).abs();
    let height = (a.y - b.y).abs();
    
    let r_top_left = rounding.tl.min(width / 2.0).min(height / 2.0);
    let r_top_right = rounding.tr.min(width / 2.0).min(height / 2.0);
    let r_bottom_right = rounding.br.min(width / 2.0).min(height / 2.0);
    let r_bottom_left = rounding.bl.min(width / 2.0).min(height / 2.0);
    
    // Center
    handle.draw_rectangle(
        (x + r_top_left) as i32,
        (y + r_top_left) as i32,
        (width - r_top_left - r_top_right) as i32,
        (height - r_top_left - r_bottom_left) as i32,
        color,
    );
    
    // Sides
    handle.draw_rectangle(
        x as i32,
        (y + r_top_left) as i32,
        r_top_left as i32,
        (height - r_top_left - r_bottom_left) as i32,
        color,
    );
    
    handle.draw_rectangle(
        (x + width - r_top_right) as i32,
        (y + r_top_right) as i32,
        r_top_right as i32,
        (height - r_top_right - r_bottom_right) as i32,
        color,
    );
    
    handle.draw_rectangle(
        (x + r_top_left) as i32,
        y as i32,
        (width - r_top_left - r_top_right) as i32,
        r_top_left as i32,
        color,
    );
    
    handle.draw_rectangle(
        (x + r_bottom_left) as i32,
        (y + height - r_bottom_left) as i32,
        (width - r_bottom_left - r_bottom_right) as i32,
        r_bottom_left as i32,
        color,
    );
    
    // Corners
    handle.draw_circle_sector(
        Vector2::new(x + r_top_left, y + r_top_left),
        r_top_left,
        180.0,
        270.0,
        16,
        color,
    );
    
    handle.draw_circle_sector(
        Vector2::new(x + width - r_top_right, y + r_top_right),
        r_top_right,
        270.0,
        360.0,
        16,
        color,
    );
    
    handle.draw_circle_sector(
        Vector2::new(x + width - r_bottom_right, y + height - r_bottom_right),
        r_bottom_right,
        0.0,
        90.0,
        16,
        color,
    );
    
    handle.draw_circle_sector(
        Vector2::new(x + r_bottom_left, y + height - r_bottom_left),
        r_bottom_left,
        90.0,
        180.0,
        16,
        color,
    );
}

use raylib::ffi;
use std::ffi::CString;

pub fn measure_text(text: &str, font_size: i32) -> i32 {
    let c_text = CString::new(text).unwrap();
    unsafe { ffi::MeasureText(c_text.as_ptr(), font_size) }
}

