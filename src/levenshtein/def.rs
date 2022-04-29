pub fn distance<'a>(a: impl Into<&'a str>, b: impl Into<&'a str>) -> usize {
  let a = a.into();
  let b = b.into();
  0
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn basic() {
    assert_eq!(2, distance("zoi", "koi"))
  }
}