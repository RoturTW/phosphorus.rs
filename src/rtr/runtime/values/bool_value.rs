use crate::rtr::runtime::memory::{Memory};
use crate::rtr::runtime::value::{RTRValue, TypeValue};

#[derive(Debug, Clone)]
pub struct RTRBool {
    pub data: bool
}

impl RTRValue for RTRBool {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, _memory: &mut Memory) -> Box<dyn RTRValue> {
        Box::new(RTRBool {
            data: self.data
        })
    }
    fn get_type(&self) -> TypeValue { TypeValue::Str }
    
    fn stringify(&self, _memory: &Memory) -> String {
        self.data.to_string()
    }
    fn boolify(&self) -> bool { self.data }
    fn numbify(&self) -> f32 {
        if self.data { 1.0 } else { 0.0 }
    }
    
    fn equal(&self, other: &dyn RTRValue) -> bool {
        if let Some(other) = other.to_bool() {
            self.data == other.data
        } else {
            false
        }
    }
    
    fn to_bool(&self) -> Option<RTRBool> { Some(self.clone()) }
}
