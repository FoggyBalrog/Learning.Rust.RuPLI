use std::iter::Peekable;
use std::slice::Iter;

use crate::ast::ASTNode;
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::Identifier(name) => {
                let name = name.clone();
                match iter.next() {
                    Some(Token::Equals) => {
                        let expr = parse_expression(&mut iter);
                        ast.push(ASTNode::Assignment {
                            name,
                            expr: Box::new(expr),
                        });
                    }
                    _ => panic!("Expected '=' after identifier"),
                }
            }
            Token::In => {
                if let Some(Token::Identifier(name)) = iter.next() {
                    ast.push(ASTNode::In { name: name.clone() });
                } else {
                    panic!("Expected identifier after 'in'");
                }
            }
            Token::Out => {
                let expr = parse_expression(&mut iter);
                ast.push(ASTNode::Out {
                    expr: Box::new(expr),
                });
            }
            Token::NewLine => {
                // Ignore
            }
            _ => panic!("Unexpected token {:?}", token),
        }
    }

    ast
}

fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    let left = match tokens.next() {
        Some(Token::Number(n)) => ASTNode::Number(*n),
        Some(Token::Identifier(name)) => ASTNode::Identifier(name.clone()),
        _ => panic!("Expected number or identifier"),
    };

    while let Some(token) = tokens.peek() {
        match token {
            Token::Plus => {
                tokens.next();
                let right = parse_expression(tokens);
                return ASTNode::Expression {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            Token::Minus => {
                tokens.next();
                let right = parse_expression(tokens);
                return ASTNode::Expression {
                    left: Box::new(left),
                    right: Box::new(ASTNode::Negate {
                        expr: Box::new(right),
                    }),
                };
            }
            _ => break,
        }
    }

    left
}
