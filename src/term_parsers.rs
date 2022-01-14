use crate::term::{Ref, Terminal};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1, many_m_n},
    sequence::{preceded, terminated},
    Err, IResult,
};

fn hexadecimal_2(input: &str) -> IResult<&str, u8> {
    map_res(
        recognize(many1(one_of("0123456789abcdefABCDEF"))),
        |out: &str| u8::from_str_radix(out, 16),
    )(input)
}

fn decimal(input: &str) -> IResult<&str, u8> {
    map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
        u8::from_str_radix(out, 10)
    })(input)
}

fn immediate(input: &str) -> IResult<&str, Terminal> {
    map_res(
        alt((preceded(tag("0x"), hexadecimal_2), decimal)),
        |out: u8| -> Result<Terminal, Err<Terminal>> { Ok(Terminal::new_immediate(out)) },
    )(input)
}

fn direct_reference(input: &str) -> IResult<&str, Ref> {
    map_res(
        preceded(tag("#"), hexadecimal_2),
        |out: u8| -> Result<Ref, Err<Ref>> { Ok(Ref::DirectRef(out)) },
    )(input)
}

fn indirect_reference(input: &str) -> IResult<&str, Ref> {
    map_res(
        preceded(tag("@"), hexadecimal_2),
        |out: u8| -> Result<Ref, Err<Ref>> { Ok(Ref::IndirectRef(out)) },
    )(input)
}

pub fn reference(input: &str) -> IResult<&str, Ref> {
    alt((direct_reference, indirect_reference))(input)
}

pub fn terminal(input: &str) -> IResult<&str, Terminal> {
    alt((
        immediate,
        map_res(reference, |ref_: Ref| -> Result<Terminal, Err<Terminal>> {
            Ok(Terminal::Ref(ref_))
        }),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hexadecimal_2_test() {
        assert_eq!(hexadecimal_2("0"), Ok(("", 0)));
        assert_eq!(hexadecimal_2("FF"), Ok(("", 255)));
        assert_eq!(hexadecimal_2("10"), Ok(("", 16)));
        assert_eq!(hexadecimal_2("45"), Ok(("", 69)));
    }

    #[test]
    fn decimal_test() {
        assert_eq!(decimal("0"), Ok(("", 0)));
        assert_eq!(decimal("10"), Ok(("", 10)));
        assert_eq!(decimal("255"), Ok(("", 255)));
        assert!(decimal("-1").is_err());
        assert!(decimal("350").is_err());
    }

    #[test]
    fn immediate_test() {
        let expected = Terminal::new_immediate(15);
        let parsed = immediate("0x0F");
        assert_eq!(Ok(("", expected)), parsed);
    }

    #[test]
    fn direct_reference_test() {
        let expected = Ref::DirectRef(15);
        let parsed = direct_reference("#0F");
        assert_eq!(Ok(("", expected)), parsed);
    }

    #[test]
    fn indirect_reference_test() {
        let expected = Ref::IndirectRef(15);
        let parsed = indirect_reference("@0F");
        assert_eq!(Ok(("", expected)), parsed);
    }

    #[test]
    fn reference_test() {
        let expected = Ref::DirectRef(5);
        let (_, parsed) = reference("#5").unwrap();
        assert_eq!(expected, parsed, "\nfailed to parse direct reference\n");

        let expected = Ref::IndirectRef(5);
        let (_, parsed) = reference("@5").unwrap();
        assert_eq!(expected, parsed, "\nfailed to parse indirect reference\n");
    }

    #[test]
    fn terminal_test() {
        let expected = Terminal::Immediate(25);
        let (_, parsed) = terminal("25").unwrap();
        assert_eq!(expected, parsed, "\nfailed to parse dec imediate\n");

        let expected = Terminal::Immediate(50);
        let (_, parsed) = terminal("0x32").unwrap();
        assert_eq!(expected, parsed, "\nfailed to parse hex imediate\n");

        let expected = Terminal::Ref(Ref::DirectRef(5));
        let (_, parsed) = terminal("#5").unwrap();
        assert_eq!(expected, parsed, "\nfailed to direct ref as terminal\n");

        let expected = Terminal::Ref(Ref::IndirectRef(5));
        let (_, parsed) = terminal("@5").unwrap();
        assert_eq!(expected, parsed, "\nfailed to indirect ref as terminal\n");
    }
}
