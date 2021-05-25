use anyhow::Result;
use itertools::Itertools;
use std::hash::Hash;

trait LazyNfa {
  type State: Eq + Hash + Ord + Clone;

  fn initial_state(&self) -> Vec<Self::State>;
  fn accepted(&self, s: &Self::State) -> bool;
  fn advance(&self, s: &Self::State, c: char) -> Result<Vec<Self::State>>;
}

struct UnionNfa<L: LazyNfa, R: LazyNfa> {
  left: L,
  right: R,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
enum Sum<L, R> {
  Left(L),
  Right(R),
}

impl<L: LazyNfa, R: LazyNfa> LazyNfa for UnionNfa<L, R> {
  type State = Sum<L::State, R::State>;

  fn initial_state(&self) -> Vec<Self::State> {
    Iterator::chain(
      self
        .left
        .initial_state()
        .iter()
        .map(|x| Self::State::Left(x.clone())),
      self
        .right
        .initial_state()
        .iter()
        .map(|x| Self::State::Right(x.clone())),
    )
    .sorted()
    .dedup()
    .collect()
  }

  fn accepted(&self, s: &Self::State) -> bool {
    match s {
      Sum::Left(x) => self.left.accepted(x),
      Sum::Right(x) => self.right.accepted(x),
    }
  }

  fn advance(&self, s: &Self::State, c: char) -> Result<Vec<Self::State>> {
    let res = match s {
      Sum::Left(x) => self
        .left
        .advance(x, c)?
        .iter()
        .map(|x| Self::State::Left(x.clone()))
        .collect(),
      Sum::Right(x) => self
        .right
        .advance(x, c)?
        .iter()
        .map(|x| Self::State::Right(x.clone()))
        .collect(),
    };
    Ok(res)
  }
}
