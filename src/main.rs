pub mod ast;
pub mod operator;
pub mod operator_parsers;
pub mod term;
pub mod term_parsers;
pub mod statement;
pub mod statement_parsers;

pub type Mem = [u8; 256];

fn main() {
    println!("Hello, world!");
}
