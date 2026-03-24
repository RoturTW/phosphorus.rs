use crate::rtr::runtime::memory::Memory;
use crate::rtr::runtime::value::{RTRValue, TypeValue};
use crate::rtr::runtime::values::type_value::RTRType;

#[derive(Debug, Clone)]
pub struct RTRNull;

impl RTRValue for RTRNull {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, memory: &mut Memory) -> Box<dyn RTRValue> {
        Box::new(RTRNull)
    }
    fn get_type(&self) -> TypeValue { TypeValue::Null }
    
    fn stringify(&self, memory: &Memory) -> String {
        String::from("null")
    }
    fn boolify(&self) -> bool { false }
    
    fn is_null(&self) -> bool { true }
    
    fn to_null(&self) -> Option<RTRNull> { Some(self.clone()) }
}
