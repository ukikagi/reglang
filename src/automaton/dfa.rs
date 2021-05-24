use crate::error::soft_assert;
use std::collections::HashMap;

struct Dfa {
  size: usize,
  initial_state: usize,
  accepted: Vec<bool>,
  transitions: HashMap<char, Vec<usize>>,
}

#[allow(dead_code)]
impl Dfa {
  pub fn new(
    size: usize,
    initial_state: usize,
    accepted: Vec<bool>,
    transitions: HashMap<char, Vec<usize>>,
  ) -> Result<Self, String> {
    soft_assert(
      initial_state < size,
      format!(
        "'initial_state' is {}; must be smaller than {}",
        initial_state, size
      ),
    )?;
    soft_assert(
      accepted.len() == size,
      format!(
        "The length of 'accepted' is {}; must be {}",
        accepted.len(),
        size
      ),
    )?;
    for (key, value) in &transitions {
      soft_assert(
        value.len() == size,
        format!(
          "The transition rule length for the character {} is {}; must be {}",
          key,
          value.len(),
          size
        ),
      )?;
    }

    Ok(Dfa {
      size,
      initial_state,
      accepted,
      transitions,
    })
  }

  pub fn run(&self, input: &str) -> Result<bool, String> {
    let mut state = self.initial_state;
    for c in input.chars() {
      state = self
        .transitions
        .get(&c)
        .ok_or(format!("'{0}' is not found in transitions.", c))?[state];
    }
    Ok(self.accepted[state])
  }

  pub fn size(&self) -> usize {
    self.size
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;

  fn tr(array: &[(char, Vec<usize>)]) -> HashMap<char, Vec<usize>> {
    array.iter().cloned().collect()
  }

  #[test]
  fn test_run() {
    let dfa = Dfa::new(
      3,
      0,
      vec![true, false, false],
      tr(&[('0', vec![0, 2, 1]), ('1', vec![1, 0, 2])]),
    )
    .unwrap();

    assert_eq!(dfa.size(), 3);

    assert_eq!(dfa.run("1"), Ok(false));
    assert_eq!(dfa.run("10"), Ok(false));
    assert_eq!(dfa.run("11"), Ok(true));
    assert_eq!(dfa.run("000"), Ok(true));
  }

  #[test]
  fn test_new_fail() {
    assert!(matches!(
      Dfa::new(
        3,
        4,
        vec![true, false, false],
        tr(&[('0', vec![0, 2, 1]), ('1', vec![1, 0, 2])]),
      ),
      Err(_)
    ));

    assert!(matches!(
      Dfa::new(
        3,
        0,
        vec![true, false],
        tr(&[('0', vec![0, 2, 1]), ('1', vec![1, 0, 2])]),
      ),
      Err(_)
    ));

    assert!(matches!(
      Dfa::new(
        3,
        4,
        vec![true, false, false],
        tr(&[('0', vec![0, 2, 1]), ('1', vec![1, 0, 2, 3])]),
      ),
      Err(_)
    ));
  }

  #[test]
  fn test_run_fail() {
    let dfa = Dfa::new(
      3,
      0,
      vec![true, false, false],
      tr(&[('0', vec![0, 2, 1]), ('1', vec![1, 0, 2])]),
    )
    .unwrap();

    assert!(matches!(dfa.run("00a1"), Err(_)));
  }
}
