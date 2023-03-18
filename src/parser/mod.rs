use crate::lexer::Lexer;
use crate::token::{Token, get_pre_of_binary_op, is_binary_op};
use crate::ast::*;
pub struct  Parser {
    tokenizer: Lexer,
}
impl Parser {
    pub fn new(code: String) -> Parser{
        Parser {
            tokenizer: Lexer::new(code)
        }
    }
    fn sytanx_error(&self) {
        panic!()
    }
    pub fn parse_program(&mut self) -> Program {
        let mut body = Vec::<ProgramItem>::new();
        loop {
            match self.tokenizer.get_token() {
                Token::EOF => {
                    return Program { body }
                }
                _ => {
                    body.push(self.parse_program_item());
                }
            }
        }
    }
    fn parse_program_item(&mut self) -> ProgramItem {
        match self.tokenizer.get_token() {
            Token::VarKeyword => {
                ProgramItem::Decl(self.parse_variable_declaration())
            }
            Token::FunctionKeyword => {
                ProgramItem::Decl(self.parse_function_declaration())
            }
            Token::BracesLeft => {
                ProgramItem::Stmt(self.parse_block_statement())
            }
            Token::WhileKeyword => {
                ProgramItem::Stmt(self.parse_while_statement())
            }
            Token::IfKeyword => {
                ProgramItem::Stmt(self.parse_if_statement())
            }
            Token::ReturnKeyword => {
                ProgramItem::Stmt(self.parse_return_statement())
            }
            _ => {
                ProgramItem::Expr(self.parse_expression())
            }
        }
    }
/** =========================================
 *  Parse Statements
 * ==========================================
 */
    fn parse_while_statement(&mut self) -> Stmt {
        let test: Expr;
        match self.tokenizer.get_token() {
             Token::WhileKeyword  => {
                self.tokenizer.next_token();
             }
             _ => {
                self.sytanx_error();
             }
        }
        match self.tokenizer.get_token() {
            Token::ParenthesesLeft  => {
                self.tokenizer.next_token();
                test = self.parse_expression();
            }
            _ => {
                panic!("[Error]: While Statement's Condition Should be Wrapped By ParentheseLeft");
            }
       }
       match self.tokenizer.get_token() {
            Token::ParenthesesRight => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: While Statement's Condition Should be Wrapper By ParentheseRight")
            }
       }
       return Stmt::WhileStmt(WhileStatement{
            test,
            body: Box::<Stmt>::new(self.parse_block_statement())
       })

    }
    fn parse_block_statement(&mut self) -> Stmt {
        match self.tokenizer.get_token() {
            Token::BracesLeft => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: BlockStatement Should Start With BraceLeft, {:?}", self.tokenizer.get_token());
            }
        }
        let mut body: Vec<ProgramItem> = Vec::<ProgramItem>::new();
        loop {
            match self.tokenizer.get_token() {
                Token::EOF => {
                    panic!("[Error]: BlockStatement End Without BraceRight");
                }
                Token::BracesRight => {
                    self.tokenizer.next_token();
                    return Stmt::BlockStmt(BlockStatement{
                        body
                    })
                }
                _ => {
                    body.push(self.parse_program_item());
                }
            }
        }
    }
    fn parse_if_statement(&mut self) -> Stmt {
        match self.tokenizer.get_token() {
            Token::IfKeyword => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: If Statement Should Start With `if` keyword.");
            }
        }
        let test:Expr;
        match self.tokenizer.get_token() {
            Token::ParenthesesLeft  => {
                self.tokenizer.next_token();
                test = self.parse_expression();
            }
            _ => {
                panic!("[Error]: Condition Of If Statement Should be Wrapper In Parentheses, Lock of ParentheseLeft");
            }
       }
       match self.tokenizer.get_token() {
            Token::ParenthesesRight => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: Condition Of If Statement Should be Wrapper In Parentheses, Lock of ParentheseRight");
            }
       }
       let consequence = self.parse_block_statement();
       match self.tokenizer.get_token() {
           Token::ElesKeyword => {
             self.tokenizer.next_token();
             match self.tokenizer.get_token() {
                Token::BracesLeft => {
                    Stmt::IfStmt(IfStatement { test, consequent: Box::new(consequence), alter:Some(Box::new(self.parse_block_statement())) })
                }
                Token::IfKeyword => {
                    Stmt::IfStmt(IfStatement { test, consequent: Box::new(consequence), alter:Some(Box::new(self.parse_if_statement())) })
                }
                _ => {
                    panic!("[Error]: Else Keyword Must Concat With Block Statement Or If Statement");
                }
             }
           }
           _ => {
            Stmt::IfStmt(IfStatement { test, consequent: Box::new(consequence), alter: None })
           }
       }
    }
    fn parse_return_statement(&mut self) -> Stmt {
        match self.tokenizer.get_token() {
            Token::ReturnKeyword => {
                self.tokenizer.next_token();
                return Stmt::ReturnStmt(ReturnStatement {
                    argument: self.parse_expression()
                })
            }
            _ => {
                panic!("[Error]: Return Statement Should Start With Return Keyword.");
            }
        }
    }
