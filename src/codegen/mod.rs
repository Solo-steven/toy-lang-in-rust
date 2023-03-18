use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::basic_block::BasicBlock;
use inkwell::values::{PointerValue, FloatValue, BasicValueEnum,BasicMetadataValueEnum};
use inkwell::types::{BasicMetadataTypeEnum};
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

use crate::ast::*;
use std::collections::HashMap;

pub struct Codegen<'ctx> {
    program: &'ctx Program,
    context:&'ctx Context,
    module: Module<'ctx>,
    current_block: Option<BasicBlock<'ctx>>,
    symbol_table: HashMap<String, PointerValue<'ctx>>
}

enum ExprResult<'ctx> {
    BasicEnum(BasicValueEnum<'ctx>), // return by load inst.
    Float(FloatValue<'ctx>),        // return
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, program: &'ctx Program ) ->  Codegen<'ctx> {
        Codegen {
            program,
            context,
            module: context.create_module("module"),
            current_block: None,
            symbol_table: HashMap::<String, PointerValue>::new()
        }
    }
    pub fn print_to_stderr(&self) {
        self.module.print_to_stderr();
    }
    pub fn generate(&mut self) {
        for program_item in &self.program.body {
            self.accecpt_program_item(program_item);
        };
    }
    pub fn execute(&mut self) {
        let engine = self.module.create_jit_execution_engine(OptimizationLevel::Default).ok().unwrap();
        let main_func: JitFunction<unsafe extern "C" fn() -> f64> = unsafe { engine.get_function("main").unwrap() };
        unsafe {
            println!("{:?}",main_func.call());
        }
    }
    fn accecpt_program_item(&mut self, program_item: &ProgramItem)  {
        match *program_item {
            ProgramItem::Stmt(ref statement) => {
                self.accecpt_statement(statement);
            }
            ProgramItem::Decl(ref delcaration) => {
                self.accecpt_declaration(delcaration);
            }
            ProgramItem::Expr(ref expression) => {
                self.accecpt_expression(expression);
            }
        };
    }
    fn accecpt_declaration(&mut self, declaration: &Decl) {
        match *declaration {
            Decl::VariableDecl(ref variable_declaration) => {
                let name = variable_declaration.name.clone();
                let llvm_basic_block = self.current_block.as_ref().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(*llvm_basic_block);
                let llvm_value = builder.build_alloca(self.context.f64_type(), name.as_str());
                builder.build_store(
                    llvm_value,
                    match variable_declaration.init {
                        Some(ref expr_ref) => {
                           match self.accecpt_expression(expr_ref) {
                                ExprResult::Float(float_value) => {
                                    float_value
                                }
                                ExprResult::BasicEnum(basic_value) => {
                                    basic_value.into_float_value()
                                } 
                           }
                        }
                        None => {
                             self.context.f64_type().const_float(0.)
                        }
                    } 
                );
            }
            Decl::FunctionDecl(ref function_declaration) => {
                /*
                   Create Function Signature
                    -> 1. set up return type
                    -> 2. set up params type
                    -> 3. set up function name
                 */
                let fun_name = function_declaration.name.clone();
                let fun_return_type = match function_declaration.return_type {
                        Type::Number => {
                            self.context.f64_type()
                        }
                        _ => {
                            panic!("");
                        }
                };
                let mut params_type:Vec::<BasicMetadataTypeEnum> = vec![];
                for _i in 0..function_declaration.arguments.len() {
                    params_type.push(BasicMetadataTypeEnum::FloatType(self.context.f64_type()))
                }
                let llvm_fun_type = fun_return_type.fn_type(
                    params_type.as_ref(), 
                    false
                );
                let llvm_fun_value = self.module.add_function(fun_name.as_str(), llvm_fun_type, None);
                /*
                  Create Entry Block's Load instruction
                    ->  1. create entry local for allocate local variable.
                    ->  2. allocate local variable.
                    ->  3. Load every local register by load instruction
                           insert into symbol table.
                 */
                let entry_block_of_function = self.context.append_basic_block(llvm_fun_value,"entry");
                let builder = self.context.create_builder();
                builder.position_at_end(entry_block_of_function);
                for i in 0..function_declaration.arguments.len() {
                    let argument = function_declaration.arguments[i].clone();
                    let local_pointer_value = builder.build_alloca(self.context.f64_type(),&argument.clone().as_str());
                    builder.build_store(
                        local_pointer_value, 
                        entry_block_of_function.get_parent().unwrap().get_nth_param(i as u32).unwrap()
                    );
                    self.symbol_table.insert(argument.clone(), local_pointer_value);
                }
                
                self.current_block = Some(entry_block_of_function);
                self.accecpt_statement(&function_declaration.body);
                self.current_block = None;
            }
        }
    }
    fn accecpt_statement(&mut self, statement: &Stmt) {
        match *statement {
            Stmt::BlockStmt(ref block_statement) => {
                for program_item in &block_statement.body {
                    self.accecpt_program_item(program_item)
                }
            }
            Stmt::ReturnStmt(ref return_statement) => {
                let llvm_basic_block = self.current_block.as_ref().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(*llvm_basic_block);
                let llvm_value = self.accecpt_expression(&return_statement.argument);
                match llvm_value {
                    ExprResult::Float(float_value) => {
                        builder.build_return(Some(&float_value));
                    }
                    ExprResult::BasicEnum(basic_value) => {
                        builder.build_return(Some(&basic_value));
                    }
                    _ => {
                        panic!()
                    }
                }
            }
            _ => {
                panic!("")
            }
        }
    }
    fn accecpt_expression(&self, expression: &Expr) -> ExprResult<'ctx> {
        match *expression {
            Expr::SequnceExpr(ref sequnce_expr) => {
                panic!()
            }
            Expr::AssigmentExpr(ref assignment_expr) => {
                self.accecpt_assigment_expression(assignment_expr)
            }
            Expr::ConditionExpr(ref conditional_expr) => {
                panic!()
            }
            Expr::BinaryExpr(ref binary_expr) => {
                self.accecpt_binary_expression(binary_expr)
            }
            Expr::UnaryExpr(ref unary_expr) => {
                panic!()
            }
            Expr::NumberExpr(ref number_literal) => {
                // return a llvm float value
                ExprResult::Float(self.context.f64_type().const_float(number_literal.value))
            }
            Expr::Ident(ref identifier) => {
                /*
                   load pointer value from symbol table
                 */
                let llvm_basic_block = self.current_block.as_ref().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(*llvm_basic_block);
                let option_symbol = self.symbol_table.get(identifier.name.as_str());
                match option_symbol {
                    Some(llvm_pointer_value) => {
                        let llvm_load_result = builder.build_load(*llvm_pointer_value, identifier.name.as_str());
                        ExprResult::BasicEnum(llvm_load_result)
                    }
                    None => {
                        panic!()
                    }
                }
            }
            Expr::CallExpr(ref call_expr) => {
                /*
                
                 */
                match self.module.get_function(call_expr.callee_name.as_str()){
                    Some(llvm_funtion_value) => {
                        let mut llvm_params_value = Vec::<BasicMetadataValueEnum>::new();
                        for param in &call_expr.params {
                            llvm_params_value.push(
                                match self.accecpt_expression(param) {
                                    ExprResult::Float(float_value) => {
                                        float_value.into()
                                    }
                                    ExprResult::BasicEnum(basic_value) => {
                                        basic_value.into()
                                    }
                                }
                            )
                        }                    
                        let llvm_basic_block = self.current_block.as_ref().unwrap();
                        let builder = self.context.create_builder();
                        builder.position_at_end(*llvm_basic_block);
                        
                        ExprResult::BasicEnum(
                            builder.build_call(
                                    llvm_funtion_value, 
                                    llvm_params_value.as_slice(), 
                                    "tmpCall"
                                )
                                .try_as_basic_value()
                                .left().unwrap()
                        )
                    }
                    None => {
                        panic!();
                    }
                }
            }
        }
    }
    fn accecpt_assigment_expression(&self, assignment_expr: &AssigmentExpression) -> ExprResult<'ctx> {
        /*
            Codegen store command to assign right hand side to left hand side
            -> 1. check left hand side llvm_value should be assignable.
            -> 2. build store inst to assign right hand side value to left hand side
        */
        self.accecpt_expression(assignment_expr.left.as_ref());
        let lhs_symbol =  match *assignment_expr.left.as_ref() {
            Expr::Ident(ref ident) => {
                // get pointer value from symbol_table
                self.symbol_table.get(ident.name.as_str()).unwrap()
            }
            _ => {
                panic!("[]");
            }
        };
        let rhs = match self.accecpt_expression(&assignment_expr.right.as_ref()) {
            ExprResult::BasicEnum(basic_value) => basic_value.into_float_value(),
            ExprResult::Float(float_value) => float_value,
            _ => panic!()
        };
        let llvm_basic_block = self.current_block.as_ref().unwrap();
        let builder = self.context.create_builder();
        builder.position_at_end(*llvm_basic_block);
        builder.build_store(*lhs_symbol, rhs);
        ExprResult::Float(rhs)
    }
    fn accecpt_binary_expression(&self, binary_expr: &BinaryExpression) -> ExprResult<'ctx>  {
                /*
                    Codegen Basic On Left-hand-side and Right-hand-side
                     -> 1. get llvm_value from left and right hand side.
                     -> 2. build the numeric inst based on operator.
                     -> 3. return llvm_value based on  numeric inst.
                 */
                let lhs_llvm_value = match self.accecpt_expression(binary_expr.left.as_ref()) {
                    ExprResult::Float(float_value) => {
                        float_value
                    }
                    ExprResult::BasicEnum(basic_enum) => {
                        basic_enum.into_float_value()
                    }
                    _ => {
                        panic!()
                    }
                };
                let rhs_llvm_value = match self.accecpt_expression(binary_expr.right.as_ref()) {
                    ExprResult::Float(float_value) => {
                        float_value
                    }
                    ExprResult::BasicEnum(basic_enum) => {
                        basic_enum.into_float_value()
                    }
                    _ => {
                        panic!()
                    }
                };
                let llvm_basic_block = self.current_block.as_ref().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(*llvm_basic_block);
                match binary_expr.operator {
                    Operator::Plus => {
                        ExprResult::Float(builder.build_float_add(lhs_llvm_value, rhs_llvm_value, "tempAdd"))
                    }
                    Operator::Minus => {
                        ExprResult::Float(builder.build_float_sub(lhs_llvm_value, rhs_llvm_value, "tempSub"))
                    }
                    Operator::Multply => {
                        ExprResult::Float(builder.build_float_mul(lhs_llvm_value, rhs_llvm_value, "tempMul"))
                    }
                    Operator::Divide => {
                        ExprResult::Float(builder.build_float_div(lhs_llvm_value, rhs_llvm_value, "tempDiv"))
                    }
                    Operator::Mod => {
                        ExprResult::Float(builder.build_float_rem(lhs_llvm_value, rhs_llvm_value, "tempMod"))
                    }
                    _ => {
                        panic!()
                    }

                }
    } 
}