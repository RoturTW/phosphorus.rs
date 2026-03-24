use crate::rtr::ast::node::Parameter;
use crate::rtr::error::Error;
use crate::rtr::log::RTRLog;
use crate::rtr::runtime::function::BuiltinFunction;
use crate::rtr::runtime::instruction::VmInstruction;
use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::value::{RTRValue, TypeValue};

type RTRRustFn = fn(logs: &mut Vec<RTRLog>, memory: &mut Memory, args: &[MemPointer]) -> Result<MemPointer, Error>;

#[derive(Debug, Clone)]
pub enum Function {
    Builtin(BuiltinFunction),
    Rust(RTRRustFn),
    Vm {
        body: Vec<VmInstruction>,
        params: Vec<Parameter>
    },
}

#[derive(Debug, Clone)]
pub struct RTRFunction {
    pub data: Function
}

impl RTRValue for RTRFunction {
    fn clone_box(&self) -> Box<dyn RTRValue> { Box::new(self.clone()) }
    fn dupe(&self, _memory: &mut Memory) -> Box<dyn RTRValue> {
        Box::new(RTRFunction {
            data: self.data.clone()
        })
    }
    fn get_type(&self) -> TypeValue { TypeValue::Function }
    
    fn call(&self, logs: &mut Vec<RTRLog>, memory: &mut Memory, args: &[MemPointer]) -> Result<MemPointer, Error> {
        match &self.data {
            Function::Builtin(builtin) => {
                builtin.call(logs, memory, args)
            }
            // Vm functions are handled in the call instruction
            
            _ => Err(Error::CannotCall {
                func: self.stringify(memory)
            })
        }
    }
    
    fn to_function(&self) -> Option<RTRFunction> { Some(self.clone()) }
}
