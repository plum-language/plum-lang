use crate::lexer::Token;
use logos::Lexer;
use std::iter::Peekable;

pub fn parse<I: Iterator<Item = Result<Token, ()>>>(iter: &mut Peekable<I>) {
    while let Some(t) = iter.next() {
        println!("{:?}", t);
    }
}

fn parse_statement() -> Stmt {
    Stmt::ExprStmt
}

fn parse_declaration() -> Decl {
    Decl::ClassDecl
}

fn parse_expression() -> Expr {
    Expr::Assign
}

enum Stmt {
    ExprStmt,
}

enum Decl {
    ClassDecl,
}

enum Expr {
    Assign,
}
