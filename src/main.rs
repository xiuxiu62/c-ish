pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod parser;
mod util;

use lexer::Lexer;

const TEST_DATA: &'static str = "void hello(void) { return; } \nchar* temp = \"hello\";";

fn main() {
    let lexer = Lexer::new(TEST_DATA);
    let tokens = lexer.run();

    println!("{tokens:#?}");
}
