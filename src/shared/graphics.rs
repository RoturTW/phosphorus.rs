use macroquad::prelude::*;
use crate::shared::area::Area;
use crate::shared::color::Color;
use crate::shared::graphics_utils::{draw_rectangle_rounded_corners, load_default_font, Rounding};
use crate::shared::vec::Vec2;

pub struct GLCtx {
    pub font: Font,
    pub running: bool,
}

impl GLCtx {
    pub async fn new() -> Self {
        let font = load_default_font().await;
        Self {
            running: true,
            font
        }
    }
    
    #[allow(clippy::unused_self)]
    pub async fn finish_frame(&self) {
        next_frame().await;
    }
    
    #[allow(clippy::unused_self)]
    pub fn running(&self) -> bool {
        self.running
    }
    
    #[allow(clippy::unused_self)]
    pub fn width(&self) -> f32 {
        screen_width()
    }
    
    #[allow(clippy::unused_self)]
    pub fn height(&self) -> f32 {
        screen_height()
    }
    
    pub fn begin_drawing(&mut self) -> GLDrawHandle<'_> {
        GLDrawHandle {
            font: &self.font,
            font_size: 16.0,
            running: &mut self.running
        }
    }
}

pub struct GLDrawHandle<'a> {
    pub font: &'a Font,
    pub font_size: f32,
    running: &'a mut bool,
}

impl GLDrawHandle<'_> {
    #[allow(clippy::unused_self)]
    pub fn clear_background(&self, color: Color) {
        clear_background(color.into());
    }
    
    #[allow(clippy::unused_self)]
    pub fn draw_rectangle(&self, area: &Area, color: Color) {
        let start: Vec2 = area.a;
        let dim: Vec2 = area.dimensions();
        let mq: macroquad::color::Color = color.into();
        
        let x = start.0;
        let y = start.1;
        let w = dim.0;
        let h = dim.1;
        
        draw_line(x,     y,     x + w, y,     1.0, mq);
        draw_line(x + w, y,     x + w, y + h, 1.0, mq);
        draw_line(x + w, y + h, x,     y + h, 1.0, mq);
        draw_line(x,     y + h, x,     y,     1.0, mq);
    }
    
    #[allow(clippy::unused_self)]
    pub fn draw_filled_rectangle(&self, area: &Area, r: &Rounding, color: Color) {
        let start: Vec2 = area.a;
        let dim: Vec2 = area.dimensions();
        draw_rectangle_rounded_corners(start.0, start.1, dim.0, dim.1, r, color);
    }
    
    pub fn draw_text(&mut self, text: &str, pos: Vec2, font_size: f32, color: Color) {
        draw_text_ex(
            text,
            pos.0, pos.1 + font_size,
            TextParams {
                font: Some(self.font),
                font_size: font_size as u16,
                color: color.into(),
                ..Default::default()
            }
        );
    }
    
    pub fn text_line_width(&self, text: &str, font_size: f32) -> f32 {
        measure_text(text, Some(self.font), font_size as u16, 1.0).width
    }
    
    pub fn text_line_height(&self, _text: &str) -> f32 {
        let m = measure_text("X", None, self.font_size as u16, 1.0);
        m.height
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "phosphorus maybe :3".to_string(),
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}