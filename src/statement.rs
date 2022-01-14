use crate::operator::Operator;
use crate::Mem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Line(Operator),
    Block(Vec<Statement>)
}


impl Statement {
    fn eval(&self, mem: &mut Mem) {
        match self {
            Self::Line(op) => op.eval(mem),
            Self::Block(statements) => {
                for s in statements {
                    s.eval(mem)
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::statement_parsers::{block, statement};

    #[test]
    fn statement_eval_test() {
        let mut mem: Mem = [0; 256];
        let (_, s) = statement("Add #00 0xFF").unwrap();
        let block_input = 
        "
        Mov #01 #00
        DIV #01 10
        ADD @01 1
        MUL @01 45
        ";
        let (_, b) = block(block_input).unwrap();
        s.eval(&mut mem);
        b.eval(&mut mem);
        assert_eq!(mem[0], 255);
        assert_eq!(mem[1], 25);
        assert_eq!(mem[25], 45);
        for i in 0..255 {
            if i > 1 && i != 25 {
                assert_eq!(mem[i], 0);
            }
        }
    }
}