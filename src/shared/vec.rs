use std::ops;
use raylib::prelude::Vector2;

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
pub fn vec_to_raylib(vec: Vec2) -> Vector2 {
    Vector2 {
        x: vec.0,
        y: vec.1
    }
}
pub fn raylib_to_vec(vec: Vector2) -> Vec2 {
    Vec2(vec.x, vec.y)
}