use anyhow::{ensure, Result};

struct Monoid {
  size: usize,
  operation: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Monoid {
  pub fn new<Matrix: AsRef<[Row]>, Row: AsRef<[usize]>>(
    size: usize,
    matrix: Matrix,
  ) -> Result<Self> {
    let matrix = matrix.as_ref();
    ensure!(
      matrix.len() == size,
      "Number of matrix rows ({}) must match `size` ({})",
      matrix.len(),
      size
    );
    let mut operation = Vec::<Vec<usize>>::new();

    for row in matrix {
      let row = row.as_ref();
      ensure!(
        row.len() == size,
        "Number of matrix columns ({}) must match `size` ({})",
        row.len(),
        size
      );
      operation.push(row.to_vec());
    }
    Ok(Monoid { size, operation })
  }

  fn op(&self, x: usize, y: usize) -> Result<usize, String> {
    Ok(self.operation[x][y])
  }

  fn size(&self) -> usize {
    self.size
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic() {
    let monoid = Monoid::new(2, [[0, 1], [1, 0]]).unwrap();
    assert_eq!(monoid.op(1, 0), Ok(1))
  }

  #[test]
  fn test_missing_domain() {
    assert!(matches!(Monoid::new(1, [[0, 1], [1, 0]]), Err(_)));
  }

  #[test]
  fn test_missing_operation() {
    assert!(matches!(Monoid::new(2, [[0]]), Err(_)));
  }
}
