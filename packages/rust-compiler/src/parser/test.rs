
mod test {
    use crate::parser::Parser;
    use crate::ast::*;
    fn to_ast(code: &'static str ) -> Program {
        let mut parser = Parser::new(String::from(code));
        return parser.parse();
    }
    #[test]
    fn test_while_statement_parse() {
        assert_eq!(
            to_ast("
                while(i >= 1) {
                    i = i - 1;
                }
            "),
            Program{
                body: vec![
                    ProgramItem::Stmt(Stmt::WhileStmt(WhileStatement{
                        test: Expr::BinaryExpr(BinaryExpression { 
                            left: Box::new(Expr::Ident(Identifier{ name: String::from("i") })), 
                            right: Box::new(Expr::NumberExpr(NumberLiteral{ value: 1.0 })), 
                            operator: Operator::Gteq
                        }),
                        body: Box::new(Stmt::BlockStmt(BlockStatement { 
                            body: vec![ProgramItem::Expr(Expr::AssigmentExpr(AssigmentExpression { 
                                left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                right: Box::new(Expr::BinaryExpr(BinaryExpression { 
                                    left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                    right: Box::new(Expr::NumberExpr(NumberLiteral { value: 1.0 })), 
                                    operator: Operator::Minus
                                }) )
                            }))]
                        }))
                    }))
                ]
            }
        );
    }
    #[test]
    fn test_if_else_statement_parse() {
        assert_eq!(
            to_ast("
                if(i >= 0) {
                    i = i + 9;
                }else {
                    i = i - 9;
                }
            "),
            Program{
                body: vec![
                    ProgramItem::Stmt(Stmt::IfStmt(IfStatement { 
                        test: Expr::BinaryExpr(BinaryExpression { 
                            left: Box::new(Expr::Ident(Identifier{ name: String::from("i") })), 
                            right: Box::new(Expr::NumberExpr(NumberLiteral{ value: 0.0 })), 
                            operator: Operator::Gteq
                        }), 
                        consequent: Box::new(Stmt::BlockStmt(BlockStatement { 
                            body: vec![ProgramItem::Expr(Expr::AssigmentExpr(AssigmentExpression { 
                                left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                right: Box::new(Expr::BinaryExpr(BinaryExpression { 
                                    left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                    right: Box::new(Expr::NumberExpr(NumberLiteral { value: 9.0 })), 
                                    operator: Operator::Plus
                                }) )
                            }))]
                        })),
                        alter: Some(Box::new(Stmt::BlockStmt(BlockStatement { 
                            body: vec![ProgramItem::Expr(Expr::AssigmentExpr(AssigmentExpression { 
                                left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                right: Box::new(Expr::BinaryExpr(BinaryExpression { 
                                    left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                    right: Box::new(Expr::NumberExpr(NumberLiteral { value: 9.0 })), 
                                    operator: Operator::Minus
                                }) )
                            }))]
                        })))
                    }))
                ]
            }
        )
    }
    #[test]
    fn test_if_else_if_statement_parse() {
        assert_eq!(
            to_ast("
                if(i >= 0) {
                    i = i + 9;
                }else if (i<=10) {
                    i = i - 9;
                }
            "),
            Program{
                body: vec![
                    ProgramItem::Stmt(Stmt::IfStmt(IfStatement { 
                        test: Expr::BinaryExpr(BinaryExpression { 
                            left: Box::new(Expr::Ident(Identifier{ name: String::from("i") })), 
                            right: Box::new(Expr::NumberExpr(NumberLiteral{ value: 0.0 })), 
                            operator: Operator::Gteq
                        }), 
                        consequent: Box::new(Stmt::BlockStmt(BlockStatement { 
                            body: vec![ProgramItem::Expr(Expr::AssigmentExpr(AssigmentExpression { 
                                left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                right: Box::new(Expr::BinaryExpr(BinaryExpression { 
                                    left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                    right: Box::new(Expr::NumberExpr(NumberLiteral { value: 9.0 })), 
                                    operator: Operator::Plus
                                }) )
                            }))]
                        })),
                        alter: Some(Box::new(Stmt::IfStmt(IfStatement { 
                            test:  Expr::BinaryExpr(BinaryExpression { 
                                left: Box::new(Expr::Ident(Identifier{ name: String::from("i") })), 
                                right: Box::new(Expr::NumberExpr(NumberLiteral{ value: 10.0 })), 
                                operator: Operator::Lteq
                            }), 
                            consequent: Box::new(Stmt::BlockStmt(BlockStatement { 
                                body: vec![ProgramItem::Expr(Expr::AssigmentExpr(AssigmentExpression { 
                                    left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                    right: Box::new(Expr::BinaryExpr(BinaryExpression { 
                                        left: Box::new(Expr::Ident(Identifier { name: String::from("i") })), 
                                        right: Box::new(Expr::NumberExpr(NumberLiteral { value: 9.0 })), 
                                        operator: Operator::Minus
                                    }) )
                                }))]
                            })), 
                            alter: None 
                        })))
                    }))
                ]
            }
        )
    }
}