use crate::{print_raw, print_warn, Log, LogKind, print_log, print_error};
use crate::rtr::ast::node::EventTarget;
use crate::rtr::RTRModule;
use crate::shared::logging::LogSource;

#[derive(Debug)]
pub struct RWLScript {
    pub module: RTRModule
}
impl RWLScript {
    pub fn new(mut module: RTRModule) -> RWLScript {
        module.inject();
        
        RWLScript {
            module
        }
    }
    
    pub fn init(&mut self) {
        self.run_event_target(&EventTarget::Global {
            name: String::from("onload")
        });
    }
    
    pub fn run_event_target(&mut self, target: &EventTarget) {
        let out = self.module.run_event_target(target);
        
        if let Err(err) = out {
            print_error!(LogSource::Rtr, "{}", err);
        }
    }
}