/** ===========================================
 *  Parse Declaration
 * ============================================
 */
    fn parse_variable_declaration(&mut self) -> Decl {
        let identifier_name: String;
        match self.tokenizer.get_token() {
            Token::VarKeyword => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: Variable Declaration Should Start With `var` keyword");
            }
        }
        match self.tokenizer.get_token() {
            Token::Identifier(value) => {
                self.tokenizer.next_token();
                identifier_name = value;
            }
            _ => {
                panic!("[Error]: Variable Delcaration Should Provide A Identifier.");
            }
        }
        return match self.tokenizer.get_token() {
            Token::Assign => {
                self.tokenizer.next_token();
                let init_expression = self.parse_expression();
                Decl::VariableDecl(VariableDeclaration { 
                    name: identifier_name, 
                    init: Some(init_expression)
                })
            }
            _ => {
                Decl::VariableDecl(VariableDeclaration { 
                    name: identifier_name, 
                    init: None
                })
            }
        }
    }
    fn parse_function_declaration(&mut self) -> Decl {
        let function_name: String;
        let function_type: Type;
        match self.tokenizer.get_token() {
            Token::FunctionKeyword => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: Function Declaration Should Start With `var` keyword");
            }
        }
        match self.tokenizer.get_token() {
            Token::Identifier(value) => {
                self.tokenizer.next_token();
                function_name = value;
            }
            _ => {
                panic!("[Error]: Function Delcaration Should Provide A Identifier.");
            }
        }
        let arguments = self.parse_function_declaration_aruguments();
        match self.tokenizer.get_token() {
            Token::Colon => {
                self.tokenizer.next_token();
            }
            _ => {
                println!("{:?}", arguments);
                panic!("[Error]: Function Declaration Must Has Return Type With Colon, But Got {:?}", self.tokenizer.get_token())
            }
        }
        match self.tokenizer.get_token() {
            Token::NumberKeyword  => {
                self.tokenizer.next_token();
                function_type  = Type::Number;
             } 
             Token::VoidKeyword => {
                self.tokenizer.next_token();
                function_type = Type::Void;
             }
            _ => {
                panic!("[Error]: Function Declaration Must Has Return Type, But Got {:?}", self.tokenizer.get_token())
            }
        }
        let body = self.parse_block_statement();
        return Decl::FunctionDecl(FunctionDeclaration { 
            name:function_name , 
            return_type: function_type, 
            arguments , 
            body
        })

    }
    // argument -> identifier [',' identifier]
    fn parse_function_declaration_aruguments(&mut self) -> Vec<String> {
        match self.tokenizer.get_token() {
            Token::ParenthesesLeft => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: Function Declaration Params Must Be Wrapped In ParenthesesLeft, But Got {:?}", self.tokenizer.get_token())
            }
        }
        let mut params = Vec::<String>::new();
        match self.tokenizer.get_token() {
            Token::Identifier(name) => {
                params.push(name);
                self.tokenizer.next_token();
            }
            _ => {
                match self.tokenizer.get_token() {
                    Token::ParenthesesRight => {
                        self.tokenizer.next_token();
                    }
                    _ => {
                        panic!("[Error]: Function Declaration Params Must Be Wrapped In ParenthesesRight, But Got {:?}", self.tokenizer.get_token())
                    }
                }
                return params;
            }
        }
        loop {
            match self.tokenizer.get_token() {
                Token::Comma => {
                    self.tokenizer.next_token();
                }
                _ => {
                    break
                }
            }
            match self.tokenizer.get_token() {
                Token::Identifier(name) => {
                    params.push(name);
                    self.tokenizer.next_token();
                }
                _ => {
                    break;
                }
            }
        }
        match self.tokenizer.get_token() {
            Token::ParenthesesRight => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: Function Declaration Params Must Be Wrapped In ParenthesesRight, But Got {:?}", self.tokenizer.get_token())
            }
        }
        return params;
    }
