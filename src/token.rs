#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    // Keywords
    WhileKeyword,   // while
    ForKeyword,     // for
    IfKeyword,      // if
    ElesKeyword,    // else
    ReturnKeyword,  // return
    VarKeyword,     // var
    FunctionKeyword, // function
    NumberKeyword,  // number
    VoidKeyword,    // void
    // StringKeyword,  // string
    // BoolKeyword,    // bool
    // NullKeyword,    // null
    // Punctuation
    Semi,           // ;
    Colon,          // :
    Comma,          // ,
    BracesLeft,     // {
    BracesRight,    // }
    BracketLeft,    // [
    BracketRight,   // ]
    ParenthesesLeft,    // (
    ParenthesesRight,   // )
    HashTag,        // #
    Dot,            // .
    Qustion,        // ?
    // Operator
    Plus,           // +
    Minus,           // -
    Multply,        // *
    Divide,         // /
    Mod,            // %
    Assign,         // =
    Eq,             // ==
    NotEq,          // !=
    Gt,             // >
    Lt,             // <
    Gteq,           // >=
    Lteq,           // <=
    LogicalOR,      // ||
    LogicalAND,     // &&
    LogicalNOT,     // !
    // Operator Not Support
    // BitwiseOR,      // |
    // BitwiseAND,     // &
    // BitwiseNOT,     // ~
    // BitwiseXOR,     // ^
    // BitwiseLeftShift,   // <<
    // BitwiseRightShift,  // >>
    // Literal
    // BoolLiteral(bool),      // true or false
    // StringLiteral(String),  // String Data type
    NumberLiteral(f64),     // float64
    //NullLiteral,            // null Value
    // EOF
    EOF,            
}

pub fn get_pre_of_binary_op(token: &Token)-> i32 {
    return match token {
        Token::Eq | Token::NotEq => {
            3
        }
        Token::Gt | Token::Gteq | Token::Lt| Token::Lteq  => {
            4
        }
        Token::Plus | Token::Minus => {
            5
        }
        Token::Mod | Token::Multply | Token::Divide => {
            6
        }
        _ => {
            panic!()
        }
        
    }
}

pub fn is_binary_op(token: &Token) -> bool {
    return match token {
        Token::Plus | Token::Minus | Token::Multply | Token::Divide | Token::Mod |
        Token::Eq | Token::NotEq | Token::Gt | Token::Gteq | Token::Lt | Token::Lteq
        => {
            true
        }
        _ => {
            false
        }
    }
}