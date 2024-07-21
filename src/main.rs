/* PROJECT: ARITHMATIC EXPRESSION SIMULATOR
 * DATE: 19TH JULY 2024
 */

use std::io;

use math_parser::tokenizer;
use math_parser::ast;
use math_parser::parser::{ParserError, Parser};

mod math_parser;

fn checking(expr: String) -> Result<f64, ParserError> {
    let expression = expr.split_whitespace().collect::<String>(); // remove whitespace chars
    let mut math_parser = Parser::new(&expression)?;
    let get_ast = math_parser.parser()?;

    println!("Generated AST is {:?}", get_ast);

    Ok(ast::eval(get_ast)?)
}

fn main() {
    println!(r"
    ┌──────────────────────────┐
    │   _   ___ ___ ___ __  __ │
    │  /_\ | __/ __|_ _|  \/  |│
    │ / _ \| _|\__ \| || |\/| |│
    │/_/ \_\___|___/___|_|  |_|│
    └──────────────────────────┘
    Enter your Arithmatic Here ..
        ");
        
    loop {
        print!(">> \n");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match checking(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(_) => {
                        println!("Error in evaluating expression \n");
                    }
                };
            }

            Err(error) => println!("error: {}", error),
        }
    }
}