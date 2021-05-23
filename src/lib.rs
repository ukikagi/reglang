use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::satisfy;
use nom::combinator::map;
use nom::combinator::value;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegExp {
  Empty(),
  Literal(char),
  Concat(Box<RegExp>, Box<RegExp>),
  Union(Box<RegExp>, Box<RegExp>),
  Star(Box<RegExp>),
}

fn empty() -> RegExp {
  RegExp::Empty()
}

fn literal(x: char) -> RegExp {
  RegExp::Literal(x)
}

fn concat((left, right): (RegExp, RegExp)) -> RegExp {
  RegExp::Concat(Box::new(left), Box::new(right))
}

fn union((left, right): (RegExp, RegExp)) -> RegExp {
  RegExp::Union(Box::new(left), Box::new(right))
}

fn star(re: RegExp) -> RegExp {
  RegExp::Star(Box::new(re))
}

fn regexp(input: &str) -> IResult<&str, RegExp> {
  unionexp(input)
}

fn unionexp(input: &str) -> IResult<&str, RegExp> {
  alt((
    map(separated_pair(interexp, tag("|"), unionexp), union),
    interexp,
  ))(input)
}

fn interexp(input: &str) -> IResult<&str, RegExp> {
  // TODO: implement intersection
  concatexp(input)
}

fn concatexp(input: &str) -> IResult<&str, RegExp> {
  alt((map(pair(repeatexp, concatexp), concat), repeatexp))(input)
}

fn repeatexp(input: &str) -> IResult<&str, RegExp> {
  // Fow now a pattern like a+* is prohibited unlike dk.brics.automaton for simplicity
  // cf. https://www.brics.dk/automaton/doc/dk/brics/automaton/RegExp.html
  alt((map(terminated(complexp, tag("*")), star), complexp))(input)
}

fn complexp(input: &str) -> IResult<&str, RegExp> {
  // TODO: implement complement
  charclassexp(input)
}

fn charclassexp(input: &str) -> IResult<&str, RegExp> {
  // TODO: implement charactor class
  simpleexp(input)
}

fn simpleexp(input: &str) -> IResult<&str, RegExp> {
  // TODO: implement other basic constructions e.g.
  // . : any single charactor
  // @ : any string
  // () : empty string
  alt((
    charexp,
    value(empty(), char('#')),
    delimited(tag("("), unionexp, tag(")")),
  ))(input)
}

fn charexp(input: &str) -> IResult<&str, RegExp> {
  // TODO: support more characters including escaped ones
  map(satisfy(|c: char| c.is_ascii_lowercase()), literal)(input)
}

#[cfg(test)]
mod tests {
  use crate::*;

  fn fully_parsed(re: RegExp) -> IResult<&'static str, RegExp> {
    Ok(("", re))
  }

  #[test]
  fn test_empty() {
    assert_eq!(simpleexp("#"), fully_parsed(empty()));
  }

  #[test]
  fn test_literal() {
    assert_eq!(simpleexp("a"), fully_parsed(literal('a')));
  }

  #[test]
  fn test_concat() {
    assert_eq!(
      concatexp("ab"),
      fully_parsed(concat((literal('a'), literal('b'))))
    );
  }

  #[test]
  fn test_union() {
    assert_eq!(
      unionexp("a|b"),
      fully_parsed(union((literal('a'), literal('b'))))
    );
  }

  #[test]
  fn test_star() {
    assert_eq!(repeatexp("a*"), fully_parsed(star(literal('a'))));
  }
}
