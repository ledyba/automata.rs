pub struct Spec {
  distance: usize,
  spec: crate::nfa::Spec<String, char>,
}

impl Spec {
  pub fn new(distance: usize) -> Self {
    Self {
      distance,
      spec: crate::nfa::Spec::new("".to_string()),
    }
  }
  pub fn add_word<'a>(&mut self, word: impl Into<&'a str>) {
    let word = word.into();
    for (idx, ch) in word.chars().enumerate() {

    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn basic() {
  }
}