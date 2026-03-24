use std::collections::HashMap;
use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::value::{RTRValue, TypeValue};
use crate::rtr::runtime::values::null_value::RTRNull;
use crate::rtr::runtime::values::str_value::RTRStr;

#[derive(Debug, Clone)]
pub struct RTRObj {
    pub data: HashMap<String, MemPointer>
}

impl RTRValue for RTRObj {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, memory: &mut Memory) -> Box<dyn RTRValue> {
        let mut new_map = HashMap::new();
        for item in &self.data {
            let val = memory.get(*item.1).clone_box().dupe(memory);
            new_map.insert(item.0.clone(), memory.alloc(val));
        }
        Box::new(RTRObj {
            data: new_map
        })
    }
    
    fn get_type(&self) -> TypeValue { TypeValue::Obj }
    fn length(&self, _memory: &Memory) -> usize {
        self.data.len()
    }
    
    fn stringify(&self, memory: &Memory) -> String {
        let mut str = String::new();
        for (i, (key, ptr)) in self.data.iter().enumerate() {
            str = format!("{str}{key}: {}", memory.get(*ptr).stringify_format(memory));
            if i < self.data.len() - 1 {
                str = format!("{str}, ");
            }
        }
        format!("{{{str}}}")
    }
    fn boolify(&self) -> bool {
        !self.data.is_empty()
    }
    fn arrify(&self, memory: &mut Memory) -> Vec<MemPointer> {
        self.data
            .keys()
            .map(|key| {
                memory.alloc(Box::new(RTRStr {
                    data: key.clone()
                }))
            })
            .collect()
    }
    
    fn get_item(&self, memory: &mut Memory, key: &dyn RTRValue) -> MemPointer {
        *self.data.get(&key.stringify(memory))
            .unwrap_or(&memory.alloc(Box::new(RTRNull)))
    }
    
    fn keys(&self, memory: &mut Memory) -> Vec<MemPointer> {
        self.data.keys()
            .map(|k| memory.alloc(Box::new(RTRStr { data: k.clone() })))
            .collect()
    }
    fn values(&self, _memory: &mut Memory) -> Vec<MemPointer> {
        self.data.values()
            .copied()
            .collect()
    }
    
    fn has(&self, _memory: &mut Memory, key: &dyn RTRValue) -> bool {
        for item in self.data.keys() {
            if key.equal(&RTRStr { data: item.clone() }) {
                return true;
            }
        }
        false
    }
    
    fn to_obj(&self) -> Option<RTRObj> { Some(self.clone()) }
}
