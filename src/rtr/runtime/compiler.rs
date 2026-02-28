use crate::{Log, LogKind, print_log};
use crate::rtr::ast::node::{AstExpression, AstStatement};
use crate::rtr::error::Error;
use crate::rtr::runtime::instruction::VmInstruction;

pub fn compile_statements(statements: Vec<AstStatement>) -> Result<Vec<VmInstruction>, Error> {
    let mut instructions = Vec::new();
    
    for statement in statements {
        instructions.append(compile_statement(statement)?.as_mut());
    }
    
    Ok(instructions)
}
pub fn compile_statement(statement: AstStatement) -> Result<Vec<VmInstruction>, Error> {
    let mut instructions = Vec::new();
    
    match statement {
        AstStatement::Expression(expr) => {
            instructions.append(compile_expression(expr)?.as_mut());
            instructions.push(VmInstruction::Pop);
        }
    }
    
    Ok(instructions)
}

pub fn compile_expressions(expressions: Vec<AstExpression>) -> Result<Vec<VmInstruction>, Error> {
    let mut instructions = Vec::new();
    
    for expression in expressions {
        instructions.append(compile_expression(expression)?.as_mut());
    }
    
    Ok(instructions)
}
pub fn compile_expression(expression: AstExpression) -> Result<Vec<VmInstruction>, Error> {
    let mut instructions = Vec::new();
    
    match expression {
        // operations
        AstExpression::Call { args, func, .. } => {
            let len = args.len();
            instructions.append(compile_expression(*func)?.as_mut());
            instructions.append(compile_expressions(args)?.as_mut());
            instructions.push(VmInstruction::Call(len));
        }
        AstExpression::Declare { name, value, .. } => {
            instructions.append(compile_expression(*value)?.as_mut());
            instructions.push(VmInstruction::Decl(name));
        }
        AstExpression::Unary { value, op, .. } => {
            instructions.append(compile_expression(*value)?.as_mut());
            instructions.push(VmInstruction::Unary(op));
        }
        AstExpression::Binary { left, right, op, .. } => {
            instructions.append(compile_expression(*left)?.as_mut());
            instructions.append(compile_expression(*right)?.as_mut());
            instructions.push(VmInstruction::Binary(op));
        }
        
        // accessing
        AstExpression::Variable { name, .. } => {
            instructions.push(VmInstruction::Get(name));
        },
        
        // values
        AstExpression::String { content, .. } => {
            instructions.push(VmInstruction::Str(content));
        }
        AstExpression::Number { content, .. } => {
            instructions.push(VmInstruction::Num(content));
        }
        AstExpression::Percentage { content, .. } => {
            instructions.push(VmInstruction::Percentage(content));
        }
    }
    
    Ok(instructions)
}