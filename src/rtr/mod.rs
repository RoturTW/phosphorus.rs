use crate::{print_raw, print_warn, Log, LogKind, print_log};
use crate::rtr::ast::node::{AstProgram, AstStatement, AstTopLevelStatement, BinaryOp, EventTarget, UnaryOp};
use crate::rtr::ast::parser::Parser;
use crate::rtr::ast::tokenise;
use crate::rtr::error::Error;
use crate::rtr::runtime::compiler;
use crate::rtr::runtime::instruction::VmInstruction;
use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::scope::Scope;
use crate::rtr::runtime::value::{BuiltinFunction, Function, TypeValue, Value};
use crate::shared::logging::LogSource;

pub mod ast;
pub mod runtime;
mod error;

#[derive(Debug)]
struct Event {
    pub body: Vec<AstStatement>,
    pub target: EventTarget
}

#[derive(Debug)]
pub struct RTRInstance {
    pub ast: Option<AstProgram>,
    
    // runtime
    pub stack: Vec<MemPointer>,
    pub scope: Scope,
    pub memory: Memory
}

impl RTRInstance {
    pub fn new() -> RTRInstance {
        
        RTRInstance {
            ast: None,
            
            stack: Vec::new(),
            scope: Scope::new(),
            memory: Memory::default()
        }
    }
    
    fn set_alloc(&mut self, name: &str, value: Value) {
        let ptr = self.memory.alloc(value);
        self.scope.set_var(
            &mut self.memory,
            String::from(name),
            ptr
        );
    }
    pub fn new_scope(&mut self) {
        self.set_alloc("log",
               Value::Function(Function::Builtin(BuiltinFunction::Log))
        );
    }
    
    pub fn parse(&mut self, src: &str) {
        let mut parser = Parser {
            pointer: 0,
            tokens: tokenise(src)
        };
        
        let out = parser.parse();
        match out {
            Err(err) => {
                print_warn!(LogSource::Rtr, "{err}");
            }
            Ok(ast) => {
                self.ast = Some(ast);
            }
        }
    }
    
    // TODO: find a better way to do this? :sob:
    fn get_eligible(&mut self, target: &EventTarget) -> Vec<Event> {
        if self.ast.is_none() {
            return Vec::new()
        }
        
        let ast_events = &self.ast.as_ref().unwrap().events;
        let mut events = Vec::new();
        
        for event in ast_events {
            #[allow(irrefutable_let_patterns)]
            if let AstTopLevelStatement::Event {
                body,
                target: ev_target
            } = event
                && match (ev_target, target.clone()) {
                    (EventTarget::Global { name: ev_name }, EventTarget::Global { name }) =>
                        *ev_name == name,
                    _ => false
                } {
                    events.push(Event {
                        body: body.clone(),
                        target: ev_target.clone()
                    });
                }
        }
        
        events
    }
    
    pub fn run_event_target(&mut self, target: &EventTarget) -> Result<(), Error> {
        // TODO: cache compiled segments :P
        let events = self.get_eligible(target);
        
        for event in events {
            self.run_event(event)?;
        }
        
        Ok(())
    }
    fn run_event(&mut self, event: Event) -> Result<(), Error> {
        let instructions = self.compile_event(event.body)?;
        //println!("{instructions:?}");
        self.run_instructions(&instructions)?;
        
        Ok(())
    }
    
    fn pop_stack(&mut self) -> (&Value, MemPointer) {
        let ptr = self.pop_stack_raw().unwrap();
        (self.memory.get(ptr), ptr)
    }
    fn pop_stack_mut(&mut self) -> (&mut Value, MemPointer) {
        let ptr = self.pop_stack_raw().unwrap();
        (self.memory.get_mut(ptr), ptr)
    }
    fn pop_stack_ptr(&mut self) -> MemPointer {
        self.pop_stack_raw().unwrap()
    }
    fn pop_stack_raw(&mut self) -> Option<MemPointer> {
        self.stack.pop()
    }
    
    fn push_stack_alloc(&mut self, val: Value) {
        let ptr = self.memory.alloc(val);
        self.push_stack_ptr(ptr);
    }
    fn push_stack_ptr(&mut self, val: MemPointer) {
        self.stack.push(val);
    }
    
