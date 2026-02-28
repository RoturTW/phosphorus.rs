use std::fmt::{Display, Formatter};
use crate::shared::position::Position;

#[derive(Clone, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position
}

impl Range {
    pub fn get_text(&self) -> &str {
        self.start.script.get(self.start.i..self.end.i).unwrap()
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}