use crate::{Log, LogKind, print_log};
use crate::rtr::ast::node::{AstConditionalType, AstExpression, AstStatement, BinaryOp, Parameter, PropertyKey};
use crate::rtr::error::Error;
use crate::rtr::error::Error::CannotAssign;
use crate::rtr::runtime::instruction::VmInstruction;
use crate::rtr::runtime::instruction::VmInstruction::AsiVar;

pub struct CompileContext {
    pub label_count: usize,
}

pub fn get_label_name(ctx: &mut CompileContext) -> String {
    ctx.label_count += 1;
    ctx.label_count.to_string()
}

pub fn compile_statements(ctx: &mut CompileContext, statements: Vec<AstStatement>) -> Result<Vec<VmInstruction>, Error> {
    let mut instructions = Vec::new();
    
    for statement in statements {
        instructions.append(compile_statement(ctx, statement)?.as_mut());
    }
    
    Ok(instructions)
}
pub fn compile_statement(ctx: &mut CompileContext, statement: AstStatement) -> Result<Vec<VmInstruction>, Error> {
    let mut instructions = Vec::new();
    
    match statement {
        AstStatement::Block { body } => {
            instructions.push(VmInstruction::NewScope);
            instructions.append(compile_statements(ctx, body)?.as_mut());
            instructions.push(VmInstruction::PopScope);
        }
        AstStatement::Branch { cond, body, elifs, else_body, .. } => {
            let has_else = !elifs.is_empty() || else_body.is_some();
            
            let end_lbl = get_label_name(ctx);
            let mut else_lbl = if has_else { get_label_name(ctx) } else { end_lbl.clone() };
            
            instructions.append(compile_expression(ctx, cond)?.as_mut());
            instructions.push(VmInstruction::JumpNotIf(else_lbl.clone()));
            instructions.append(compile_statement(ctx, *body)?.as_mut());
            if has_else {
                instructions.push(VmInstruction::Jump(end_lbl.clone()));
            }
            
            for (i, elif) in elifs.iter().enumerate() {
                let is_last = i == elifs.len() && else_body.is_none();
                
                instructions.push(VmInstruction::Label(else_lbl.clone()));
                else_lbl = if is_last { end_lbl.clone() } else { get_label_name(ctx) };
                
                instructions.append(compile_expression(ctx, elif.0.clone())?.as_mut());
                instructions.push(VmInstruction::JumpNotIf(else_lbl.clone()));
                instructions.append(compile_statement(ctx, elif.1.clone())?.as_mut());
                
                if !is_last {
                    instructions.push(VmInstruction::Jump(end_lbl.clone()));
                }
            }
            
            if let Some(body) = else_body {
                instructions.push(VmInstruction::Label(else_lbl));
                instructions.append(compile_statement(ctx, *body)?.as_mut());
            }
            
            instructions.push(VmInstruction::Label(end_lbl));
        }
        AstStatement::ConditionalLoop { kind, cond, body, .. } => {
            let start_lbl = get_label_name(ctx);
            let end_lbl = get_label_name(ctx);
            
            instructions.push(VmInstruction::Label(start_lbl.clone()));
            instructions.append(compile_expression(ctx, cond)?.as_mut());
            instructions.push(match kind {
                AstConditionalType::While => VmInstruction::JumpNotIf(end_lbl.clone()),
                AstConditionalType::Until => VmInstruction::JumpIf(end_lbl.clone())
            });
            instructions.append(compile_statement(ctx, *body)?.as_mut());
            instructions.push(VmInstruction::Jump(start_lbl));
            instructions.push(VmInstruction::Label(end_lbl));
        }
        AstStatement::Repeat { amount, body, .. } => {
            let start_lbl = get_label_name(ctx);
            let end_lbl = get_label_name(ctx);
            
            instructions.push(VmInstruction::Num(1.0));
            instructions.push(VmInstruction::Label(start_lbl.clone()));
            instructions.push(VmInstruction::Dupe(0));
            instructions.append(compile_expression(ctx, amount)?.as_mut());
            instructions.push(VmInstruction::Binary(BinaryOp::Bigger));
            instructions.push(VmInstruction::JumpIf(end_lbl.clone()));
            instructions.append(compile_statement(ctx, *body)?.as_mut());
            instructions.push(VmInstruction::Num(1.0));
            instructions.push(VmInstruction::Binary(BinaryOp::Add));
            instructions.push(VmInstruction::Jump(start_lbl));
            instructions.push(VmInstruction::Label(end_lbl));
            instructions.push(VmInstruction::Pop);
        }
        AstStatement::For { var, iterator, body, .. } => {
            let start_lbl = get_label_name(ctx);
            let end_lbl = get_label_name(ctx);
            
            instructions.append(compile_expression(ctx, iterator)?.as_mut());
            instructions.push(VmInstruction::Arrify);
            instructions.push(VmInstruction::Num(0.0));                 // index
            instructions.push(VmInstruction::Label(start_lbl.clone()));
            instructions.push(VmInstruction::DupePtr(1));               // array.clone()
            instructions.push(VmInstruction::LenNoFree);                // array.len (no free)
            instructions.push(VmInstruction::Dupe(1));                  // index.clone()
            instructions.push(VmInstruction::Binary(BinaryOp::SmallerEql)); // index < array.len
            instructions.push(VmInstruction::JumpIf(end_lbl.clone()));  // jump if index < len
            instructions.push(VmInstruction::DupePtr(1));               // array.clone()
            instructions.push(VmInstruction::DupePtr(1));               // index.clone()
            instructions.push(VmInstruction::PropNoFree);               // array[index]
            instructions.push(VmInstruction::NewScope);                 // {
            instructions.push(VmInstruction::Decl(var));                // var = array[index]
            instructions.push(VmInstruction::Pop);                      // pop output
            instructions.append(compile_statement(ctx, *body)?.as_mut()); // <body>
            instructions.push(VmInstruction::PopScope);                 // }
            instructions.push(VmInstruction::Num(1.0));                 // 1
            instructions.push(VmInstruction::Binary(BinaryOp::Add));    // index + 1
            instructions.push(VmInstruction::Jump(start_lbl));          // jump back
            instructions.push(VmInstruction::Label(end_lbl));
            instructions.push(VmInstruction::Pop);
            instructions.push(VmInstruction::Pop);
        }
        
        AstStatement::Expression(expr) => {
            instructions.append(compile_expression(ctx, expr)?.as_mut());
            instructions.push(VmInstruction::Pop);
        }
    }
    
    Ok(instructions)
}

