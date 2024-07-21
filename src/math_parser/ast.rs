use std::error;

#[derive(Debug, Clone)]
pub enum Node {
        Add(Box<Node>, Box<Node>),
        Subtract(Box<Node>, Box<Node>),
        Multiply(Box<Node>, Box<Node>),
        Divide(Box<Node>, Box<Node>),
        Caret(Box<Node>, Box<Node>),
        Negative(Box<Node>),
        Number(f64),
}
pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>>{
        match expr {
                Node::Number(i) => Ok(i),
                Node::Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
                Node::Subtract(expr1, expr2 ) => Ok(eval(*expr1)? - eval(*expr2)?),
                Node::Multiply(expr1, expr2 ) => Ok(eval(*expr1)? * eval(*expr2)?),
                Node::Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
                Node::Negative(expr1) => Ok(-(eval(*expr1)?)),
                Node::Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
        }
}