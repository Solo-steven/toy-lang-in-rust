pub mod lexer;
pub mod parser;
mod codegen;
mod ast;
mod token;
mod utils;


use std::fmt::Display;
use inkwell::context::Context;
use parser::Parser;
use codegen::Codegen;

#[derive(Debug, PartialEq)]
pub struct ExecuteResult {
    pub llvm_code: Option<String>,
    pub return_value: f64
}

pub fn execute_program(code_string: String, emit_llvm: bool) -> ExecuteResult{
    let mut parser = Parser::new(code_string);
    let program_ast = parser.parse();
    let llvm_context = Context::create();
    let mut code_generator = Codegen::new(&llvm_context, &program_ast);
    code_generator.generate();
    let execute_return_value = code_generator.execute();
    let llvm_code_string = code_generator.get_llvm_code_as_string();

    if emit_llvm {
        return  ExecuteResult{
            llvm_code: Some(llvm_code_string),
            return_value: execute_return_value,
        }
    }
    return ExecuteResult{
        llvm_code: None,
        return_value: execute_return_value,
    }
}