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
    fn free_scope(&mut self, memory: &mut Memory, layer: &HashMap<String, MemPointer>) {
        for var in layer.values() {
            let ptr = *var;
            
            memory.rm_ref(ptr);
            memory.free(ptr);
        }
    }
    
    pub fn decl_var(&mut self, name: String, value: MemPointer) {
        self.layers.last_mut().unwrap().insert(name, value);
    }
    pub fn set_var(&mut self, memory: &mut Memory, name: String, value: MemPointer) {
        memory.add_ref(value);
        for layer in &mut self.layers {
            if layer.contains_key(&name) {
                let ptr = *layer.get(&name).unwrap();
                memory.rm_ref(ptr);
                memory.free(ptr);
                layer.insert(name, value);
                return;
            }
        }
        self.decl_var(name, value);
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