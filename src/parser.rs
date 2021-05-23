use super::ast::{concat, empty, literal, star, union, RegExp};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::satisfy;
use nom::combinator::map;
use nom::combinator::value;
use nom::error::Error;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::Finish;
use nom::IResult;

pub fn parse_regexp(input: &str) -> Result<RegExp, Error<&str>> {
  let (input, result) = nom::combinator::all_consuming(regexp)(input).finish()?;
  assert!(input.is_empty());
  Ok(result)
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
  use super::parse_regexp;
  use crate::ast::*;

  #[test]
  fn test_empty() {
    assert_eq!(parse_regexp("#"), Ok(empty()));
  }

  #[test]
  fn test_literal() {
    assert_eq!(parse_regexp("a"), Ok(literal('a')));
  }

  #[test]
  fn test_concat() {
    assert_eq!(parse_regexp("ab"), Ok(concat((literal('a'), literal('b')))));
  }

  #[test]
  fn test_union() {
    assert_eq!(parse_regexp("a|b"), Ok(union((literal('a'), literal('b')))));
  }

  #[test]
  fn test_star() {
    assert_eq!(parse_regexp("a*"), Ok(star(literal('a'))));
  }

  #[test]
  fn test_composite() {
    let expected = concat((
      star(union((literal('a'), literal('b')))),
      star(literal('c')),
    ));
    assert_eq!(parse_regexp("(a|b)*c*"), Ok(expected));
  }
}
