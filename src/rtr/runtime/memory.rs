use std::fmt::Debug;
use crate::rtr::runtime::value::{RTRValue};

const MEM_DEBUG: bool = false;

fn debug_log(indent: u8, txt: &str) {
    if MEM_DEBUG {
        println!("{}{}", "    ".repeat(indent as usize + 1), txt);
    }
}

#[derive(Default, Debug)]
pub struct Memory {
    pub cells: Vec<Option<MemCell>>
}

impl Memory {
    pub fn get_open(&mut self) -> usize {
        self.cells.iter().position(Option::is_none)
            .unwrap_or_else(|| {
                self.cells.push(None);
                self.cells.len() - 1
            })
    }
    pub fn get_open_pointer(&mut self) -> MemPointer {
        let i = self.get_open();
        MemPointer { id: MemId(i) }
    }
    
    pub fn alloc(&mut self, val: Box<dyn RTRValue>) -> MemPointer {
        let ptr = self.get_open_pointer();
        self.cells[ptr.id.0] = Some(MemCell {
            val,
            id: MemId(ptr.id.0),
            refs: 0
        });
        ptr
    }
    pub fn free(&mut self, ptr: MemPointer) {
        if !self.has_cell(ptr) {
            debug_log(0, &format!("free {}, doesnt exist", ptr.id.0));
            return;
        }
        debug_log(0, &format!("free {}", ptr.id.0));
        
        let cell = self.get_cell(ptr);
        debug_log(1, &format!("{:?}", cell.val));
        
        if cell.refs > 0 {
            debug_log(1, &format!("couldnt free {}, has ref of {}", ptr.id.0, cell.refs));
            return;
        }
        
        let val = self.cells[ptr.id.0].take().unwrap().val;
        val.free(self);
    }
    
    pub fn add_ref(&mut self, ptr: MemPointer) {
        assert!(self.has_cell(ptr), "ptr to dead cell");
        debug_log(0, &format!("add ref {}", ptr.id.0));
        let cell = self.cells[ptr.id.0].as_mut().unwrap();
        debug_log(1, &format!("now {}", cell.refs + 1));
        cell.refs += 1;
    }
    pub fn rm_ref(&mut self, ptr: MemPointer) {
        assert!(self.has_cell(ptr), "ptr to dead cell");
        debug_log(0, &format!("rm ref {}", ptr.id.0));
        let cell = self.cells[ptr.id.0].as_mut().unwrap();
        if cell.refs > 0 {
            cell.refs -= 1;
        }
        debug_log(1, &format!("now {}", cell.refs));
    }
    
    pub fn has_cell(&self, ptr: MemPointer) -> bool {
        self.cells.get(ptr.id.0).is_some_and(Option::is_some)
    }
    pub fn get_cell(&self, ptr: MemPointer) -> &MemCell {
        assert!(self.has_cell(ptr), "ptr to dead cell");
        self.cells[ptr.id.0].as_ref().unwrap()
    }
    pub fn get_cell_mut(&mut self, ptr: MemPointer) -> &mut MemCell {
        assert!(self.has_cell(ptr), "ptr to dead cell");
        self.cells[ptr.id.0].as_mut().unwrap()
    }
    pub fn get_cell_option(&self, ptr: MemPointer) -> Option<&MemCell> {
        self.cells.get(ptr.id.0)?.as_ref()
    }
    
    pub fn get(&self, ptr: MemPointer) -> &dyn RTRValue {
        assert!(self.has_cell(ptr), "ptr to dead cell");
        debug_log(0, &format!("get {}", ptr.id.0));
        self.get_cell(ptr).val.as_ref()
    }
    pub fn get_option(&self, ptr: MemPointer) -> Option<&dyn RTRValue> {
        debug_log(0, &format!("get {}", ptr.id.0));
        Some(self.get_cell_option(ptr)?.val.as_ref())
    }
    pub fn get_mut(&mut self, ptr: MemPointer) -> &mut dyn RTRValue {
        assert!(self.has_cell(ptr), "ptr to dead cell");
        debug_log(0, &format!("get mut {}", ptr.id.0));
        self.get_cell_mut(ptr).val.as_mut()
    }
    
    /*
    pub fn get_as<T: RTRValue + 'static>(&self, ptr: MemPointer) -> Option<&T> {
        self.get(ptr).as_any().downcast_ref::<T>()
    }
    pub fn get_as_mut<T: RTRValue + 'static>(&mut self, ptr: MemPointer) -> Option<&mut T> {
        self.get_mut(ptr).as_any_mut().downcast_mut::<T>()
    }
     */
}

#[derive(Debug)]
pub struct MemCell {
    pub val: Box<dyn RTRValue>,
    #[allow(unused)]
    pub id: MemId,
    pub refs: usize
}

#[derive(Clone, Copy, Debug)]
pub struct MemPointer {
    pub id: MemId
}

impl MemPointer {
    pub fn free(self, memory: &mut Memory) {
        memory.free(self);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MemId(pub usize);