pub fn compile_expressions(ctx: &mut CompileContext, expressions: Vec<AstExpression>) -> Result<Vec<VmInstruction>, Error> {
    let mut instructions = Vec::new();
    
    for expression in expressions {
        instructions.append(compile_expression(ctx, expression)?.as_mut());
    }
    
    Ok(instructions)
}
pub fn compile_expression(ctx: &mut CompileContext, expression: AstExpression) -> Result<Vec<VmInstruction>, Error> {
    let mut instructions = Vec::new();
    
    match expression {
        // operations
        AstExpression::Call { args, func, .. } => {
            let len = args.len();
            instructions.append(compile_expression(ctx, *func)?.as_mut());
            instructions.append(compile_expressions(ctx, args)?.as_mut());
            instructions.push(VmInstruction::Call(len));
        }
        AstExpression::Declare { name, value, .. } => {
            instructions.append(compile_expression(ctx, *value)?.as_mut());
            instructions.push(VmInstruction::Decl(name));
        }
        AstExpression::Assignment { op, target, value, .. } => {
            instructions.append(compile_expression(ctx, *value)?.as_mut());
            
            match *target {
                AstExpression::Variable { name, .. } => {
                    instructions.push(AsiVar(name, op));
                }
                AstExpression::Property { obj, key, .. } => {
                    instructions.append(compile_expression(ctx, *obj)?.as_mut());
                    match key {
                        PropertyKey::Str(name) => {
                            instructions.push(VmInstruction::Str(name));
                        }
                        PropertyKey::Expr(expr) => {
                            instructions.append(compile_expression(ctx, *expr)?.as_mut());
                        }
                    }
                    instructions.push(VmInstruction::AsiProp(op));
                }
                
                _ => {
                    return Err(CannotAssign {
                        to: String::from("non-variable or property")
                    })
                }
            }
        }
        AstExpression::Unary { value, op, .. } => {
            instructions.append(compile_expression(ctx, *value)?.as_mut());
            instructions.push(VmInstruction::Unary(op));
        }
        AstExpression::Binary { left, right, op, .. } => {
            instructions.append(compile_expression(ctx, *left)?.as_mut());
            instructions.append(compile_expression(ctx, *right)?.as_mut());
            instructions.push(VmInstruction::Binary(op));
        }
        AstExpression::Property { obj, key, .. } => {
            instructions.append(compile_expression(ctx, *obj)?.as_mut());
            match key {
                PropertyKey::Str(name) => {
                    instructions.push(VmInstruction::Str(name));
                }
                PropertyKey::Expr(expr) => {
                    instructions.append(compile_expression(ctx, *expr)?.as_mut());
                }
            }
            instructions.push(VmInstruction::Prop);
        }
        
        // accessing
        AstExpression::Variable { name, .. } => {
            instructions.push(VmInstruction::Get(name));
        },
        
        // values
        AstExpression::Array { items, .. } => {
            let len = items.len();
            instructions.append(compile_expressions(ctx, items)?.as_mut());
            instructions.push(VmInstruction::Arr {
                len
            });
        }
        AstExpression::Object { pairs, .. } => {
            let mut keys = Vec::new();
            for pair in pairs {
                keys.push(pair.0);
                instructions.append(compile_expression(ctx, pair.1)?.as_mut());
            }
            instructions.push(VmInstruction::Obj {
                keys
            });
        }
        AstExpression::String { content, .. } => {
            instructions.push(VmInstruction::Str(content));
        }
        AstExpression::Number { content, .. } => {
            instructions.push(VmInstruction::Num(content));
        }
        AstExpression::Percentage { content, .. } => {
            instructions.push(VmInstruction::Percentage(content));
        }
        AstExpression::Func { params, body, .. } => {
            instructions.push(VmInstruction::Func {
                body: compile_statement(ctx, *body)?,
                args: params
                    .iter()
                    .map(|name| Parameter { name: name.clone() })
                    .collect()
            });
        }
    }
    
    Ok(instructions)
}