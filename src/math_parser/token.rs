use self::OperPrec::*; 
use self::Token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
        Add, 
        Subtract, 
        Multiply, 
        Divide, 
        Caret, 
        LeftParen, 
        RightParen, 
        Num(f64), 
        EOF,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum OperPrec {
        DefaultZero, 
        AddSub, 
        MulDiv, 
        Power, 
        Negative,
}

impl Token {
        pub fn get_oper_prec(&self) -> OperPrec {
                match *self {
                        Add | Subtract => AddSub, 
                        Multiply | Divide => MulDiv, 
                        Caret => Power, 
                        _=> DefaultZero,
                }       
        }
}

