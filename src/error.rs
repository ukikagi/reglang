pub fn soft_assert<E>(cond: bool, e: E) -> std::result::Result<(), E> {
  if cond {
    Ok(())
  } else {
    Err(e)
  }
}
