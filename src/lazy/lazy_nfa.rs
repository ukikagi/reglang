use anyhow::Result;
use std::hash::Hash;

pub trait LazyNfa {
  type State: Eq + Hash + Ord + Clone;

  fn initial_state(&self) -> Vec<Self::State>;
  fn accepted(&self, s: &Self::State) -> bool;
  fn advance(&self, s: &Self::State, c: char) -> Result<Vec<Self::State>>;
}
