use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Debug)]
pub struct Position {
    pub ln: usize,
    pub col: usize,
    pub i: usize,
    pub script: String
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ln, self.col)
    }
}

impl Add<usize> for Position {
    type Output = Position;
    
    fn add(self, rhs: usize) -> Self::Output {
        Position {
            ln: self.ln,
            col: self.col + rhs,
            i: self.i + rhs,
            script: self.script
        }
    }
}

impl Sub<usize> for Position {
    type Output = Position;
    
    fn sub(self, rhs: usize) -> Self::Output {
        Position {
            ln: self.ln,
            col: self.col - rhs,
            i: self.i - rhs,
            script: self.script
        }
    }
}

impl AddAssign<usize> for Position {
    fn add_assign(&mut self, rhs: usize) {
        *self = self.clone() + rhs
    }
}

impl SubAssign<usize> for Position {
    fn sub_assign(&mut self, rhs: usize) {
        *self = self.clone() - rhs
    }
}