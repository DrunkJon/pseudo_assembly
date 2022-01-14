use crate::operator::Operator;
use crate::term::{Ref, Terminal};
use crate::term_parsers::{reference, terminal};
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{multispace0, multispace1},
    combinator::map_res,
    error::{Error, ErrorKind, FromExternalError},
    sequence::{delimited, preceded, tuple},
    multi::{many1},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
enum OperatorEnum {
    Add,
    Sub,
    Mul,
    Div,
    Mov,
}

impl OperatorEnum {
    fn tag(&self) -> &str {
        match self {
            Self::Add => "ADD",
            Self::Sub => "SUB",
            Self::Mul => "MUL",
            Self::Div => "DIV",
            Self::Mov => "MOV",
            _x => panic!("tag is not implementet for {:?}", _x),
        }
    }

    fn build(&self, ref_: Ref, term_: Terminal) -> Operator {
        match self {
            Self::Add => Operator::Add(ref_, term_),
            Self::Sub => Operator::Sub(ref_, term_),
            Self::Mul => Operator::Mul(ref_, term_),
            Self::Div => Operator::Div(ref_, term_),
            Self::Mov => Operator::Mov(ref_, term_),
            _x => panic!("build is not implementet for {:?}", _x),
        }
    }
}

fn operator_map<'a>(
    op: &'a OperatorEnum,
) -> impl FnMut(&'a str) -> IResult<&str, Operator> {
    map_res(
        delimited(
            multispace0,
            preceded(
                tag_no_case(op.tag()),
                tuple((
                    preceded(multispace1, reference),
                    preceded(multispace1, terminal),
                )),
            ),
            multispace0,
        ),
        move |input: (Ref, Terminal)| -> Result<Operator, Error<Operator>> {
            Ok(op.build(input.0, input.1))
        },
    )
}

pub fn operator<'a>(input: &'a str) -> IResult<&'a str, Operator> {
    use OperatorEnum::*;
    alt((
        operator_map(&Add),
        operator_map(&Sub),
        operator_map(&Mul),
        operator_map(&Div),
        operator_map(&Mov),
    ))(input)
}

fn statement(input: &str) -> IResult<&str, Vec<Operator>> {
    many1(operator)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::term::{Ref, Terminal};

    fn generic_operator_template(op: OperatorEnum) {
        let expected = op.build(Ref::DirectRef(50), Terminal::Immediate(255));
        let input = format!(" {} #32 0xFF \n", op.tag().to_uppercase());
        let (_, result) = operator_map(&op)(
            input.as_str(),
        )
        .unwrap();
        assert_eq!(
            expected,
            result,
            "\nfailed to parse '{} #32 0xFF'\n",
            op.tag().to_uppercase()
        );

        let expected = op.build(Ref::IndirectRef(50), Terminal::Immediate(255));
        let input = format!(" {} @32 0xFF \n", op.tag().to_lowercase());
        let (_, result) = operator_map(&op)(
            input.as_str(),
        )
        .unwrap();
        assert_eq!(
            expected,
            result,
            "\nfailed to parse '{} @32 0xFF'\n",
            op.tag().to_lowercase()
        );
    }

    #[test]
    fn operator_test() {
        assert!(!operator("ADD #FF 69").is_err());
        assert!(!operator("SUB #FF 69").is_err());
        assert!(!operator("MUL #FF 69").is_err());
        assert!(!operator("DIV #FF 69").is_err());
        assert!(!operator("MOV #FF 69").is_err());
    }

    #[test]
    fn operator_map_test() {
        use OperatorEnum::*;

        let (_, result) = operator_map(&Add)("ADD #FF 69").unwrap();
        let expected = Operator::Add(Ref::DirectRef(255), Terminal::Immediate(69));
        assert_eq!(result, expected);

        let (_, result) = operator_map(&Sub)("Sub @0F #00").unwrap();
        let expected = Operator::Sub(Ref::IndirectRef(15), Terminal::Ref(Ref::DirectRef(0)));
        assert_eq!(result, expected);

        let (_, result) = operator_map(&Mul)("  Mul  #FF 69  \n   ").unwrap();
        let expected = Operator::Mul(Ref::DirectRef(255), Terminal::Immediate(69));
        assert_eq!(result, expected);

        let (_, result) = operator_map(&Div)("diV #FF 69").unwrap();
        let expected = Operator::Div(Ref::DirectRef(255), Terminal::Immediate(69));
        assert_eq!(result, expected);

        let (_, result) = operator_map(&Mov)("Mov #FF @FF").unwrap();
        let expected = Operator::Mov(Ref::DirectRef(255), Terminal::Ref(Ref::IndirectRef(255)));
        assert_eq!(result, expected);
    }

    #[test]
    fn automatic_operator_map() {
        use OperatorEnum::*;

        generic_operator_template(Add);
        generic_operator_template(Sub);
        generic_operator_template(Mul);
        generic_operator_template(Div);
        generic_operator_template(Mov);
    }
}
