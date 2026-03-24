use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::value::{RTRValue, TypeValue};

#[derive(Debug, Clone)]
pub struct RTRStr {
    pub data: String
}

impl RTRValue for RTRStr {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, _memory: &mut Memory) -> Box<dyn RTRValue> {
        Box::new(RTRStr {
            data: self.data.clone()
        })
    }
    fn get_type(&self) -> TypeValue { TypeValue::Str }
    
    fn length(&self, _memory: &Memory) -> usize { self.data.len() }
    
    fn stringify(&self, _memory: &Memory) -> String {
        self.data.clone()
    }
    fn stringify_format(&self, _memory: &Memory) -> String {
        format!("{:?}", self.data)
    }
    fn boolify(&self) -> bool {
        !self.data.is_empty()
    }
    fn numbify(&self) -> f32 {
        self.data.parse().unwrap_or(f32::NAN)
    }
    fn arrify(&self, memory: &mut Memory) -> Vec<MemPointer> {
        self.data
            .chars()
            .map(|c| {
                memory.alloc(Box::new(RTRStr { data: c.to_string() }))
            })
            .collect()
    }
    
    fn get_item(&self, memory: &mut Memory, key: &dyn RTRValue) -> MemPointer {
        // TODO: handle out of range
        // TODO: allow for negative indexes
        memory.alloc(Box::new(RTRStr {
            data: self.data[key.numbify().trunc() as usize..].to_string()
        }))
    }
    
    fn equal(&self, other: &dyn RTRValue) -> bool {
        if let Some(other) = other.to_str() {
            self.data == other.data
        } else {
            false
        }
    }
    
    fn to_str(&self) -> Option<RTRStr> { Some(self.clone()) }
}
