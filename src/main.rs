mod lexer;
mod parser;
mod codegen;
mod ast;
mod token;
mod utils;

use inkwell::context::Context;

fn main() {
    let mut lexer = lexer::Lexer::new(String::from("
    function test(a, b): number {
        return a + b
    }
    function main(): number {
        return test(1.0, 2.0)
    }
    "));
    let mut token_vec = Vec::<token::Token>::new();
    loop {
        let t = lexer.next_token();
        token_vec.push(t.clone());
        match t {
            token::Token::EOF => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
    println!("{:?}", token_vec);
    let mut parser = parser::Parser::new(String::from("
        function test(a, b): number {
            var c ;
            if(a) {
                c = a + b ;
            }else {
                c = a - b ;
            }
            var d 
            d = a + b
            return c + d
        }
        function main(): number {
            return test(1, 2)
        }
    "));
    let program = parser.parse_program();
    println!("{:?}", program);
    let context = Context::create();
    let mut codegen = codegen::Codegen::new(&context,&program );
    codegen.generate();
    codegen.print_to_stderr();
    println!("{:?}", program);
    codegen.execute();
    

}
