use anyhow::{ensure, Result};
use std::collections::{HashMap, HashSet};

pub struct State {
  initial: bool,
  accepted: bool,
  transitions: HashMap<Option<char>, HashSet<usize>>,
}

impl State {
  pub fn new() -> Self {
    State {
      initial: false,
      accepted: false,
      transitions: HashMap::new(),
    }
  }
}

type SubsetState = HashSet<usize>;

pub struct Nfa {
  alphabet: HashSet<char>,
  states: Vec<State>,
}

#[allow(dead_code)]
impl Nfa {
  pub fn new(alphabet: HashSet<char>, states: Vec<State>) -> Result<Self> {
    let nfa = Nfa { alphabet, states };
    nfa.check_invariant()?;
    Ok(nfa)
  }

  fn check_invariant(&self) -> Result<()> {
    for state in &self.states {
      for (key, next_states) in &state.transitions {
        if let Some(c) = key {
          ensure!(self.alphabet.contains(&c));
        }
        ensure!(next_states.iter().all(|&x| x < self.size()));
      }
    }
    Ok(())
  }

  fn size(&self) -> usize {
    self.states.len()
  }

  fn add_transition(&mut self, c: Option<char>, x: usize, y: usize) -> Result<()> {
    if let Some(x) = c {
      ensure!(self.alphabet.contains(&x));
    }
    ensure!(x < self.size() && y < self.size());
    let next_state = self.states[x]
      .transitions
      .entry(c)
      .or_insert_with(|| HashSet::new());
    ensure!(next_state.insert(y));
    self.check_invariant()
  }

  fn add_state(&mut self) -> Result<&mut State> {
    self.states.push(State::new());
    self.check_invariant()?;
    Ok(self.states.last_mut().unwrap())
  }

  fn initial_states(&self) -> SubsetState {
    (0..self.size())
      .filter(|&x| self.states[x].initial)
      .collect()
  }

  fn advance(&self, states: &SubsetState, c: char) -> SubsetState {
    // TODO: include epsilon transitions
    assert!(self.alphabet.contains(&c));
    states
      .iter()
      .filter_map(|&s| self.states[s].transitions.get(&Some(c)))
      .flatten()
      .cloned()
      .collect()
  }

  fn is_accepted(&self, states: &SubsetState) -> bool {
    states.iter().any(|&i| self.states[i].accepted)
  }

  fn resolve_eps(&mut self) -> Result<()> {
    unimplemented!()
  }

  pub fn run(&self, input: &str) -> Result<bool> {
    let mut states = self.initial_states();

    for c in input.chars() {
      ensure!(self.alphabet.contains(&c));
      states = self.advance(&states, c);
    }
    Ok(self.is_accepted(&states))
  }
}
