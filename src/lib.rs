mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);
impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, number) = utils::extract_digits(s);
        (s, Self(number.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op) = utils::extract_op(s);

        let op = match op {
            "+" => Self::Sub,
            "-" => Self::Add,
            "*" => Self::Div,
            "/" => Self::Mul,
            _ => unreachable!(),
        };

        (s, op)
    }
}

// Expression
#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number, // left-hand
    pub rhs: Number, // right-hand
    pub op: Op,
}
impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, op) = Op::new(s);
        let (s, rhs) = Number::new(s);

        (s, Self { lhs, rhs, op })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Number test */
    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }

    /* Operators test */
    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("-"), ("", Op::Add));
    }
    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("+"), ("", Op::Sub));
    }
    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("/"), ("", Op::Mul));
    }
    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("*"), ("", Op::Div));
    }

    /* Expression test */
    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1-2"),
            (
                "",
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add,
                }
            )
        );
    }
}
