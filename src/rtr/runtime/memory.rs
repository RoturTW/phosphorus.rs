use crate::rtr::runtime::value::Value;

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
        
        MemPointer {
            id: MemId(i)
        }
    }
    
    // operations
    pub fn alloc(&mut self, val: Value) -> MemPointer {
        let ptr = self.get_open_pointer();
        
        debug_log(0, &format!("alloc {} {:?}", ptr.id.0, val));
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
        
        let val = self.get(ptr).clone();
        val.free(self);
        
        self.cells[ptr.id.0] = None;
    }
    
    pub fn add_ref(&mut self, ptr: MemPointer) {
        debug_log(0, &format!("add ref {}", ptr.id.0));
        self.cells[ptr.id.0].as_mut().unwrap().refs += 1;
    }
    pub fn rm_ref(&mut self, ptr: MemPointer) {
        debug_log(0, &format!("rm ref {}", ptr.id.0));
        let cell = self.cells[ptr.id.0].as_mut().unwrap();
        if cell.refs > 0 {
            cell.refs -= 1;
        }
    }
    
    // accessing
    pub fn has_cell(&self, ptr: MemPointer) -> bool {
        self.cells[ptr.id.0].is_some()
    }
    pub fn get_cell(&self, ptr: MemPointer) -> &MemCell {
        assert!(self.has_cell(ptr), "ptr to dead cell");
        self.cells[ptr.id.0].as_ref().unwrap()
    }
    pub fn get_cell_mut(&mut self, ptr: MemPointer) -> &mut MemCell {
        assert!(self.has_cell(ptr), "ptr to dead cell");
        self.cells[ptr.id.0].as_mut().unwrap()
    }
    
    pub fn get(&self, ptr: MemPointer) -> &Value {
        assert!(self.has_cell(ptr), "ptr to dead cell");
        debug_log(0, &format!("get {}", ptr.id.0));
        
        let cell = self.get_cell(ptr);
        
        &cell.val
    }
    pub fn get_mut(&mut self, ptr: MemPointer) -> &mut Value {
        let cell = self.get_cell_mut(ptr);
        
        &mut cell.val
    }
}

#[derive(Debug)]
pub struct MemCell {
    pub val: Value,
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
pub struct MemId (pub usize);
