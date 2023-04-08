mod lexer;
mod parser;
mod codegen;
mod ast;
mod token;
mod utils;

use inkwell::context::Context;

fn main() {
    let mut parser = parser::Parser::new(String::from("
        function nothing (): void {
            return;
        }
        function test(a, b): number {
            var c ;
            while(a >= -6) {
                c = c + 1;
                a = a - 1;
            }
            return c
        }

        function main(): number {
            return test(-1, 2)
        }
    "));
    let program = parser.parse();
    println!("{:?}", program);
    let context = Context::create();
    let mut codegen = codegen::Codegen::new(&context,&program );
    codegen.generate();
    codegen.print_to_stderr();
    codegen.execute();
    

}
