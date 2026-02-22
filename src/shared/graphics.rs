use raylib;
use raylib::color::Color as RLColor;
use raylib::ffi::Vector2;
use raylib::prelude::{RaylibDraw, TraceLogLevel};
use crate::shared::area::Area;
use crate::shared::color::{Color};
use crate::shared::graphics_utils::{draw_rectangle_rounded_corners, measure_text, Rounding};
use crate::shared::vec::{Vec2};

pub struct GLCtx {
    pub rl: raylib::RaylibHandle,
    pub thread: raylib::RaylibThread
}

impl GLCtx {
    pub fn running(&self) -> bool {
        !self.rl.window_should_close()
    }
    
    pub fn width(&self) -> i32 {
        self.rl.get_render_width()
    }
    pub fn height(&self) -> i32 {
        self.rl.get_render_height()
    }
    
    pub fn begin_drawing(&'_ mut self) -> GLDrawHandle<'_> {
        GLDrawHandle {
            handle: self.rl.begin_drawing(&self.thread)
        }
    }
}

pub struct GLDrawHandle<'a> {
    pub handle: raylib::drawing::RaylibDrawHandle<'a>
}

impl GLDrawHandle<'_> {
    pub fn clear_background(&mut self, color: Color) {
        self.handle.clear_background(
            <Color as Into<RLColor>>::into(color.into())
        );
    }
    
    pub fn draw_rectangle(&mut self, area: &Area, color: Color) {
        let start: Vector2 = area.a.into();
        let dimensions: Vector2 = area.dimensions().into();
        self.handle.draw_rectangle_lines(
            start.x as i32,
            start.y as i32,
            dimensions.x as i32,
            dimensions.y as i32,
            <Color as Into<RLColor>>::into(color.into())
        );
    }
    pub fn draw_filled_rectangle(&mut self, area: &Area, r: &Rounding, color: Color) {
        draw_rectangle_rounded_corners(
            &mut self.handle,
            area.a.into(),
            area.b.into(),
            r,
            color.into()
        );
    }
    
    pub fn draw_text(&mut self, text: &str, pos: Vec2, font_size: f32, color: Color) {
        self.handle.draw_text(
            text,
            pos.0 as i32,
            pos.1 as i32,
            font_size as i32,
            <Color as Into<RLColor>>::into(color.into())
        );
    }
    
    pub fn text_line_width(&self, text: &str) -> i32 {
        Self::text_line_width_raw(text)
    }
    pub fn text_line_height(&self, _text: &str) -> i32 {
        10
    }
    
    pub fn text_line_width_raw(text: &str) -> i32 {
        measure_text(text, 1)
    }
}

pub fn init() -> GLCtx {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .resizable()
        .title("phosphorus maybe :3")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();
    
    rl.set_target_fps(60);
    
    GLCtx {
        rl,
        thread
    }
}
