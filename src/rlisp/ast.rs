
use crate::rlisp::types;




pub enum ASTNode {
    Expression(AST),
    Equals(types::String, ASTValue),
    Number(types::Number),
    String(types::String),
    Id(types::String),
    Literal(types::Literal)
}

pub enum ASTValue {
    Number(types::Number),
    String(types::String),
    Id(types::String),
    Expression(AST),
    Literal(types::Literal)
}



// for example,   (+ 1 2)
// another example:  (= x 2)
// Expression( Id('+'),  Number(1),  Number(2) )
pub type AST = std::vec::Vec<ASTNode>;


