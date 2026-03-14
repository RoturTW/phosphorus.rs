use std::collections::HashMap;
use std::time::Instant;
use crate::{print_raw, print_warn, Log, LogKind, print_log};
use crate::rtr::ast::node::{AssignmentOp, AstProgram, AstStatement, AstTopLevelStatement, BinaryOp, EventTarget, UnaryOp};
use crate::rtr::ast::parser::Parser;
use crate::rtr::ast::tokenise;
use crate::rtr::error::Error;
use crate::rtr::runtime::compiler;
use crate::rtr::runtime::compiler::CompileContext;
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
        self.set_alloc("true", Value::Bool { data: true });
        self.set_alloc("false", Value::Bool { data: false });
        self.set_alloc("null", Value::Null);
        
        self.set_alloc("log", Value::Function(
            Function::Builtin(BuiltinFunction::Log)
        ));
        self.set_alloc("error", Value::Function(
            Function::Builtin(BuiltinFunction::Error)
        ));
        self.set_alloc("return", Value::Function(
            Function::Builtin(BuiltinFunction::Return)
        ));
        self.set_alloc("typeof", Value::Function(
            Function::Builtin(BuiltinFunction::Typeof)
        ));
        self.set_alloc("length", Value::Function(
            Function::Builtin(BuiltinFunction::Length)
        ));
        
        // mathematical
        self.set_alloc("min", Value::Function(
            Function::Builtin(BuiltinFunction::Min)
        ));
        self.set_alloc("max", Value::Function(
            Function::Builtin(BuiltinFunction::Max)
        ));
        
        self.set_alloc("abs", Value::Function(
            Function::Builtin(BuiltinFunction::Abs)
        ));
        self.set_alloc("sqrt", Value::Function(
            Function::Builtin(BuiltinFunction::Sqrt)
        ));
        
        self.set_alloc("round", Value::Function(
            Function::Builtin(BuiltinFunction::Round)
        ));
        self.set_alloc("floor", Value::Function(
            Function::Builtin(BuiltinFunction::Floor)
        ));
        self.set_alloc("ceil", Value::Function(
            Function::Builtin(BuiltinFunction::Ceil)
        ));
        
        // string
        self.set_alloc("join", Value::Function(
            Function::Builtin(BuiltinFunction::Join)
        ));
        self.set_alloc("split", Value::Function(
            Function::Builtin(BuiltinFunction::Split)
        ));
        
        self.set_alloc("chr", Value::Function(
            Function::Builtin(BuiltinFunction::Chr)
        ));
        self.set_alloc("ord", Value::Function(
            Function::Builtin(BuiltinFunction::Ord)
        ));
        
        self.set_alloc("toUpper", Value::Function(
            Function::Builtin(BuiltinFunction::ToUpper)
        ));
        self.set_alloc("toLower", Value::Function(
            Function::Builtin(BuiltinFunction::ToLower)
        ));
        self.set_alloc("toTitle", Value::Function(
            Function::Builtin(BuiltinFunction::ToTitle)
        ));
        
        // array
        self.set_alloc("item", Value::Function(
            Function::Builtin(BuiltinFunction::Item)
        ));
        self.set_alloc("range", Value::Function(
            Function::Builtin(BuiltinFunction::Range)
        ));
        
        // object
        self.set_alloc("keys", Value::Function(
            Function::Builtin(BuiltinFunction::Keys)
        ));
        self.set_alloc("values", Value::Function(
            Function::Builtin(BuiltinFunction::Values)
        ));
        self.set_alloc("has", Value::Function(
            Function::Builtin(BuiltinFunction::Has)
        ));
        self.set_alloc("obj", Value::Function(
            Function::Builtin(BuiltinFunction::Obj)
        ));
        
        // logical
        self.set_alloc("all", Value::Function(
            Function::Builtin(BuiltinFunction::All)
        ));
        self.set_alloc("any", Value::Function(
            Function::Builtin(BuiltinFunction::Any)
        ));
        self.set_alloc("not", Value::Function(
            Function::Builtin(BuiltinFunction::Not)
        ));
    }
    
    pub fn parse(&mut self, src: &str) {
        let tokens = tokenise(src);
        let mut parser = Parser {
            pointer: 0,
            tokens
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
    
    pub fn run_instructions(&mut self, instructions: &[VmInstruction]) -> Result<Option<MemPointer>, Error> {
        self.new_scope();
        
        let mut labels: HashMap<String, usize> = instructions
            .iter()
            .enumerate()
            .filter_map(|(i, instruction)| {
                if let VmInstruction::Label(name) = &instruction {
                    Some((name.clone(), i))
                } else {
                    None
                }
            })
            .collect();
        
        let mut i = 0;
        let mut instructions_ran = 0;
        while i < instructions.len() && instructions_ran < 10000 {
            instructions_ran += 1;
            let inst = &instructions[i];
            
            if false {
                println!("{:?}\n    {:?}", inst, self.stack.iter().map(|ptr| {
                    let cell = self.memory.get_cell_option(*ptr);
                    if let Some(cell) = cell {
                        format!("{:?}", cell.val)
                    } else {
                        String::from("EMPTY")
                    }
                }).collect::<Vec<_>>());
            }
            
            match inst {
                // program flow
                VmInstruction::Label(..) => (),
                VmInstruction::Jump(lbl) => {
                    i = labels[lbl];
                }
                VmInstruction::JumpIf(lbl) => {
                    let (cond, cond_ptr) = self.pop_stack();
                    if cond.boolify() {
                        i = labels[lbl];
                    }
                    self.memory.free(cond_ptr);
                }
                VmInstruction::JumpNotIf(lbl) => {
                    let (cond, cond_ptr) = self.pop_stack();
                    if !cond.boolify() {
                        i = labels[lbl];
                    }
                    self.memory.free(cond_ptr);
                }
                
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
                        let func = self.memory.get_mut(func_ptr).clone();
                        if let Value::Function(Function::Vm { body, params }) = func {
                            self.scope.new_layer();
                            for (i, param) in params.iter().enumerate() {
                                let arg = *args.get(i).unwrap_or(&self.memory.alloc(Value::Null));
                                self.scope.decl_var(&mut self.memory, param.name.clone(), arg);
                            }
                            self.run_instructions(&body)?.unwrap_or(self.memory.alloc(Value::Null))
                        } else {
                            func.call(&mut self.memory, &args)?
                        }
                    };
                    
                    self.stack.push(out_ptr);
                    
                    self.memory.rm_ref(func_ptr);
                    self.memory.free(func_ptr);
                    for arg in args {
                        self.memory.free(arg);
                    }
                }
                VmInstruction::Unary(op) => {
                    let (right, right_ptr) = self.pop_stack();
                    
                    let right = right.clone();
                    
                    self.push_stack_alloc(match op {
                        UnaryOp::Minus =>
                            Value::Num { data: -<Value as Into<f32>>::into(right) },
                        UnaryOp::Number =>
                            Value::Num { data: <Value as Into<f32>>::into(right) },
                        
                        UnaryOp::Invert =>
                            Value::Bool { data: !<Value as Into<bool>>::into(right) },
                        UnaryOp::Boolify =>
                            Value::Bool { data: <Value as Into<bool>>::into(right) }
                    });
                    
                    self.memory.free(right_ptr);
                }
                VmInstruction::Binary(op) => {
                    let (right, right_ptr) = self.pop_stack();
                    let right = right.clone();
                    let (left, left_ptr) = self.pop_stack();
                    let left = left.clone();
                    
                    let val = self.run_binary_op(&left, &right, op);
                    self.push_stack_alloc(val);
                    
                    self.memory.free(right_ptr);
                    self.memory.free(left_ptr);
                }
                VmInstruction::Prop
                | VmInstruction::PropNoFree => {
                    let (key, key_ptr) = self.pop_stack();
                    let key = key.clone();
                    let (obj, obj_ptr) = self.pop_stack();
                    let obj = obj.clone();
                    
                    let ptr = obj.get_item(&mut self.memory, key);
                    self.push_stack_ptr(ptr);
                    
                    if let VmInstruction::Prop = inst {
                        self.memory.free(key_ptr);
                        self.memory.free(obj_ptr);
                    }
                }
                VmInstruction::Len
                | VmInstruction::LenNoFree => {
                    let (obj, obj_ptr) = self.pop_stack();
                    
                    let val = Value::Num {
                        data: obj.length() as f32
                    };
                    self.push_stack_alloc(val);
                    
                    if let VmInstruction::Len = inst {
                        self.memory.free(obj_ptr);
                    }
                }
                VmInstruction::Arrify => {
                    let (obj, obj_ptr) = self.pop_stack();
                    let obj = obj.clone();
                    
                    let items = obj.arrify(&mut self.memory);
                    for item in &items {
                        self.memory.add_ref(*item);
                    }
                    
                    let val = Value::Arr {
                        items
                    };
                    self.push_stack_alloc(val);
                    
                    self.memory.free(obj_ptr);
                }
                
                // values
                VmInstruction::Null => {
                    self.push_stack_alloc(Value::Null);
                }
                VmInstruction::Str(str) => {
                    self.push_stack_alloc(Value::Str { data: str.clone() });
                }
                VmInstruction::Num(num) => {
                    self.push_stack_alloc(Value::Num { data: *num });
                }
                VmInstruction::Percentage(num) => {
                    self.push_stack_alloc(Value::Percentage { data: *num });
                }
                VmInstruction::Bool(bool) => {
                    self.push_stack_alloc(Value::Bool { data: *bool });
                }
                VmInstruction::Func { body, args } => {
                    self.push_stack_alloc(Value::Function(Function::Vm {
                        body: body.clone(),
                        params: args.clone()
                    }));
                },
                VmInstruction::Arr { len } => {
                    let mut items = Vec::new();
                    
                    for _i in 0..*len {
                        let ptr = self.pop_stack_ptr();
                        self.memory.add_ref(ptr);
                        items.push(ptr);
                    }
                    
                    items.reverse();
                    
                    self.push_stack_alloc(Value::Arr {
                        items
                    });
                }
                VmInstruction::Obj { keys } => {
                    let mut values = Vec::new();
                    for _key in keys {
                        values.push(self.pop_stack_ptr());
                    }
                    values.reverse();
                    
                    let mut map = HashMap::new();
                    for (i, key) in keys.iter().enumerate() {
                        if let Some(already) = map.get(&key.clone()) {
                            self.memory.rm_ref(*already);
                            self.memory.free(*already);
                        }
                        let ptr = values[i];
                        self.memory.add_ref(ptr);
                        map.insert(key.clone(), ptr);
                    }
                    
                    
                    self.push_stack_alloc(Value::Obj {
                        data: map
                    });
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
                    self.scope.decl_var(&mut self.memory, name.clone(), ptr);
                    self.push_stack_ptr(ptr);
                }
                VmInstruction::NewScope => {
                    self.scope.new_layer();
                }
                VmInstruction::PopScope => {
                    let layer =self.scope.pop_layer();
                    
                    self.scope.free_layer(&mut self.memory, &layer);
                }
                
                // assignments
                VmInstruction::AsiVar(name, op) => {
                    let ptr = self.scope.get_var(name).unwrap_or(self.memory.alloc(Value::Null));
                    let (val, val_ptr) = self.pop_stack();
                    let val = val.clone();
                    match op {
                        AssignmentOp::Default => {
                            let new_val_ptr = self.memory.alloc(val);
                            self.scope.set_var(&mut self.memory, name.clone(), new_val_ptr);
                        }
                        _ => {
                            let original = self.memory.get(ptr).clone();
                            self.memory.get_cell_mut(ptr).val = self.run_binary_op(&original, &val, &op.clone().into());
                        }
                    }
                    self.push_stack_ptr(val_ptr);
                }
                VmInstruction::AsiProp(op) => {
                    let (key, key_ptr) = self.pop_stack();
                    let key = key.clone();
                    
                    let (obj, obj_ptr) = self.pop_stack();
                    
                    let index_key = match &obj {
                        Value::Obj { ..} => IndexKey::Str(key.stringify(&self.memory)),
                        _ => IndexKey::Int(key.numbify() as usize),
                    };
                    
                    let (val, val_ptr) = self.pop_stack();
                    let val = val.clone();
                    self.memory.add_ref(val_ptr);
                    
                    let obj = self.memory.get_mut(obj_ptr);
                    
                    match op {
                        AssignmentOp::Default => {
                            let old_ptr = {
                                let mut obj = self.memory.get_mut(obj_ptr);
                                obj.set_item(index_key, val_ptr)
                            };
                            
                            if let Some(ptr) = old_ptr {
                                self.memory.free(ptr);
                            }
                        }
                        _ => {
                            let ptr = {
                                let obj = self.memory.get(obj_ptr).clone();
                                obj.get_item(&mut self.memory, key.clone())
                            };
                            
                            let original = self.memory.get(ptr).clone();
                            let val = self.run_binary_op(&original, &val, &op.clone().into());
                            let val_ptr = self.memory.alloc(val);
                            
                            let old_ptr = {
                                let obj = self.memory.get_mut(obj_ptr);
                                obj.set_item(index_key, val_ptr)
                            };
                            
                            if let Some(ptr) = old_ptr {
                                self.memory.free(ptr);
                            }
                        }
                    }
                    
                    self.memory.free(key_ptr);
                    self.memory.free(obj_ptr);
                    self.push_stack_ptr(val_ptr);
                }
                
                // stack
                VmInstruction::Pop => {
                    let ptr = self.pop_stack_ptr();
                    self.memory.free(ptr);
                }
                VmInstruction::Dupe(idx) => {
                    let val = self.memory.get(self.stack[self.stack.len() - 1 - idx]);
                    let val = val.clone().dupe(&mut self.memory);
                    self.push_stack_alloc(val);
                }
                VmInstruction::DupePtr(idx) => {
                    let ptr = self.stack[self.stack.len() - 1 - idx];
                    self.push_stack_ptr(ptr);
                }
                
                _ => {
                    print_warn!(LogSource::Rtr, "couldnt run instruction '{:?}'", inst);
                }
            }
            
            i += 1;
        }
        
        Ok(None)
    }
    fn run_binary_op(&mut self, left: &Value, right: &Value, op: &BinaryOp) -> Value {
        fn get_num(left: &Value, right: &Value) -> Option<(f32, f32)> {
            if let (
                Value::Num { data: left },
                Value::Num { data: right }
            ) = (&left, &right) {
                Some((*left, *right))
            } else {
                None
            }
        }
        
        match op {
            BinaryOp::Add =>
                if let Some((left, right)) = get_num(left, right) {
                    Value::Num { data: left + right }
                } else {
                    Value::Str { data: format!("{}{}", left.stringify(&self.memory), right.stringify(&self.memory)) }
                }
            BinaryOp::Sub =>
                if let Some((left, right)) = get_num(left, right) {
                    Value::Num { data: left - right }
                } else {
                    Value::Num { data: f32::NAN }
                }
            BinaryOp::Mul =>
                if let Some((left, right)) = get_num(left, right) {
                    Value::Num { data: left * right }
                } else {
                    Value::Num { data: f32::NAN }
                }
            BinaryOp::Div =>
                if let Some((left, right)) = get_num(left, right) {
                    Value::Num { data: left / right }
                } else {
                    Value::Num { data: f32::NAN }
                }
            BinaryOp::Mod =>
                if let Some((left, right)) = get_num(left, right) {
                    Value::Num { data: left % right }
                } else {
                    Value::Num { data: f32::NAN }
                }
            BinaryOp::Pow =>
                if let Some((left, right)) = get_num(left, right) {
                    Value::Num { data: left.powf(right) }
                } else {
                    Value::Num { data: f32::NAN }
                }
            
            BinaryOp::Eql =>
                Value::Bool { data: left.equal(right) },
            BinaryOp::NotEql =>
                Value::Bool { data: !left.equal(right) },
            
            BinaryOp::Bigger =>
                Value::Bool { data: left.numbify() > right.numbify() },
            BinaryOp::BiggerEql =>
                Value::Bool { data: left.numbify() >= right.numbify() },
            BinaryOp::Smaller =>
                Value::Bool { data: left.numbify() < right.numbify() },
            BinaryOp::SmallerEql =>
                Value::Bool { data: left.numbify() <= right.numbify() },
            
            BinaryOp::NullishCoalescence =>
                match left {
                    Value::Null =>
                        right.clone(),
                    
                    _ => left.clone()
                }
            
            _ => todo!("{:?}", op)
        }
    }
    
    #[allow(clippy::unused_self)]
    pub fn compile_event(&self, statement: Vec<AstStatement>) -> Result<Vec<VmInstruction>, Error> {
        compiler::compile_statements(&mut CompileContext {
            label_count: 0
        }, statement)
    }
}

pub enum IndexKey {
    Str(String),
    Int(usize),
}