use crate::Mem;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Terminal {
    Ref(Ref),
    Immediate(u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ref {
    DirectRef(u8),
    IndirectRef(u8),
}

impl Ref {
    pub fn write(&self, mem: &mut Mem, val: u8) -> () {
        match self {
            Self::DirectRef(adr) => {
                mem[adr.clone() as usize] = val;
            }
            Self::IndirectRef(adr) => {
                mem[mem[adr.clone() as usize] as usize] = val;
            }
        }
    }

    pub fn eval(&self, mem: &Mem) -> u8 {
        match self {
            Self::DirectRef(adr) => mem[adr.clone() as usize],
            Self::IndirectRef(adr) => mem[mem[adr.clone() as usize] as usize],
        }
    }
}

impl Terminal {
    pub fn new_d_ref(adr: u8) -> Self {
        Self::Ref(Ref::DirectRef(adr))
    }

    pub fn new_id_ref(adr: u8) -> Self {
        Self::Ref(Ref::IndirectRef(adr))
    }

    pub fn new_immediate(val: u8) -> Self {
        Self::Immediate(val)
    }

    pub fn eval(&self, mem: &Mem) -> u8 {
        match self {
            Self::Ref(ref_) => ref_.eval(mem),
            Self::Immediate(val) => val.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn Terminal_eval() {
        let mut mem: [u8; 256] = [0; 256];
        mem[55] = 5;
        mem[25] = 55;
        let d_ref0 = Terminal::new_d_ref(0);
        assert_eq!(d_ref0.eval(&mem), 0);
        let d_ref5 = Terminal::new_d_ref(55);
        assert_eq!(d_ref5.eval(&mem), 5);
        let i_ref0 = Terminal::new_id_ref(10);
        assert_eq!(i_ref0.eval(&mem), 0);
        let i_ref5 = Terminal::new_id_ref(25);
        assert_eq!(i_ref5.eval(&mem), 5);
    }
}
