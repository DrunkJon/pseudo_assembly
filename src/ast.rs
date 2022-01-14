use crate::operator::Operator;
use crate::term::Terminal;

pub enum AstNode {
    Terminal(Terminal),
    NonTerminal(String, Vec<AstNode>),
}
