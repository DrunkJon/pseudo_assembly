use crate::operator_parsers::operator;
use crate::statement::Statement;
use nom::{
    branch::alt,
    multi::many1,
    IResult,
};

pub fn statement(input: &str) -> IResult<&str, Statement> {
    match operator(input) {
        Ok((out, op)) => Ok((out, Statement::Line(op))),
        Err(e) => Err(e),
    }
}

pub fn block(input: &str) -> IResult<&str, Statement> {
    match many1(statement)(input) {
        Ok((out, s)) => Ok((out, Statement::Block(s))),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::operator::Operator;
    use crate::term::{Ref, Terminal};

    #[test]
    fn block_test() {
        let input = 
        "
        ADD #00 1
        SUB #01 3
        MUL #00 5
        Div #01 5
        MOV @00 #00
        ";
        let (rest, result) = block(input).unwrap();
        if let Statement::Block(result_vec) = result {
            assert_eq!(result_vec[0], Statement::Line(Operator::Add(Ref::DirectRef(0), Terminal::Immediate(1))));
            assert_eq!(result_vec[1], Statement::Line(Operator::Sub(Ref::DirectRef(1), Terminal::Immediate(3))));
            assert_eq!(result_vec[2], Statement::Line(Operator::Mul(Ref::DirectRef(0), Terminal::Immediate(5))));
            assert_eq!(result_vec[3], Statement::Line(Operator::Div(Ref::DirectRef(1), Terminal::Immediate(5))));
            assert_eq!(result_vec[4], Statement::Line(Operator::Mov(Ref::IndirectRef(0), Terminal::Ref(Ref::DirectRef(0)))));
        } else {
            panic!("block parser returned ({}, {:?}) instead of block", rest, result);
        }
    }
}