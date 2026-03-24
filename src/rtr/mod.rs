use std::collections::HashMap;
use crate::{print_raw, print_warn, Log, LogKind, print_log};
use crate::rtr::apis::inject;
use crate::rtr::ast::node::{AssignmentOp, AstProgram, AstStatement, AstTopLevelStatement, BinaryOp, EventTarget, UnaryOp};
use crate::rtr::ast::parser::Parser;
use crate::rtr::ast::tokenise;
use crate::rtr::error::Error;
use crate::rtr::log::RTRLog;
use crate::rtr::runtime::compiler;
use crate::rtr::runtime::compiler::CompileContext;
use crate::rtr::runtime::function::BuiltinFunction;
use crate::rtr::runtime::instruction::VmInstruction;
use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::scope::Scope;
use crate::rtr::runtime::value::RTRValue;
use crate::rtr::runtime::values::arr_value::RTRArr;
use crate::rtr::runtime::values::bool_value::RTRBool;
use crate::rtr::runtime::values::color_value::RTRColor;
use crate::rtr::runtime::values::function_value::{Function, RTRFunction};
use crate::rtr::runtime::values::null_value::RTRNull;
use crate::rtr::runtime::values::num_value::RTRNum;
use crate::rtr::runtime::values::obj_value::RTRObj;
use crate::rtr::runtime::values::percentage_value::RTRPercentage;
use crate::rtr::runtime::values::str_value::RTRStr;
use crate::shared::logging::LogSource;

pub mod ast;
pub mod runtime;
pub(crate) mod error;
pub mod log;
mod apis;

#[derive(Debug)]
struct Event {
    pub body: Vec<AstStatement>,
    pub target: EventTarget
}

#[derive(Debug)]
pub struct RTRModule {
    pub ast: Option<AstProgram>,
    
    // runtime
    pub stack: Vec<MemPointer>,
    pub scope: Scope,
    pub memory: Memory,
    
    pub logs: Vec<RTRLog>
}

#[derive(Debug)]
pub struct RTRInstance {

}

impl RTRModule {
    pub fn new() -> RTRModule {
        let mut module = RTRModule {
            ast: None,
            
            stack: Vec::new(),
            scope: Scope::new(),
            memory: Memory::default(),
            
            logs: Vec::new()
        };
        
        module.init();
        
        module
    }
    
    pub fn init(&mut self) {
        self.new_scope();
    }
    
    // add apis
    pub fn inject(&mut self) {
        inject(&mut self.memory, &mut self.scope);
    }
    
