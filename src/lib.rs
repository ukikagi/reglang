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

fn concat(left: RegExp, right: RegExp) -> RegExp {
  RegExp::Concat(Box::new(left), Box::new(right))
}

fn union(left: RegExp, right: RegExp) -> RegExp {
  RegExp::Union(Box::new(left), Box::new(right))
}

fn star(re: RegExp) -> RegExp {
  RegExp::Star(Box::new(re))
}

fn parse(source: &str) -> IResult<&str, RegExp> {
  Ok((source, empty()))
}

#[cfg(test)]
mod tests {
  use crate::*;

  fn fully_parsed(re: RegExp) -> IResult<&'static str, RegExp> {
    Ok(("", re))
  }

  #[test]
  fn test_empty_set() {
    assert_eq!(parse("#"), fully_parsed(empty()));
  }

  #[test]
  fn test_literal() {
    assert_eq!(parse("#"), fully_parsed(literal('a')));
  }

  #[test]
  fn test_concat() {
    assert_eq!(
      parse("ab"),
      fully_parsed(concat(literal('a'), literal('b')))
    );
  }

  #[test]
  fn test_union() {
    assert_eq!(
      parse("a|b"),
      fully_parsed(union(literal('a'), literal('b')))
    );
  }

  #[test]
  fn test_star() {
    assert_eq!(parse("a*"), fully_parsed(star(literal('a'))));
  }
}
