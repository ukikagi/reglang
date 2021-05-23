#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegExp {
  Empty(),
  Literal(char),
  Concat(Box<RegExp>, Box<RegExp>),
  Union(Box<RegExp>, Box<RegExp>),
  Star(Box<RegExp>),
}

pub fn empty() -> RegExp {
  RegExp::Empty()
}

pub fn literal(x: char) -> RegExp {
  RegExp::Literal(x)
}

pub fn concat((left, right): (RegExp, RegExp)) -> RegExp {
  RegExp::Concat(Box::new(left), Box::new(right))
}

pub fn union((left, right): (RegExp, RegExp)) -> RegExp {
  RegExp::Union(Box::new(left), Box::new(right))
}

pub fn star(re: RegExp) -> RegExp {
  RegExp::Star(Box::new(re))
}
