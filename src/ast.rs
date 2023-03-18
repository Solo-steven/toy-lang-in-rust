use crate::token::Token;
/* =============================================
   Entry Structures
   =============================================
 */
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub body: Vec<ProgramItem>
}
#[derive(Debug, PartialEq, Clone)]
pub enum ProgramItem {
    Stmt(Stmt),
    Decl(Decl),
    Expr(Expr)
}
#[derive(Debug, PartialEq, Clone)]
pub enum Decl {
    VariableDecl(VariableDeclaration),
    FunctionDecl(FunctionDeclaration)
}
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    WhileStmt(WhileStatement),
    IfStmt(IfStatement),
    ReturnStmt(ReturnStatement),
    BlockStmt(BlockStatement),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    NumberExpr(NumberLiteral),
    Ident(Identifier),
    UnaryExpr(UnaryExpression),
    BinaryExpr(BinaryExpression),
    ConditionExpr(ConditionExpression),
    AssigmentExpr(AssigmentExpression),
    SequnceExpr(SequnceExpression),
    CallExpr(CallExpression)
}
/* =============================================
   Expression
   =============================================
 */
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus, 
    Minus,
    Multply,
    Divide,
    Mod,
    Eq,
    NotEq,
    Gt,
    Lt ,
    Gteq,
    Lteq ,
}
pub fn get_ast_type_of_binary_op_token(token: Token) -> Operator{
    return match token {
        Token::Plus=> { Operator::Plus }
        Token::Minus => { Operator::Minus  }
        Token::Multply => { Operator::Multply }
        Token::Divide => {Operator::Divide }
        Token::Mod => { Operator::Mod }
        Token::Eq => { Operator::Eq }
        Token::NotEq => { Operator::NotEq }
        Token::Gt => { Operator::Gt }
        Token::Lt => { Operator::Lt }
        Token::Gteq => { Operator::Gteq }
        Token::Lteq => { Operator::Lteq } 
        _ => {
            panic!("[Error]: ")
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct NumberLiteral {
    pub value: f64
}
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Operator
}
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub argument: Box<Expr>,
    pub operator: Operator,
}
#[derive(Debug, PartialEq, Clone)]
pub struct  ConditionExpression {
    pub test: Box<Expr>,
    pub consequnce: Box<Expr>,
    pub alter: Box<Expr>
}
#[derive(Debug, PartialEq, Clone)]
pub struct  AssigmentExpression {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct  SequnceExpression {
    pub expressions: Vec<Expr>
}
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub callee_name: String,
    pub params: Vec<Expr>, 
}
/* =============================================
   Delcrations
   =============================================
 */
#[derive(Debug, PartialEq, Clone, Copy)]
 pub enum Type {
    Number,
    Void,
}
#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub init: Option<Expr>
}
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: Type,
    pub arguments: Vec<String>,
    pub body: Stmt
}
/* ==========================================
   Statement
   ==========================================
 */
#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub test: Expr,
    pub body: Box<Stmt>
}
#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
   pub body: Vec<ProgramItem>
}
#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub test: Expr,
    pub consequent: Box<Stmt>,
    pub alter: Option<Box<Stmt>>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct  ReturnStatement {
    pub argument: Expr
}


