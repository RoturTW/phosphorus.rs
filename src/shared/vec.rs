use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 (pub f32, pub f32);

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;
    
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl ops::Div<Vec2> for Vec2 {
    type Output = Vec2;
    
    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

// macroquad utils
impl From<Vec2> for macroquad::math::Vec2 {
    fn from(v: Vec2) -> Self {
        macroquad::math::Vec2::new(v.0, v.1)
    }
}
impl From<macroquad::math::Vec2> for Vec2 {
    fn from(v: macroquad::math::Vec2) -> Self {
        Vec2(v.x, v.y)
    }
}