use std::collections::HashMap;
use crate::rtr::runtime::memory::{MemPointer, Memory};

#[derive(Debug)]
pub struct Scope {
    pub layers: Vec<HashMap<String, MemPointer>>
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            layers: vec![HashMap::new()]
        }
    }
    
    pub fn new_layer(&mut self) {
        self.layers.push(HashMap::new());
    }
    pub fn pop_layer(&mut self) -> HashMap<String, MemPointer> {
        self.layers.pop().unwrap()
    }
    #[allow(clippy::unused_self)]
    pub fn free_layer(&mut self, memory: &mut Memory, layer: &HashMap<String, MemPointer>) {
        for var in layer.values() {
            let ptr = *var;
            
            memory.rm_ref(ptr);
            memory.free(ptr);
        }
    }
    
    pub fn decl_var(&mut self, memory: &mut Memory, name: String, value: MemPointer) {
        let top = self.layers.last_mut().unwrap();
        if let Some(existing) = top.get(name.as_str()) {
            memory.rm_ref(*existing);
            memory.free(*existing);
        }
        memory.add_ref(value);
        top.insert(name, value);
    }
    pub fn set_var(&mut self, memory: &mut Memory, name: String, value: MemPointer) {
        for layer in &mut self.layers {
            if let Some(existing) = layer.get(name.as_str()) {
                memory.rm_ref(*existing);
                memory.free(*existing);
                memory.add_ref(value);
                layer.insert(name, value);
                return;
            }
        }
        self.decl_var(memory, name, value);
    }
    pub fn get_var(&self, name: &str) -> Option<MemPointer> {
        for scope in self.layers.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(*value);
            }
        }
        None
    }
}