/* In here , tokens are converting into ast, 
*  what is an Ast?? https://en.wikipedia.org/wiki/Abstract_syntax_tree
*/


use std::fmt;

use super::ast::Node;
use super::tokenizer::Tokenizer;
use super::token::{OperPrec, Token};


pub struct Parser<'a> {
        tokenizer: Tokenizer<'a>, 
        current_token: Token,
}

#[derive(Debug)]
pub enum ParserError {
        UnableToParse(String),
        InvalidOperator(String),
}


impl<'a>Parser<'a> {
        pub fn new(expr: &'a str) -> Result<Self, ParserError> {
                let mut lexer = Tokenizer::new(expr);
                let cur_token = match lexer.next() {
                        Some(token) => token, 
                        None => return Err(ParserError::InvalidOperator("Invalid charcater".into())),
                };
                Ok(Parser {
                        tokenizer: lexer, 
                        current_token: cur_token, 
                })
        }

        // take arithmatic expression and resturn it into ast

        pub fn parser(&mut self) -> Result<Node, ParserError> {
                let get_ast = self.generate_ast(OperPrec::DefaultZero);
                match get_ast {
                        Ok(my_ast) => Ok(my_ast),
                        Err(e) => Err(e),
                }
        }
}

// parser method -- private --

impl<'a> Parser<'a> {
        fn get_next_token(&mut self) -> Result<(), ParserError> {
                let next_token = match self.tokenizer.next() {
                        Some(token) => token, 
                        None => return Err(ParserError::InvalidOperator("Invalid Chracter".into())),
                };
                self.current_token = next_token;
                Ok(())
        }

        fn generate_ast(&mut self,get_oper: OperPrec) -> Result<Node, ParserError> {
                let mut left_expr = self.parse_number()?;
                
                while get_oper < self.current_token.get_oper_prec() {
                        if self.current_token == Token::EOF {
                                break;
                        }
                        let right_expr = self.convert_token_to_node(left_expr.clone());
                        left_expr = right_expr?;
                }
                Ok(left_expr)
        }

        // ast node for numbers 

        fn parse_number(&mut self) -> Result<Node, ParserError> {
                let token = self.current_token.clone();
                match token {
                        Token::Subtract => {
                                self.get_next_token()?;
                                let expr = self.generate_ast(OperPrec::Negative)?;
                                Ok(Node::Negative(Box::new(expr)))
                        }
                        Token::Num(i) => {
                                self.get_next_token()?;
                                Ok(Node::Number(i))
                        }
                        Token::LeftParen => {
                                self.get_next_token()?;
                                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                                self.check_paren(Token::RightParen)?;
                                        if self.current_token == Token::LeftParen {
                                                let right = self.generate_ast(OperPrec::MulDiv)?;
                                                return Ok(Node::Multiply(Box::new(expr.clone()), Box::new(expr)));
                                        }
                                Ok(expr)
                        }
                        _ => Err(ParserError::UnableToParse("Unable to parse".to_string())),
                } 
        }

        //check for balacing paranthesis
        fn check_paren(&mut self, expected: Token) -> Result<(), ParserError> {
                if expected == self.current_token {
                        self.get_next_token()?;
                        Ok(())
                }else {
                        Err(ParserError::InvalidOperator(format!("Expected {:?}, got {:?}", expected, self.current_token)))
                }
        }

        //construct Operator ast node 
        fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParserError> {
                match self.current_token {
                        Token::Add => {
                                self.get_next_token()?;
                                // right side expression
                                let  right_expr = self.generate_ast(OperPrec::AddSub)?;
                                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
                        }
                        Token::Subtract => {
                                self.get_next_token()?;
                                //Get right-side expression
                                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
                            }
                            Token::Multiply => {
                                self.get_next_token()?;
                                //Get right-side expression
                                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
                            }
                            Token::Divide => {
                                self.get_next_token()?;
                                //Get right-side expression
                                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
                            }
                            Token::Caret => {
                                self.get_next_token()?;
                                //Get right-side expression
                                let right_expr = self.generate_ast(OperPrec::Power)?;
                                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
                            }
                            _ => Err(ParserError::InvalidOperator(format!(
                                "Please enter valid operator {:?}",
                                self.current_token
                            ))),
                        
                }
        }
}

// Error handling
impl fmt::Display for ParserError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ParserError::UnableToParse(e) => write!(f, "Unable to parse: {}", e),
                ParserError::InvalidOperator(e) => write!(f, "Invalid operator: {}", e),
            }
        }
    }
    

impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParserError {
        fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
            return ParserError::UnableToParse("Unable to parse".into());
        }
    }