    fn set_alloc(&mut self, name: &str, value: Box<dyn RTRValue>) {
        let ptr = self.memory.alloc(value);
        self.scope.set_var(
            &mut self.memory,
            String::from(name),
            ptr
        );
    }
    pub fn new_scope(&mut self) {
        self.set_alloc("true", Box::new(RTRBool { data: true }));
        self.set_alloc("false", Box::new(RTRBool { data: false }));
        self.set_alloc("null", Box::new(RTRNull));
        
        self.set_alloc("log", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Log)
        }));
        self.set_alloc("error", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Error)
        }));
        self.set_alloc("return", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Return)
        }));
        self.set_alloc("typeof", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Typeof)
        }));
        self.set_alloc("length", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Length)
        }));
        
        // mathematical
        self.set_alloc("min", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Min)
        }));
        self.set_alloc("max", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Max)
        }));
        
        self.set_alloc("abs", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Abs)
        }));
        self.set_alloc("sqrt", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Sqrt)
        }));
        
        self.set_alloc("round", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Round)
        }));
        self.set_alloc("floor", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Floor)
        }));
        self.set_alloc("ceil", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Ceil)
        }));
        
        // string
        self.set_alloc("join", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Join)
        }));
        self.set_alloc("split", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Split)
        }));
        
        self.set_alloc("chr", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Chr)
        }));
        self.set_alloc("ord", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Ord)
        }));
        
        self.set_alloc("toUpper", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::ToUpper)
        }));
        self.set_alloc("toLower", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::ToLower)
        }));
        self.set_alloc("toTitle", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::ToTitle)
        }));
        
        // array
        self.set_alloc("item", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Item)
        }));
        self.set_alloc("range", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Range)
        }));
        
        // object
        self.set_alloc("keys", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Keys)
        }));
        self.set_alloc("values", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Values)
        }));
        self.set_alloc("has", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Has)
        }));
        self.set_alloc("obj", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Obj)
        }));
        
        // logical
        self.set_alloc("all", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::All)
        }));
        self.set_alloc("any", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Any)
        }));
        self.set_alloc("not", Box::new(RTRFunction {
            data: Function::Builtin(BuiltinFunction::Not)
        }));
    }
    
    pub fn parse(&mut self, src: &str) -> Result<(), Error> {
        let tokens = tokenise(src);
        let mut parser = Parser {
            pointer: 0,
            tokens
        };
        
        let out = parser.parse();
        match out {
            Err(err) => {
                print_warn!(LogSource::Rtr, "{err}");
                return Err(err);
            }
            Ok(ast) => {
                self.ast = Some(ast);
            }
        }
        
        Ok(())
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
        let val = self.run_instructions(&instructions)?;
        
        let val = val.map(|ptr| self.memory.get(ptr));
        
        Ok(())
    }
    
    fn pop_stack(&mut self) -> (&dyn RTRValue, MemPointer) {
        let ptr = self.pop_stack_raw().unwrap();
        (self.memory.get(ptr), ptr)
    }
    fn pop_stack_mut(&mut self) -> (&mut dyn RTRValue, MemPointer) {
        let ptr = self.pop_stack_raw().unwrap();
        (self.memory.get_mut(ptr), ptr)
    }
    fn pop_stack_ptr(&mut self) -> MemPointer {
        self.pop_stack_raw().unwrap()
    }
    fn pop_stack_raw(&mut self) -> Option<MemPointer> {
        self.stack.pop()
    }
    
    fn push_stack_alloc(&mut self, val: Box<dyn RTRValue>) {
        let ptr = self.memory.alloc(val);
        self.push_stack_ptr(ptr);
    }
    fn push_stack_ptr(&mut self, val: MemPointer) {
        self.stack.push(val);
    }
    
    #[allow(clippy::too_many_lines)]
    pub fn run_instructions(&mut self, instructions: &[VmInstruction]) -> Result<Option<MemPointer>, Error> {
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
        
        let start_scope_layers = self.scope.layers.iter().len();
        
        let mut i = 0;
        let mut instructions_ran = 0;
        while i < instructions.len() && instructions_ran < 10000 {
            instructions_ran += 1;
            let inst = &instructions[i];
            
            if false {
                println!("{:?}\n    {:?}\n    {:?}", inst, self.stack.iter().map(|ptr| {
                    let cell = self.memory.get_cell_option(*ptr);
                    if let Some(cell) = cell {
                        format!("{:?}", cell.val)
                    } else {
                        String::from("EMPTY")
                    }
                }).collect::<Vec<_>>(), self.stack);
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
                        let func = self.memory.get_mut(func_ptr).clone_box();
                        match func.to_function() {
                            Some(RTRFunction { data: Function::Vm { body, params } }) => {
                                self.scope.new_layer();
                                for (i, param) in params.iter().enumerate() {
                                    let arg = *args.get(i).unwrap_or(&self.memory.alloc(Box::new(RTRNull)));
                                    self.scope.decl_var(&mut self.memory, param.name.clone(), arg);
                                }
                                self.run_instructions(&body)?.unwrap_or(self.memory.alloc(Box::new(RTRNull)))
                            }
                            Some(RTRFunction { data: Function::Builtin(BuiltinFunction::Return) }) => {
                                let ptr = args.first().copied();
                                
                                // add ref to stop from being freed
                                if let Some(ptr) = &ptr {
                                    self.memory.add_ref(*ptr);
                                }
                                
                                // free scope layers
                                for _ in start_scope_layers..self.scope.layers.len() {
                                    let layer = &self.scope.pop_layer();
                                    self.scope.free_layer(&mut self.memory, layer);
                                }
                                
                                // rm ref
                                if let Some(ptr) = &ptr {
                                    self.memory.rm_ref(*ptr);
                                }
                                
                                // free any spare arguments
                                for arg in &args[1..] {
                                    self.memory.free(*arg);
                                }
                                
                                return Ok(ptr);
                            }
                            _ => {
                                func.call(&mut self.logs, &mut self.memory, &args)?
                            }
                        }
                    };
                    
                    self.stack.push(out_ptr);
                    
                    self.memory.rm_ref(func_ptr);
                    self.memory.free(func_ptr);
                    for arg in args {
                        self.memory.free(arg);
                    }
                }
                VmInstruction::CallEv(name) => {
                    self.run_event_target(&EventTarget::Global { name: name.clone() })?;
                }
                VmInstruction::Unary(op) => {
                    let (right, right_ptr) = self.pop_stack();
                    
                    let right = right.clone_box();
                    
                    self.push_stack_alloc(match op {
                        UnaryOp::Minus =>
                            Box::new(RTRNum { data: -right.numbify() }),
                        UnaryOp::Number =>
                            Box::new(RTRNum { data: right.numbify() }),
                        
                        UnaryOp::Invert =>
                            Box::new(RTRBool { data: !right.boolify() }),
                        UnaryOp::Boolify =>
                            Box::new(RTRBool { data: right.boolify() })
                    });
                    
                    self.memory.free(right_ptr);
                }
                VmInstruction::Binary(op) => {
                    let (right, right_ptr) = self.pop_stack();
                    let right = right.clone_box();
                    let (left, left_ptr) = self.pop_stack();
                    let left = left.clone_box();
                    
                    let val = self.run_binary_op(&*left, &*right, op);
                    self.push_stack_alloc(val);
                    
                    self.memory.free(right_ptr);
                    self.memory.free(left_ptr);
                }
                VmInstruction::Prop
                | VmInstruction::PropNoFree => {
                    let (key, key_ptr) = self.pop_stack();
                    let key = key.clone_box();
                    let (obj, obj_ptr) = self.pop_stack();
                    let obj = obj.clone_box();
                    
                    let ptr = obj.get_item(&mut self.memory, &*key);
                    self.push_stack_ptr(ptr);
                    
                    if let VmInstruction::Prop = inst {
                        self.memory.free(key_ptr);
                        self.memory.free(obj_ptr);
                    }
                }
                VmInstruction::Len
                | VmInstruction::LenNoFree => {
                    let (obj, obj_ptr) = self.pop_stack();
                    let obj = obj.clone_box();
                    
                    let val = RTRNum {
                        data: obj.length(&self.memory) as f32
                    };
                    self.push_stack_alloc(Box::new(val));
                    
                    if let VmInstruction::Len = inst {
                        self.memory.free(obj_ptr);
                    }
                }
                VmInstruction::Arrify => {
                    let (obj, obj_ptr) = self.pop_stack();
                    let obj = obj.clone_box();
                    
                    let items = obj.arrify(&mut self.memory);
                    for item in &items {
                        self.memory.add_ref(*item);
                    }
                    
                    let val = RTRArr {
                        items
                    };
                    self.push_stack_alloc(Box::new(val));
                    
                    self.memory.free(obj_ptr);
                }
                
                // values
                VmInstruction::Null => {
                    self.push_stack_alloc(Box::new(RTRNull));
                }
                VmInstruction::Str(str) => {
                    self.push_stack_alloc(Box::new(RTRStr { data: str.clone() }));
                }
                VmInstruction::Num(num) => {
                    self.push_stack_alloc(Box::new(RTRNum { data: *num }));
                }
                VmInstruction::Percentage(num) => {
                    self.push_stack_alloc(Box::new(RTRPercentage { data: *num }));
                }
                VmInstruction::Bool(bool) => {
                    self.push_stack_alloc(Box::new(RTRBool { data: *bool }));
                }
                VmInstruction::Func { body, args } => {
                    self.push_stack_alloc(Box::new(RTRFunction {
                        data: Function::Vm {
                            body: body.clone(),
                            params: args.clone()
                        }
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
                    
                    self.push_stack_alloc(Box::new(RTRArr {
                        items
                    }));
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
                    
                    
                    self.push_stack_alloc(Box::new(RTRObj {
                        data: map
                    }));
                }
                VmInstruction::Color(color) => {
                    self.push_stack_alloc(Box::new(RTRColor {
                        data: *color
                    }));
                }
                
                // scope
                VmInstruction::Get(name) => {
                    let var = self.scope.get_var(name);
                    if let Some(ptr) = var {
                        self.push_stack_ptr(ptr);
                    } else {
                        self.push_stack_alloc(Box::new(RTRNull));
                    }
                }
                VmInstruction::Decl(name) => {
                    let ptr = self.pop_stack_ptr();
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
                    let ptr = self.scope.get_var(name).unwrap_or(self.memory.alloc(Box::new(RTRNull)));
                    let (val, val_ptr) = self.pop_stack();
                    let val = val.clone_box();
                    if let AssignmentOp::Default = op {
                        let new_val_ptr = self.memory.alloc(val);
                        self.scope.set_var(&mut self.memory, name.clone(), new_val_ptr);
                    } else {
                        let original = self.memory.get(ptr).clone_box();
                        self.memory.get_cell_mut(ptr).val = self.run_binary_op(&*original, &*val, &op.clone().into());
                    }
                    self.push_stack_ptr(val_ptr);
                }
                VmInstruction::AsiProp(op) => {
                    let (key, key_ptr) = self.pop_stack();
                    let key = key.clone_box();
                    
                    let (obj, obj_ptr) = self.pop_stack();
                    
                    let index_key = match obj.to_obj() {
                        Some(RTRObj { .. }) => IndexKey::Str(key.stringify(&self.memory)),
                        _ => IndexKey::Int(key.numbify() as usize),
                    };
                    
                    let (val, val_ptr) = self.pop_stack();
                    let val = val.clone_box();
                    self.memory.add_ref(val_ptr);
                    
                    //let _obj = self.memory.get_mut(obj_ptr);
                    
                    if let AssignmentOp::Default = op {
                        let old_ptr = {
                            let mut obj = self.memory.get_mut(obj_ptr);
                            obj.set_item(index_key, val_ptr)
                        };
                        
                        if let Some(ptr) = old_ptr {
                            self.memory.free(ptr);
                        }
                    } else {
                        let ptr = {
                            let obj = self.memory.get(obj_ptr).clone_box();
                            obj.get_item(&mut self.memory, &*key)
                        };
                        
                        let original = self.memory.get(ptr).clone_box();
                        let val = self.run_binary_op(&*original, &*val, &op.clone().into());
                        let val_ptr = self.memory.alloc(val);
                        
                        let old_ptr = {
                            let obj = self.memory.get_mut(obj_ptr);
                            obj.set_item(index_key, val_ptr)
                        };
                        
                        if let Some(ptr) = old_ptr {
                            self.memory.free(ptr);
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
                    let val = val.clone_box().dupe(&mut self.memory);
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
    fn run_binary_op(&mut self, left: &dyn RTRValue, right: &dyn RTRValue, op: &BinaryOp) -> Box<dyn RTRValue> {
        fn get_num(left: &dyn RTRValue, right: &dyn RTRValue) -> Option<(f32, f32)> {
            if let (
                Some(RTRNum { data: left }),
                Some(RTRNum { data: right })
            ) = (left.to_num(), right.to_num()) {
                Some((left, right))
            } else {
                None
            }
        }
        
        match op {
            BinaryOp::Add =>
                if let Some((left, right)) = get_num(left, right) {
                    Box::new(RTRNum { data: left + right })
                } else {
                    Box::new(RTRStr { data: format!("{}{}", left.stringify(&self.memory), right.stringify(&self.memory)) })
                }
            BinaryOp::Sub =>
                if let Some((left, right)) = get_num(left, right) {
                    Box::new(RTRNum { data: left - right })
                } else {
                    Box::new(RTRNum { data: f32::NAN })
                }
            BinaryOp::Mul =>
                if let Some((left, right)) = get_num(left, right) {
                    Box::new(RTRNum { data: left * right })
                } else {
                    Box::new(RTRNum { data: f32::NAN })
                }
            BinaryOp::Div =>
                if let Some((left, right)) = get_num(left, right) {
                    Box::new(RTRNum { data: left / right })
                } else {
                    Box::new(RTRNum { data: f32::NAN })
                }
            BinaryOp::Mod =>
                if let Some((left, right)) = get_num(left, right) {
                    Box::new(RTRNum { data: left % right })
                } else {
                    Box::new(RTRNum { data: f32::NAN })
                }
            BinaryOp::Pow =>
                if let Some((left, right)) = get_num(left, right) {
                    Box::new(RTRNum { data: left.powf(right) })
                } else {
                    Box::new(RTRNum { data: f32::NAN })
                }
            
            BinaryOp::Eql =>
                Box::new(RTRBool { data: left.equal(right) }),
            BinaryOp::NotEql =>
                Box::new(RTRBool { data: !left.equal(right) }),
            
            BinaryOp::Bigger =>
                Box::new(RTRBool { data: left.numbify() > right.numbify() }),
            BinaryOp::BiggerEql =>
                Box::new(RTRBool { data: left.numbify() >= right.numbify() }),
            BinaryOp::Smaller =>
                Box::new(RTRBool { data: left.numbify() < right.numbify() }),
            BinaryOp::SmallerEql =>
                Box::new(RTRBool { data: left.numbify() <= right.numbify() }),
            
            BinaryOp::NullishCoalescence =>
                if left.is_null() {
                    right.clone_box()
                } else {
                    left.clone_box()
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