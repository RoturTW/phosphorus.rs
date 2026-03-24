use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::value::{RTRValue, TypeValue};
use crate::rtr::runtime::values::null_value::RTRNull;
use crate::rtr::runtime::values::num_value::RTRNum;
use crate::rtr::runtime::values::obj_value::RTRObj;

#[derive(Debug, Clone)]
pub struct RTRArr {
    pub items: Vec<MemPointer>
}

impl RTRValue for RTRArr {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, memory: &mut Memory) -> Box<dyn RTRValue> {
        let mut new_items = Vec::new();
        for item in &self.items {
            let val = memory.get(*item).clone_box().dupe(memory);
            new_items.push(memory.alloc(val));
        }
        Box::new(RTRArr {
            items: new_items
        })
    }
    
    fn get_type(&self) -> TypeValue { TypeValue::Arr }
    fn length(&self, _memory: &Memory) -> usize {
        self.items.len()
    }
    
    fn stringify(&self, memory: &Memory) -> String {
        let mut str = String::new();
        for item in self.items.iter().enumerate() {
            str = format!("{str}{}", memory.get(*item.1).stringify_format(memory));
            if item.0 < self.items.len() - 1 {
                str = format!("{str}, ");
            }
        }
        format!("[{str}]")
    }
    fn boolify(&self) -> bool {
        !self.items.is_empty()
    }
    fn arrify(&self, _memory: &mut Memory) -> Vec<MemPointer> {
        self.items.clone()
    }
    
    fn get_item(&self, memory: &mut Memory, key: &dyn RTRValue) -> MemPointer {
        // TODO: handle out of range
        // TODO: allow for negative indexes
        *self.items.get(key.numbify().trunc() as usize)
            .unwrap_or(&memory.alloc(Box::new(RTRNull)))
    }
    
    fn keys(&self, memory: &mut Memory) -> Vec<MemPointer> {
        (0..self.items.len())
            .map(|n| memory.alloc(Box::new(RTRNum {
                data: n as f32
            })))
            .collect()
    }
    fn values(&self, _memory: &mut Memory) -> Vec<MemPointer> {
        self.items.clone()
    }
    
    fn has(&self, memory: &mut Memory, key: &dyn RTRValue) -> bool {
        for item in &self.items {
            if memory.get(*item).equal(key) {
                return true;
            }
        }
        false
    }
    
    fn to_arr(&self) -> Option<RTRArr> { Some(self.clone()) }
}
