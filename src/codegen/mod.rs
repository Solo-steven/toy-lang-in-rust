use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::basic_block::BasicBlock;
use inkwell::values::{PointerValue, FloatValue, BasicValueEnum,BasicMetadataValueEnum};
use inkwell::FloatPredicate;
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
    BasicEnum(BasicValueEnum<'ctx>), // return by load inst. (IdentExpr)
    Float(FloatValue<'ctx>),        // return by other 
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
                self.symbol_table.insert(name.clone(), llvm_value);
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
                let llvm_value = self.accecpt_expression(&return_statement.argument);
                let llvm_basic_block = self.current_block.as_ref().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(*llvm_basic_block);
                match llvm_value {
                    ExprResult::Float(float_value) => {
                        builder.build_return(Some(&float_value));
                    }
                    ExprResult::BasicEnum(basic_value) => {
                        builder.build_return(Some(&basic_value));
                    }
                }
            }
            Stmt::IfStmt(ref if_statement) => {
                // Get test expr's value
                let test_expr = &if_statement.test;
                let test_expr_llvm_value = match self.accecpt_expression(test_expr) {
                    ExprResult::BasicEnum(basic_value) => basic_value.into_float_value(),
                    ExprResult::Float(float_value) => float_value
                };
                let llvm_basic_block = self.current_block.unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(llvm_basic_block);
                let test_llvm_value = builder.build_float_compare(FloatPredicate::OEQ, test_expr_llvm_value, self.context.f64_type().const_float(1.0), "tmpCompare");
                let llvm_function = llvm_basic_block.get_parent().unwrap();
                let final_llvm_basic_block = self.context.append_basic_block(llvm_function, "tmpFinal");
                // build conseq block
                let conseq_llvm_basic_block = self.context.insert_basic_block_after(llvm_basic_block, "tmpConseq");
                self.current_block = Some(conseq_llvm_basic_block);
                self.accecpt_statement(if_statement.consequent.as_ref());
                let conseq_builder = self.context.create_builder();
                conseq_builder.position_at_end(conseq_llvm_basic_block);
                conseq_builder.build_unconditional_branch(final_llvm_basic_block);
                // build alter block if exist
                let alter_llvm_basic_block = match &if_statement.alter {
                    Some(alter_statement) => {
                        let temp_alter_llvm_basic_block = self.context.insert_basic_block_after(conseq_llvm_basic_block, "tmpAlter");
                        self.current_block = Some(temp_alter_llvm_basic_block);
                        self.accecpt_statement(&alter_statement);
                        let alter_builder = self.context.create_builder();
                        alter_builder.position_at_end(temp_alter_llvm_basic_block);
                        alter_builder.build_unconditional_branch(final_llvm_basic_block);

                        temp_alter_llvm_basic_block
                    }
                    None => {
                        final_llvm_basic_block
                    }
                };
                // build branch from original block (predecessor)
                builder.build_conditional_branch(test_llvm_value, conseq_llvm_basic_block, alter_llvm_basic_block);
                self.current_block = Some(final_llvm_basic_block);
            }
            _ => {
                panic!("")
            }
        }
    }
    fn accecpt_expression(&mut self, expression: &Expr) -> ExprResult<'ctx> {
        match *expression {
            Expr::SequnceExpr(ref sequnce_expr) => {
                for index in 1..sequnce_expr.expressions.len() {
                   if index == sequnce_expr.expressions.len() -1 {
                    return self.accecpt_expression(&sequnce_expr.expressions[index]);
                   }
                   self.accecpt_expression(&sequnce_expr.expressions[index]);
                }
                panic!("[Error]: Unreach");
            }
            Expr::AssigmentExpr(ref assignment_expr) => {
                self.accecpt_assigment_expression(assignment_expr)
            }
            Expr::ConditionExpr(ref conditional_expr) => {
                self.accept_conditional_expression(conditional_expr)
            }
            Expr::BinaryExpr(ref binary_expr) => {
                self.accecpt_binary_expression(binary_expr)
            }
            Expr::UnaryExpr(ref unary_expr) => {
                self.accept_unary_expression(unary_expr)
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
                self.accecpt_call_expression(call_expr)
            }
        }
    }
    fn accecpt_assigment_expression(&mut self, assignment_expr: &AssigmentExpression) -> ExprResult<'ctx> {
        /*
            Codegen store command to assign right hand side to left hand side
            -> 1. check left hand side llvm_value should be assignable.
            -> 2. build store inst to assign right hand side value to left hand side
        */        
        let rhs = match self.accecpt_expression(&assignment_expr.right.as_ref()) {
            ExprResult::BasicEnum(basic_value) => basic_value.into_float_value(),
            ExprResult::Float(float_value) => float_value,
        };
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
        let llvm_basic_block = self.current_block.as_ref().unwrap();
        let builder = self.context.create_builder();
        builder.position_at_end(*llvm_basic_block);
        builder.build_store(*lhs_symbol, rhs);
        ExprResult::Float(rhs)
    }
    fn accept_conditional_expression(&mut self, conditional_expr: &ConditionExpression) -> ExprResult<'ctx> {
        // test
        let test_llvm_value = match self.accecpt_expression(&conditional_expr.test) {
            ExprResult::Float(float_value) => float_value,
            ExprResult::BasicEnum(basic_value) => basic_value.into_float_value(),
        };
        let llvm_basic_block = self.current_block.unwrap();
        let builder = self.context.create_builder();
        builder.position_at_end(llvm_basic_block);
        let test_llvm_value = builder.build_float_compare(FloatPredicate::OEQ, test_llvm_value, self.context.f64_type().const_float(1.0), "tmpFComp");
        let llvm_function = llvm_basic_block.get_parent().unwrap();
        let conseq_llvm_basic_block = self.context.append_basic_block(llvm_function, "tmpConseq");
        let alter_llvm_basic_block = self.context.append_basic_block(llvm_function, "tmpAlter");
        let final_llvm_basic_block = self.context.append_basic_block(llvm_function, "tmpFinal");
        builder.build_conditional_branch(test_llvm_value, conseq_llvm_basic_block, alter_llvm_basic_block);
        // consequnce
        self.current_block = Some(conseq_llvm_basic_block);
        let conseq_llvm_value = match self.accecpt_expression(conditional_expr.consequnce.as_ref()) {
            ExprResult::Float(float_value) => float_value,
            ExprResult::BasicEnum(basic_value) => basic_value.into_float_value()
        };
        let conseq_builder = self.context.create_builder();
        conseq_builder.position_at_end(conseq_llvm_basic_block);
        conseq_builder.build_unconditional_branch(final_llvm_basic_block);
        // alter
        self.current_block = Some(alter_llvm_basic_block);
        let alter_llvm_value = match self.accecpt_expression(conditional_expr.alter.as_ref()) {
            ExprResult::Float(float_value) => float_value,
            ExprResult::BasicEnum(basic_value) => basic_value.into_float_value()
        };
        let alter_builder = self.context.create_builder();
        alter_builder.position_at_end(alter_llvm_basic_block);
        alter_builder.build_unconditional_branch(final_llvm_basic_block);
        // final
        let final_builder = self.context.create_builder();
        final_builder.position_at_end(final_llvm_basic_block);
        let phi_node =  final_builder.build_phi(self.context.f64_type(), "conditionPhi");
        phi_node.add_incoming(&[(&conseq_llvm_value, conseq_llvm_basic_block ), (&alter_llvm_value, alter_llvm_basic_block)]);
        self.current_block = Some(final_llvm_basic_block);
        ExprResult::BasicEnum(phi_node.as_basic_value())
    }
    fn accecpt_binary_expression(&mut self, binary_expr: &BinaryExpression) -> ExprResult<'ctx>  {
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
        };
        let rhs_llvm_value = match self.accecpt_expression(binary_expr.right.as_ref()) {
            ExprResult::Float(float_value) => {
                float_value
            }
            ExprResult::BasicEnum(basic_enum) => {
                basic_enum.into_float_value()
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
    fn accept_unary_expression(&mut self, unary_expr: &UnaryExpression) -> ExprResult<'ctx> {
        let llvm_value = match self.accecpt_expression(unary_expr.argument.as_ref()) {
            ExprResult::BasicEnum(basic_value) => basic_value.into_float_value(),
            ExprResult::Float(float_value) => float_value
        };
        match unary_expr.operator {
            Operator::Plus => {
                ExprResult::Float(llvm_value)
            }
            Operator::Minus => {
                let llvm_basic_block = self.current_block.as_ref().unwrap();
                let builder = self.context.create_builder();
                builder.position_at_end(*llvm_basic_block);
                ExprResult::Float(builder.build_float_neg(llvm_value, "tmpNeg"))
            }
            _ => {
                panic!()
            }
        }
    }
    fn accecpt_call_expression(&mut self, call_expr: &CallExpression) -> ExprResult<'ctx> {
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