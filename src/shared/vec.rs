use std::ops;
use raylib::ffi::Vector2 as FFIVector2;
use raylib::prelude::Vector2 as PreludeVector2;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 (pub f32, pub f32);

impl Vec2 {
    pub fn add_vec(self, other: Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    
    fn add(self, rhs: Vec2) -> Vec2 {
        self.add_vec(rhs)
    }
}

// raylib utils
impl From<Vec2> for FFIVector2 {
    fn from(value: Vec2) -> Self {
        FFIVector2 {
            x: value.0,
            y: value.1
        }
    }
}
impl From<FFIVector2> for Vec2 {
    fn from(value: FFIVector2) -> Self {
        Vec2(value.x, value.y)
    }
}
impl From<Vec2> for PreludeVector2 {
    fn from(value: Vec2) -> Self {
        PreludeVector2 {
            x: value.0,
            y: value.1
        }
    }
}
impl From<PreludeVector2> for Vec2 {
    fn from(value: PreludeVector2) -> Self {
        Vec2(value.x, value.y)
    }
}