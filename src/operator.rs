use crate::term::{Ref, Terminal};
use crate::Mem;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add(Ref, Terminal),
    Sub(Ref, Terminal),
    Mul(Ref, Terminal),
    Div(Ref, Terminal),
    Mov(Ref, Terminal),
    // Or(Ref, Terminal),
    // And(Ref, Terminal),
    // Xor(Ref, Terminal),
    // Not(Ref)
    // Nil(Ref) // sets memory to 0
    // In(Ref) // copies top of Input to memory
    // Out(Ref) // copies memory to bottom of Output
}

impl Operator {
    pub fn eval(&self, mem: &mut Mem) {
        match self {
            Self::Add(ref_, term_) => {
                let result = ref_.eval(mem).wrapping_add(term_.eval(mem));
                ref_.write(mem, result);
            },
            Self::Sub(ref_, term_) => {
                let result = ref_.eval(mem).wrapping_sub(term_.eval(mem));
                ref_.write(mem, result);
            },
            Self::Mul(ref_, term_) => {
                let result = ref_.eval(mem).wrapping_mul(term_.eval(mem));
                ref_.write(mem, result);
            },
            Self::Div(ref_, term_) => {
                let result = ref_.eval(mem) / term_.eval(mem);
                ref_.write(mem, result);
            },
            Self::Mov(ref_, term_) => {
                let result = term_.eval(mem);
                ref_.write(mem, result);
            },
            _ => {
                panic!("eval is not implemented for {:?}", self)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_eval_test() {
        let mut mem: Mem = [0; 256];
        let immediate = Terminal::Immediate(5);
        let d_ref = Ref::DirectRef(50);

        // ADD #32 5 --> mem[50] = 0 += 5
        let add1 = Operator::Add(d_ref, immediate);
        add1.eval(&mut mem);
        assert_eq!(mem[50], 5, "\nValue at direct reference is wrong\n");

        // ADD @32 #32  --> mem[mem[50]] = mem[5] = 0 += 5
        let id_ref = Ref::IndirectRef(50);
        let d_ref = Ref::DirectRef(50);
        let add2 = Operator::Add(id_ref, Terminal::Ref(d_ref));
        add2.eval(&mut mem);
        assert_eq!(mem[5], 5, "\nValue at indirect reference is wrong\n");

        // ADD #32 5 --> mem[50] = 5 += 5
        let d_ref = Ref::DirectRef(50);
        let immediate = Terminal::Immediate(5);
        let add3 = Operator::Add(d_ref, immediate);
        add3.eval(&mut mem);
        assert_eq!(
            mem[50], 10,
            "\nValue at direct reference is wrong after being changed a second time\n"
        );

        let mut expected_mem: Mem = [0; 256];
        expected_mem[5] = 5;
        expected_mem[50] = 10;

        assert_eq!(mem, expected_mem, "\nmem changed at unexpected adresses\n");
    }

    #[test]
    fn sub_eval_test() {
        let mut mem: Mem = [50; 256];
        let immediate = Terminal::Immediate(5);
        let d_ref = Ref::DirectRef(50);

        let sub = Operator::Sub(d_ref, immediate);
        sub.eval(&mut mem);
        assert_eq!(mem[50], 45, "\nValue at direct reference is wrong\n");
        sub.eval(&mut mem);
        assert_eq!(mem[50], 40, "\nValue at direct reference is wrong after second eval\n");
    }

    #[test]
    fn mul_eval_test() {
        let mut mem: Mem = [1; 256];
        let immediate = Terminal::Immediate(2);
        let d_ref = Ref::DirectRef(50);

        let mul = Operator::Mul(d_ref, immediate);
        mul.eval(&mut mem);
        assert_eq!(mem[50], 2, "\nValue at direct reference is wrong\n");
        mul.eval(&mut mem);
        assert_eq!(mem[50], 4, "\nValue at direct reference is wrong after second eval\n");
        mul.eval(&mut mem);
        assert_eq!(mem[50], 8, "\nValue at direct reference is wrong after third eval\n");
    }

    #[test]
    fn div_eval_test() {
        let mut mem: Mem = [10; 256];
        let immediate = Terminal::Immediate(2);
        let d_ref = Ref::DirectRef(50);

        let div = Operator::Div(d_ref, immediate);
        div.eval(&mut mem);
        assert_eq!(mem[50], 5, "\nValue at direct reference is wrong\n");
        div.eval(&mut mem);
        assert_eq!(mem[50], 2, "\nValue at direct reference is wrong after div with remainder eval\n");
    }

    #[test]
    fn mov_eval_test() {
        let mut mem: Mem = [0; 256];
        mem[5] = 5;
        let immediate = Terminal::Immediate(2);
        let d_ref = Ref::DirectRef(50);
        let d_ref2 = Ref::DirectRef(5);
        let id_ref = Ref::IndirectRef(50);

        let mov = Operator::Mov(d_ref, immediate);
        mov.eval(&mut mem);
        assert_eq!(mem[50], 2, "\nValue at direct reference is wrong\n");
        let mov = Operator::Mov(id_ref, immediate);
        mov.eval(&mut mem);
        assert_eq!(mem[2], 2, "\nValue at direct reference is wrong after div with remainder eval\n");
        let mov = Operator::Mov(id_ref, Terminal::Ref(d_ref2));
        mov.eval(&mut mem);
        assert_eq!(mem[2], 5, "\nValue at direct reference is wrong after div with remainder eval\n");
    }

    #[test]
    fn overflow_test() {
        let mut mem: Mem = [0; 256];
        let add255 = Operator::Add(Ref::DirectRef(0), Terminal::Immediate(255));
        let add1 = Operator::Add(Ref::DirectRef(0), Terminal::Immediate(1));
        add255.eval(&mut mem);
        assert_eq!(mem[0], 255);
        add1.eval(&mut mem);
        assert_eq!(mem[0], 0);
        // TODO add Tests for sub and Mul
    }
}
