use std::f32;
use crate::shared::vec::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Area {
    pub a: Vec2,
    pub b: Vec2
}

impl Area {
    pub fn center(&self) -> Vec2 {
        Vec2 (
            f32::midpoint(self.a.0, self.b.0),
            f32::midpoint(self.a.1, self.b.1)
        )
    }
    pub fn dimensions(&self) -> Vec2 {
        Vec2 (
            self.b.0 - self.a.0,
            self.b.1 - self.a.1,
        )
    }
    
    pub fn width(&self) -> f32 {
        self.b.0 - self.a.0
    }
    pub fn height(&self) -> f32 {
        self.b.1 - self.a.1
    }
    
    pub fn width_total(&self) -> f32 {
        self.b.0 + self.a.0
    }
    pub fn height_total(&self) -> f32 {
        self.b.1 + self.a.1
    }
    
    pub fn pad(&self, amount: Area) -> Area {
        Area {
            a: Vec2 (
                self.a.0 + amount.a.0,
                self.a.1 + amount.a.1,
            ),
            b: Vec2 (
                self.b.0 - amount.b.0,
                self.b.1 - amount.b.1,
            )
        }
    }
    pub fn margin(&self, amount: Area) -> Area {
        self.pad(amount.flip())
    }
    
    pub fn left(&self) -> f32 {
        self.a.0
    }
    pub fn right(&self) -> f32 {
        self.b.0
    }
    pub fn top(&self) -> f32 {
        self.a.1
    }
    pub fn bottom(&self) -> f32 {
        self.b.1
    }
    
    pub fn flip(&self) -> Area {
        Area {
            a: Vec2 (
                -self.a.0,
                -self.a.1,
            ),
            b: Vec2 (
                -self.b.0,
                -self.b.1,
            )
        }
    }
}

impl From<i32> for Vec2 {
    fn from(value: i32) -> Self {
        Vec2(value as f32, value as f32)
    }
}