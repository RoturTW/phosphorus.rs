use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::value::{RTRValue, TypeValue};
use crate::rtr::runtime::values::str_value::RTRStr;
use crate::shared::color::Color;

#[derive(Debug, Clone)]
pub struct RTRColor {
    pub data: Color
}

impl RTRValue for RTRColor {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, _memory: &mut Memory) -> Box<dyn RTRValue> {
        Box::new(RTRColor {
            data: self.data
        })
    }
    fn get_type(&self) -> TypeValue { TypeValue::Str }
    
    fn length(&self, memory: &Memory) -> usize {
        self.stringify(memory).len()
    }
    
    fn stringify(&self, _memory: &Memory) -> String {
        self.data.to_hex_rgb()
    }
    fn boolify(&self) -> bool {
        self.data.above_zero()
    }
    
    fn equal(&self, other: &dyn RTRValue) -> bool {
        if let Some(other) = other.to_color() {
            self.data == other.data
        } else {
            false
        }
    }
    
    fn to_color(&self) -> Option<RTRColor> { Some(self.clone()) }
}