/** ===================================================
 *  Parse Expression
 * ====================================================
 */
    fn parse_expression(&mut self) -> Expr {
        let mut expressions = vec![self.parse_assignment_expression()];
        loop {
            match self.tokenizer.get_token() {
                Token::Comma => {
                    self.tokenizer.next_token();
                    expressions.push(self.parse_assignment_expression())
                }
                _ => {
                    break;
                }
            }
        }
        if expressions.len() == 1 {
            return expressions[0].clone()
        }else {
            return Expr::SequnceExpr(SequnceExpression {
                expressions
            })
        }
    }
    fn parse_assignment_expression(&mut self) -> Expr {
        let left = self.parse_condition_expression();
        return match self.tokenizer.get_token() {
            Token::Assign => {
                self.tokenizer.next_token();
                Expr::AssigmentExpr(AssigmentExpression { 
                    left: Box::<Expr>::new(left), 
                    right: Box::<Expr>::new(self.parse_condition_expression()), 
                }) 
            }
            _ => {
                left
            }
        }
    }
    fn parse_condition_expression(&mut self) -> Expr {
        let test = self.parse_binary_expression();
        match self.tokenizer.get_token() {
            Token::Qustion => {
                self.tokenizer.next_token();
            }
            _ => {
                return test;
            }
        }
        let consequence =  self.parse_binary_expression();
        match self.tokenizer.get_token() {
            Token::Colon => {
                self.tokenizer.next_token();
                return Expr::ConditionExpr(ConditionExpression {
                    test: Box::<Expr>::new(test),
                    consequnce: Box::<Expr>::new(consequence),
                    alter:Box::<Expr>::new(self.parse_binary_expression())

                })
            }
            _ => {
                panic!("[Error]: Conditional Expression Should And Consequnce And Alter Expression");
            }
        }
    }
    fn parse_binary_expression(&mut self)-> Expr {
        let atom = self.parse_unary_expression();
        let op = self.tokenizer.get_token();
        if is_binary_op(&op) {
            return self.parse_binary_ops(atom, -1)
        }
        return atom;
    }
    fn parse_binary_ops(&mut self, mut left: Expr, last_pre: i32) -> Expr {
        loop {
            let current_op = self.tokenizer.get_token();
            if !is_binary_op(&current_op) || get_pre_of_binary_op(&current_op) < last_pre {
                break;
            }
            self.tokenizer.next_token();
            let mut right = self.parse_unary_expression();
            let next_op = self.tokenizer.get_token();
            if  
                is_binary_op(&next_op) && 
                (get_pre_of_binary_op(&next_op) > get_pre_of_binary_op(&current_op)) 
            {
                
                right = self.parse_binary_ops(right, get_pre_of_binary_op(&next_op))
            }
            left = Expr::BinaryExpr(BinaryExpression { 
                left: Box::<Expr>::new(left.clone()), 
                right: Box::<Expr>::new(right.clone()), 
                operator: get_ast_type_of_binary_op_token(current_op)
            } );
        }
        return left;
    }
    fn parse_unary_expression(&mut self) -> Expr {
        return match self.tokenizer.get_token() {
            Token::Plus => {
                self.tokenizer.next_token();
                Expr::UnaryExpr(UnaryExpression {
                    operator: Operator::Plus,
                    argument: Box::<Expr>::new(self.parse_primary_expression())
                })
            }
            Token::Minus => {
                self.tokenizer.next_token();
                Expr::UnaryExpr(UnaryExpression {
                    operator: Operator::Plus,
                    argument: Box::<Expr>::new(self.parse_primary_expression())
                })
            }
            _ => {
                self.parse_primary_expression()
            }
        }
    }
    fn parse_primary_expression(&mut self) -> Expr {
        match self.tokenizer.get_token() {
            Token::Identifier(identifier) => {
                self.tokenizer.next_token();
                match self.tokenizer.get_token() {
                    Token::ParenthesesLeft => {
                        let params = self.parse_call_expression_param();
                        Expr::CallExpr(CallExpression { callee_name: identifier, params })
                    }
                    _ => {
                        Expr::Ident(Identifier {
                            name: identifier,
                        })
                    }
                }
            }
            Token::NumberLiteral(value) => {
                self.tokenizer.next_token();
                Expr::NumberExpr(NumberLiteral{
                    value
                })
            }
            Token::ParenthesesLeft => {
                self.tokenizer.next_token();
                let expr = self.parse_expression();
                return match self.tokenizer.get_token() {
                    Token::ParenthesesRight => {
                        self.tokenizer.next_token();
                        expr
                    }
                    _ => {
                        panic!("[Error]: CoverParenthesizedExpression Must End With ParentheseRight, But Got {:?}", self.tokenizer.get_token());
                    }
                }
            }
            _ => {
                panic!("[Error]: Failed For Get Primary Expression, Unexecpted Token {:?}.", self.tokenizer.get_token())
            }
        }
    }
    fn parse_call_expression_param(&mut self) -> Vec<Expr> {
        let mut params = Vec::<Expr>::new();
        match self.tokenizer.get_token() {
            Token::ParenthesesLeft  => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: CallExpression's Param Call Be Wrapped By ParentheseLeft, But Got {:?}", self.tokenizer.get_token());
            }
        }
        loop {
            match self.tokenizer.get_token() {
                Token::ParenthesesRight => {
                    break;
                }
                _ => {
                    params.push(self.parse_condition_expression());
                    println!("{:?}",params);
                }
            }
            match self.tokenizer.get_token() {
                Token::Comma => {
                    self.tokenizer.next_token();
                }
                _ => {
                    println!("{:?}", self.tokenizer.get_token() );
                    break;
                }
            }
        }
        match self.tokenizer.get_token() {
            Token::ParenthesesRight  => {
                self.tokenizer.next_token();
            }
            _ => {
                panic!("[Error]: CallExpression's Param Call Be Wrapped By ParentheseRight, But Got {:?}", self.tokenizer.get_token());
            }
        }
        return params;
    }
}