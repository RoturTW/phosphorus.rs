use crate::rwl::RWLInstance;
use crate::shared::graphics::GLDrawHandle;

#[derive(Debug)]
pub struct Document {
    pub rwl_instance: RWLInstance
}

impl Document {
    pub fn new() -> Document {
        Document {
            rwl_instance: RWLInstance::new()
        }
    }
    
    pub fn render(&mut self, d: &mut GLDrawHandle) {
        self.rwl_instance.render(d);
    }
}
