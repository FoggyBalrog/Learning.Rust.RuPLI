#[derive(Debug)]
pub enum ASTNode {
    Assignment {
        name: String,
        expr: Box<ASTNode>,
    },
    In {
        name: String,
    },
    Out {
        expr: Box<ASTNode>,
    },
    Expression {
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Negate {
        expr: Box<ASTNode>,
    },
    Number(i64),
    Identifier(String),
}

impl ASTNode {
    pub fn get_representation(&self, indent: usize) -> String {
        let padding = " ".repeat(indent);
        match self {
            ASTNode::Assignment { name, expr } => {
                format!(
                    "{}Assignment: {} =\n{}",
                    padding,
                    name,
                    expr.get_representation(indent + 2)
                )
            }
            ASTNode::In { name } => format!("{}In: {}", padding, name),
            ASTNode::Out { expr } => {
                format!("{}Out:\n{}", padding, expr.get_representation(indent + 2))
            }
            ASTNode::Expression { left, right } => {
                let left_str = left.get_representation(indent + 2);
                let right_str = right.get_representation(indent + 2);
                format!(
                    "{}Expression:\n{}\n{}+\n{}",
                    padding,
                    left_str,
                    " ".repeat(indent + 2),
                    right_str
                )
            }
            ASTNode::Negate { expr } => format!(
                "{}Negate:\n{}",
                padding,
                expr.get_representation(indent + 2)
            ),
            ASTNode::Number(n) => format!("{}Number: {}", padding, n),
            ASTNode::Identifier(name) => format!("{}Identifier: {}", padding, name),
        }
    }
}

pub trait ASTDisplay {
    fn display(&self);
}

impl ASTDisplay for Vec<ASTNode> {
    fn display(&self) {
        let str = self.iter()
            .map(|node| node.get_representation(0))
            .collect::<Vec<_>>()
            .join("\n");

        println!("\n");
        println!("{}", str);
        println!("\n");
    }
}
