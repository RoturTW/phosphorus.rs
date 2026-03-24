use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::value::{RTRValue, TypeValue};
use crate::rtr::runtime::values::str_value::RTRStr;

#[derive(Debug, Clone)]
pub struct RTRNum {
    pub data: f32
}

impl RTRValue for RTRNum {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, _memory: &mut Memory) -> Box<dyn RTRValue> {
        Box::new(RTRNum {
            data: self.data
        })
    }
    fn get_type(&self) -> TypeValue { TypeValue::Str }
    
    fn length(&self, memory: &Memory) -> usize {
        self.stringify(memory).len()
    }
    
    fn stringify(&self, _memory: &Memory) -> String {
        self.data.to_string()
    }
    fn boolify(&self) -> bool {
        self.data > 0.0
    }
    fn numbify(&self) -> f32 {
        self.data
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
    
    fn get_item(&self, memory: &mut Memory, key: &dyn RTRValue) -> MemPointer {
        // TODO: handle out of range
        // TODO: allow for negative indexes
        memory.alloc(Box::new(RTRStr {
            data: self.data.to_string()[key.numbify().trunc() as usize..].to_string()
        }))
    }
    
    fn equal(&self, other: &dyn RTRValue) -> bool {
        if let Some(other) = other.to_num() {
            (self.data - other.data).abs() < 0.000_000_1
        } else {
            false
        }
    }
    
    fn to_num(&self) -> Option<RTRNum> { Some(self.clone()) }
}
