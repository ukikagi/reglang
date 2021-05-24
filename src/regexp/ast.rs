use itertools::Itertools;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegExp {
  Empty,
  Unit,
  Literal(char),
  Concat(Vec<RegExp>),
  Union(Vec<RegExp>),
  Star(Box<RegExp>),
}

impl fmt::Display for RegExp {
  // TODO: make it prettier
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use RegExp::*;
    match self {
      Empty => write!(f, "#"),
      Unit => write!(f, "()"),
      Literal(c) => write!(f, "{}", c),
      Concat(rs) => {
        let formatter = rs
          .iter()
          .format_with("", |elt, f| f(&format_args!("({})", elt)));
        write!(f, "{}", formatter)
      }
      Union(rs) => {
        let formatter = rs
          .iter()
          .format_with("|", |elt, f| f(&format_args!("({})", elt)));
        write!(f, "{}", formatter)
      }
      Star(r) => write!(f, "({})*", r.as_ref()),
    }
  }
}

pub fn empty() -> RegExp {
  RegExp::Empty
}

pub fn unit() -> RegExp {
  RegExp::Unit
}

pub fn literal(x: char) -> RegExp {
  RegExp::Literal(x)
}

pub fn concat<Rs: AsRef<[RegExp]>>(res: Rs) -> RegExp {
  let res = res.as_ref();
  match res.len() {
    0 => RegExp::Unit,
    1 => res[0].clone(),
    _ => RegExp::Concat(res.to_vec()),
  }
}

pub fn union<Rs: AsRef<[RegExp]>>(res: Rs) -> RegExp {
  let res = res.as_ref();
  match res.len() {
    0 => RegExp::Empty,
    1 => res[0].clone(),
    _ => RegExp::Union(res.to_vec()),
  }
}

pub fn star(re: RegExp) -> RegExp {
  RegExp::Star(Box::new(re))
}
