use anyhow::{ensure, Result};
use std::collections::{HashMap, HashSet};

pub struct State {
  accepted: bool,
  transitions: HashMap<char, usize>,
}

#[allow(dead_code)]
impl State {
  pub fn new(accepted: bool, transitions: HashMap<char, usize>) -> State {
    State {
      accepted,
      transitions,
    }
  }
}

pub struct Dfa {
  alphabet: HashSet<char>,
  states: Vec<State>,
}

#[allow(dead_code)]
impl Dfa {
  pub fn new(alphabet: HashSet<char>, states: Vec<State>) -> Result<Self> {
    let dfa = Dfa { alphabet, states };
    dfa.check_invariant()?;
    Ok(dfa)
  }

  fn check_invariant(&self) -> Result<()> {
    ensure!(self.size() > 0);
    for state in &self.states {
      for (c, next_state) in &state.transitions {
        ensure!(self.alphabet.contains(&c));
        ensure!(next_state < &self.size());
      }
      for c in &self.alphabet {
        // Requiring totality
        ensure!(state.transitions.contains_key(&c));
      }
    }
    Ok(())
  }

  fn size(&self) -> usize {
    self.states.len()
  }

  fn initial_state(&self) -> usize {
    0
  }

  fn advance(&self, state: usize, c: char) -> usize {
    assert!(self.alphabet.contains(&c));
    self.states[state].transitions.get(&c).unwrap().clone()
  }

  fn is_accepted(&self, state: usize) -> bool {
    self.states[state].accepted
  }

  pub fn run(&self, input: &str) -> Result<bool> {
    let mut state = self.initial_state();
    for c in input.chars() {
      ensure!(self.alphabet.contains(&c));
      state = self.advance(state, c);
    }
    Ok(self.is_accepted(state))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use maplit::{hashmap, hashset};

  #[test]
  fn test_run() {
    let alphabet = hashset!['0', '1'];
    let states = vec![
      State::new(true, hashmap!['0' => 0, '1' => 1]),
      State::new(false, hashmap!['0' => 2, '1' => 0]),
      State::new(false, hashmap!['0' => 1, '1' => 2]),
    ];
    let dfa = Dfa::new(alphabet, states).unwrap();

    assert_eq!(dfa.size(), 3);

    assert!(matches!(dfa.run("1"), Ok(false)));
    assert!(matches!(dfa.run("10"), Ok(false)));
    assert!(matches!(dfa.run("11"), Ok(true)));
    assert!(matches!(dfa.run("000"), Ok(true)));
  }

  #[test]
  fn test_run_fail() {
    let alphabet = hashset!['0', '1'];
    let states = vec![
      State::new(true, hashmap!['0' => 0, '1' => 1]),
      State::new(false, hashmap!['0' => 2, '1' => 0]),
      State::new(false, hashmap!['0' => 1, '1' => 2]),
    ];
    let dfa = Dfa::new(alphabet, states).unwrap();

    assert!(matches!(dfa.run("00a1"), Err(_)));
  }
}
