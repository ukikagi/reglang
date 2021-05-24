use crate::error::soft_assert;

struct Monoid {
  size: usize,
  operation: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Monoid {
  pub fn new<Matrix: AsRef<[Row]>, Row: AsRef<[usize]>>(
    size: usize,
    matrix: Matrix,
  ) -> Result<Self, String> {
    let matrix = matrix.as_ref();
    soft_assert(matrix.len() == size, format!(""))?;
    let mut operation = Vec::<Vec<usize>>::new();

    for row in matrix {
      let row = row.as_ref();
      soft_assert(row.len() == size, format!(""))?;
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
