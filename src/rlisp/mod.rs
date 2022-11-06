

mod ast;
mod types;
mod rustapi;
mod stdlib;
mod execution;


use std::{fmt::{Display, Formatter}};
use core::fmt::Error;

use std::vec::Vec;

use logos::{Logos, Lexer};

use crate::rlisp::ast::{ASTValue, ASTNode};



#[derive(Default)]
pub struct ExtraLexInfo {
    line_count: u32
}


fn sepcb(lex: &mut Lexer<Token>) {
    let slice = lex.slice();
    let c: u32 = slice.matches("\n").count() as u32;
    lex.extras.line_count += c;
}



#[derive(Logos, Debug, PartialEq)]
#[logos(extras = ExtraLexInfo)]
pub enum Token {
    // Tokens can be literal strings, of any length.
    #[token(r"(")]
    Open,
    #[token(r")")]
    Close,

    #[token(r".")]
    Dot,

    #[token(r"=")]
    Equal,

    #[token(r"nil")]
    Nil,

    #[regex("(\".*\")|('.*')")]
    String,

    #[regex(r"[+-]?([0-9]*[.])?[0-9]+", priority = 2)]
    Number,

    // Or regular expressions.
    #[regex(r"[%\^<>\+\-/\*a-zA-Z0-9]+", priority = 1)]
    Id,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,

    #[regex(r"[ \t\n\f]+", sepcb)]
    Sep,
}



impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let st = match self {
            Token::Open => "(",
            Token::Close => ")",

            Token::Equal => "=",

            Token::Nil => "literal",

            Token::Dot => "dot",

            Token::String => "string",

            Token::Number => "number",
            Token::Id => "id",

            Token::Error => "ERROR",
            Token::Sep => " "
        };
        return write!(f, "{}", st);
    }
}


pub struct TokenInfo {
    pub token: Token,
    pub string: std::string::String,
    pub line: u32
}


impl Display for TokenInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        return write!(f, "{} line {}", self.token, self.line);
    }
}



pub fn tokenify(input: &str) -> Vec<TokenInfo> {
    let mut ret: Vec<TokenInfo> = Vec::new();
    let mut lex = Token::lexer(input);
    let mut n = lex.next();
    while n.is_some() {
        let tokinfo = TokenInfo {
            token: n.unwrap(),
            string: lex.slice().to_string(),
            line: lex.extras.line_count
        };
        ret.push(tokinfo);
        n = lex.next();
    }
    return ret;
}




pub fn id(toks: &Vec<TokenInfo>, index: usize) -> (ast::ASTNode, usize) {
    if Token::Id == toks[index].token {
        let x = &toks[index].string;
        return (ast::ASTNode::Id(x.to_string()), index + 1);
    } else {
        panic!("Token wasn't a symbol")
    }
}


pub fn string(toks: &Vec<TokenInfo>, index: usize) -> (ast::ASTNode, usize) {
    if Token::String == toks[index].token {
        let x = &toks[index].string;
        let val = &x[1..x.len() - 1]; // take first and last chars off
        return (ast::ASTNode::String(val.to_string()), index + 1);
    } else {
        panic!("Token wasn't a string")
    }
}



pub fn number(toks: &Vec<TokenInfo>, index: usize) -> (ast::ASTNode, usize) {
    if Token::Number == toks[index].token {
        let x = &toks[index].string.to_string();
        let number = x.parse::<f64>().unwrap();
        // TODO: Do better error checking here.
        // currently this just panics if the number can't be unwrapped
        return (ast::ASTNode::Number(number), index + 1);
    } else {
        panic!("Token wasn't a number")
    }
}


pub fn equals(toks: &Vec<TokenInfo>, index: usize) -> (ast::ASTNode, usize) {
    assert!(toks[index].token == Token::Equal);
    let mut i = index;
    i += 1;
    while toks[index].token == Token::Sep {
        i += 1;
    }

    assert!(Token::Id == toks[index].token);
    let varname = toks[index].token.to_string();
    let u = match_next(toks, i);
    let (anode, i) = u.unwrap();
    let astval = match anode {
        ast::ASTNode::Expression(x) => ASTValue::Expression(x),
        ast::ASTNode::Number(x) => ASTValue::Number(x),
        ast::ASTNode::String(x) => ASTValue::String(x),
        ast::ASTNode::Id(x) => ASTValue::Id(x),
        _ => panic!("Bad equals value, expected number, string, expression, or variable.")
    };
    
    let ret = ast::ASTNode::Equals(varname, astval);

    return (ret, i);
}



pub fn match_next(toks: &Vec<TokenInfo>, i: usize) -> Option<(ast::ASTNode, usize)> {
    let t = &toks[i];
    match t.token {
        Token::Open => {
            return Option::Some(expr(toks, i));
        },
        Token::Close => {
            panic!("Syntax error: Unexpected close.");
        },
        Token::Dot => {
            panic!("Dot is NYI");
        },
        Token::String => {
            return Option::Some(string(toks, i));
        },
        Token::Id => {
            return Option::Some(id(toks, i));
        },
        Token::Number => {
            return Option::Some(number(toks, i));
        },
        Token::Error => {
            panic!("Syntax error");
        },
        Token::Equal => {
            return Option::Some(equals(toks, i));
        },
        Token::Sep => {
            return match_next(toks, i + 1);
        },
        Token::Nil => {
            return Option::Some((ast::ASTNode::Literal(types::Literal::Nil), i + 1));
        }
    }
}




pub fn expr(toks: &Vec<TokenInfo>, index: usize) -> (ast::ASTNode, usize) {
    let mut ret = Vec::new();
    assert!(toks[index].token == Token::Open);
    let mut i = index;
    i += 1;

    while toks.len() > i {
        let t = &toks[i];

        match t.token {
            Token::Open => {
                let (nest_ast, nest_i) = expr(toks, i);
                ret.push(nest_ast);
                i = nest_i;
            },
            Token::Nil => {
                let v = ASTNode::Literal(
                    types::Literal::Nil
                );
                ret.push(v);
            },
            Token::Close => {
                return (ast::ASTNode::Expression(ret), i);
            },
            Token::Dot => {
                panic!("NYI");
            },
            Token::String => {
                let v;
                (v, i) = string(toks, i);
                ret.push(v);
            },
            Token::Id => {
                let v;
                (v, i) = id(toks, i);
                ret.push(v);
            },
            Token::Number => {
                let v;
                (v, i) = number(toks, i);
                ret.push(v);
            },
            Token::Error => {
                panic!("Syntax error");
            },
            Token::Equal => {
                let v;
                (v, i) = equals(toks, i);
                ret.push(v);
            }
            Token::Sep => {
                i += 1;
            }
        }
    }

    panic!("Syntax error: Missing close.");
}



pub fn parse(toks: Vec<TokenInfo>) -> ast::AST {
    let mut i: usize = 0;
    while toks[i].token == Token::Sep {
        i += 1;
    }

    let mut a = std::vec::Vec::new();
    let (x, _) = expr(&toks, i);

    a.push(x);
    return a;
}


pub fn make_scope() -> types::Scope {
    let mut scope = types::Scope {
        local: std::collections::HashMap::new(),
        parent: Option::None
    };
    stdlib::export_default_lib(&mut scope);
    return scope;
}




pub fn execute(ast: ast::AST, scope: &mut types::Scope) {
}




