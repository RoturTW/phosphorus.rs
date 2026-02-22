use macroquad::prelude::*;
use crate::shared::color::Color;
use crate::shared::vec::Vec2;

#[derive(Debug, Clone)]
pub struct Rounding {
    pub tl: f32,
    pub tr: f32,
    pub bl: f32,
    pub br: f32,
}

impl Default for Rounding {
    fn default() -> Self { 0.0.into() }
}

impl From<f32> for Rounding {
    fn from(value: f32) -> Self {
        Rounding { tl: value, tr: value, bl: value, br: value }
    }
}

pub fn draw_rectangle_rounded_corners(
    x: f32, y: f32, width: f32, height: f32,
    rounding: &Rounding, color: Color,
) {
    let mq: macroquad::color::Color = color.into();
    
    let tl = rounding.tl.min(width / 2.0).min(height / 2.0);
    let tr = rounding.tr.min(width / 2.0).min(height / 2.0);
    let br = rounding.br.min(width / 2.0).min(height / 2.0);
    let bl = rounding.bl.min(width / 2.0).min(height / 2.0);
    
    // center
    draw_rectangle(x + tl, y + tl, width - tl - tr, height - tl - bl, mq);
    
    draw_rectangle(x, y + tl, tl, height - tl - bl, mq);
    draw_rectangle(x + width - tr, y + tr, tr, height - tr - br, mq);
    draw_rectangle(x + tl, y, width - tl - tr, tl, mq);
    draw_rectangle(x + bl, y + height - bl, width - bl - br, bl, mq);
    
    // corners
    draw_corner(x + tl,         y + tl,          tl, 180.0, 270.0, mq);
    draw_corner(x + width - tr, y + tr,           tr, 270.0, 360.0, mq);
    draw_corner(x + width - br, y + height - br,  br,   0.0,  90.0, mq);
    draw_corner(x + bl,         y + height - bl,  bl,  90.0, 180.0, mq);
}

fn draw_corner(
    cx: f32, cy: f32, radius: f32,
    start_deg: f32, end_deg: f32,
    color: macroquad::color::Color,
) {
    if radius <= 0.0 {
        return;
    }
    let segments = 16;
    let step = (end_deg - start_deg) / segments as f32;
    
    for i in 0..segments {
        let a1 = (start_deg + step * i as f32).to_radians();
        let a2 = (start_deg + step * (i + 1) as f32).to_radians();
        
        let x1 = cx + radius * a1.cos();
        let y1 = cy + radius * a1.sin();
        let x2 = cx + radius * a2.cos();
        let y2 = cy + radius * a2.sin();
        
        draw_triangle(
            macroquad::math::Vec2::new(cx, cy),
            macroquad::math::Vec2::new(x1, y1),
            macroquad::math::Vec2::new(x2, y2),
            color,
        );
    }
}

pub fn measure_text_width(text: &str, font_size: f32) -> f32 {
    let m = measure_text(text, None, font_size as u16, 1.0);
    m.width
}

pub fn render_text(text: &str, pos: Vec2, font_size: f32, color: Color) {
    draw_text(text, pos.0, pos.1 + font_size, font_size, color.into());
}

pub async fn load_default_font() -> Font {
    let bytes = include_bytes!("../../assets/font/NotoSans-Regular.ttf");
    load_ttf_font_from_bytes(bytes).unwrap()
}