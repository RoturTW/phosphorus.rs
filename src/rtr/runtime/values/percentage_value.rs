use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::value::{RTRValue, TypeValue};
use crate::rtr::runtime::values::str_value::RTRStr;

#[derive(Debug, Clone)]
pub struct RTRPercentage {
    pub data: f32
}

impl RTRValue for RTRPercentage {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, _memory: &mut Memory) -> Box<dyn RTRValue> {
        Box::new(RTRPercentage {
            data: self.data
        })
    }
    fn get_type(&self) -> TypeValue { TypeValue::Str }
    
    fn length(&self, memory: &Memory) -> usize {
        self.stringify(memory).len()
    }
    
    fn stringify(&self, _memory: &Memory) -> String {
        format!("{}%", self.data)
    }
    fn boolify(&self) -> bool {
        self.data > 0.0
    }
    fn numbify(&self) -> f32 {
        self.data / 100.0
    }
    fn arrify(&self, memory: &mut Memory) -> Vec<MemPointer> {
        self.stringify(memory)
            .chars()
            .map(|c| {
                memory.alloc(Box::new(RTRStr {
                    data: c.to_string()
                }))
            })
            .collect()
    }
    
    // TODO: get_item
    
    fn equal(&self, other: &dyn RTRValue) -> bool {
        if let Some(other) = other.to_percentage() {
            (self.data - other.data).abs() < 0.000_000_1
        } else {
            false
        }
    }
    
    fn to_percentage(&self) -> Option<RTRPercentage> { Some(self.clone()) }
}
