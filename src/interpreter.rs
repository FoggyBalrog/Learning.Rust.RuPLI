use crate::ast::ASTNode;
use std::collections::HashMap;
use std::io::Write;

pub fn execute(ast: &[ASTNode]) {
    let mut env = HashMap::new();

    for node in ast {
        match node {
            ASTNode::Assignment { name, expr } => {
                let value = evaluate(expr, &env);
                env.insert(name.clone(), value);
            }
            ASTNode::In { name } => {
                print!("Enter value for {}: ", name);
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let value: i64 = input.trim().parse().unwrap();
                env.insert(name.clone(), value);
            }
            ASTNode::Out { expr } => {
                let value = evaluate(expr, &env);
                println!("{}", value);
            }
            _ => panic!("Unexpected AST node"),
        }
    }
}

fn evaluate(expr: &ASTNode, env: &HashMap<String, i64>) -> i64 {
    match expr {
        ASTNode::Number(n) => *n,
        ASTNode::Identifier(name) => *env.get(name).expect("Undefined variable"),
        ASTNode::Expression { left, right } => {
            let left_val = evaluate(left, env);
            let right_val = evaluate(right, env);
            left_val + right_val
        }
        ASTNode::Negate { expr } => -evaluate(expr, env),
        _ => panic!("Unexpected expression node"),
    }
}