    pub fn run_instructions(&mut self, instructions: &[VmInstruction]) -> Result<Option<Value>, Error> {
        self.new_scope();
        
        let mut i = 0;
        while i < instructions.len() {
            let inst = &instructions[i];
            
            //println!("{:?} {:?}", inst, self.stack);
            
            match inst {
                // program flow
                
                // operations
                VmInstruction::Call(arg_count) => {
                    let mut args = Vec::new();
                    for _ in 0..*arg_count {
                        args.push(self.pop_stack_ptr());
                    }
                    args.reverse();
                    
                    let func_ptr = self.pop_stack_ptr();
                    self.memory.add_ref(func_ptr);
                    
                    let out_ptr = {
                        let func_val = self.memory.get_mut(func_ptr).clone();
                        func_val.call(&mut self.memory, &args)?
                    };
                    
                    self.stack.push(out_ptr);
                    
                    self.memory.rm_ref(func_ptr);
                    self.memory.free(func_ptr);
                    for arg in args {
                        self.memory.free(arg);
                    }
                }
                VmInstruction::Unary(op) => {
                    let (right_val, right_ptr) = self.pop_stack();
                    
                    let right_val = right_val.clone();
                    
                    self.push_stack_alloc(match op {
                        UnaryOp::Minus =>
                            Value::Num { data: -<Value as Into<f32>>::into(right_val) },
                        UnaryOp::Number =>
                            Value::Num { data: <Value as Into<f32>>::into(right_val) },
                        
                        UnaryOp::Invert =>
                            Value::Bool { data: !<Value as Into<bool>>::into(right_val) },
                        UnaryOp::Boolify =>
                            Value::Bool { data: <Value as Into<bool>>::into(right_val) }
                    });
                    
                    self.memory.free(right_ptr);
                }
                VmInstruction::Binary(op) => {
                    
                    let (right_val, right_ptr) = self.pop_stack();
                    let right_val = right_val.clone();
                    let (left_val, left_ptr) = self.pop_stack();
                    let left_val = left_val.clone();
                    
                    self.push_stack_alloc(Self::run_binary_op(&left_val, &right_val, op));
                    
                    self.memory.free(right_ptr);
                    self.memory.free(left_ptr);
                }
                
                // values
                VmInstruction::Str(str) => {
                    self.push_stack_alloc(Value::Str { data: str.clone() });
                }
                VmInstruction::Num(num) => {
                    self.push_stack_alloc(Value::Num { data: *num });
                }
                VmInstruction::Percentage(num) => {
                    self.push_stack_alloc(Value::Percentage { data: *num });
                }
                
                // scope
                VmInstruction::Get(name) => {
                    let var = self.scope.get_var(name);
                    if let Some(ptr) = var {
                        self.push_stack_ptr(ptr);
                    } else {
                        self.push_stack_alloc(Value::Null);
                    }
                }
                VmInstruction::Decl(name) => {
                    let ptr = self.pop_stack_ptr();
                    self.memory.add_ref(ptr);
                    self.scope.set_var(&mut self.memory, name.clone(), ptr);
                    self.push_stack_ptr(ptr);
                }
                // assignments
                // stack
                VmInstruction::Pop => {
                    let ptr = self.pop_stack_ptr();
                    self.memory.free(ptr);
                }
                
                _ => {
                    print_warn!(LogSource::Rtr, "couldnt run instruction '{:?}'", inst);
                }
            }
            
            i += 1;
        }
        
        Ok(None)
    }
    fn run_binary_op(left_val: &Value, right_val: &Value, op: &BinaryOp) -> Value {
        fn get_num(left_val: &Value, right_val: &Value) -> Option<(f32, f32)> {
            if let (
                Value::Num { data: left },
                Value::Num { data: right }
            ) = (&left_val, &right_val) {
                Some((*left, *right))
            } else {
                None
            }
        }
        
        match op {
            BinaryOp::Add =>
                if let Some((left, right)) = get_num(left_val, right_val) {
                    Value::Num { data: left + right }
                } else {
                    Value::Str { data: format!("{left_val}{right_val}") }
                }
            BinaryOp::Sub =>
                if let Some((left, right)) = get_num(left_val, right_val) {
                    Value::Num { data: left - right }
                } else {
                    Value::Num { data: f32::NAN }
                }
            BinaryOp::Mul =>
                if let Some((left, right)) = get_num(left_val, right_val) {
                    Value::Num { data: left * right }
                } else {
                    Value::Num { data: f32::NAN }
                }
            BinaryOp::Div =>
                if let Some((left, right)) = get_num(left_val, right_val) {
                    Value::Num { data: left / right }
                } else {
                    Value::Num { data: f32::NAN }
                }
            BinaryOp::Mod =>
                if let Some((left, right)) = get_num(left_val, right_val) {
                    Value::Num { data: left % right }
                } else {
                    Value::Num { data: f32::NAN }
                }
            BinaryOp::Pow =>
                if let Some((left, right)) = get_num(left_val, right_val) {
                    Value::Num { data: left.powf(right) }
                } else {
                    Value::Num { data: f32::NAN }
                }
            
            _ => panic!()
        }
    }
    
    #[allow(clippy::unused_self)]
    pub fn compile_event(&self, statement: Vec<AstStatement>) -> Result<Vec<VmInstruction>, Error> {
        compiler::compile_statements(statement)
    }
}
