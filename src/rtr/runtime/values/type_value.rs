use crate::rtr::runtime::memory::Memory;
use crate::rtr::runtime::value::{RTRValue, TypeValue};
use crate::rtr::runtime::values::str_value::RTRStr;

#[derive(Debug, Clone)]
pub struct RTRType {
    pub data: TypeValue
}

impl RTRValue for RTRType {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, _memory: &mut Memory) -> Box<dyn RTRValue> {
        Box::new(RTRType {
            data: self.data.clone()
        })
    }
    fn get_type(&self) -> TypeValue { TypeValue::Type }
    
    fn stringify(&self, _memory: &Memory) -> String {
        format!("<type:{}>", self.data)
    }
    
    fn equal(&self, other: &dyn RTRValue) -> bool {
        if let Some(other) = other.to_type() {
            self.data == other.data
        } else {
            false
        }
    }
    
    fn to_type(&self) -> Option<RTRType> { Some(self.clone()) }
}
