use crate::rwl::RWLInstance;

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
}
