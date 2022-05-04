fn create<'a>(word: impl Into<&'a str>, distance: usize) -> crate::nfa::Spec<(usize, usize), u8> {
  let word: &'a str = word.into();
  let word = word.as_bytes();
  use crate::nfa::Spec;
  let mut spec: Spec<(usize, usize), u8> = crate::nfa::Spec::new((0, 0));
  for (idx, ch) in word.iter().enumerate() {
    let ch = *ch;
    for lv in 0..=distance {
      // normal
      spec.add_token_transition((lv, idx), ch, (lv, idx + 1));
      if lv + 1 <= distance {
        // insert
        spec.add_any_transition((lv, idx), (lv + 1, idx));
        // delete
        spec.add_epsilon_transition((lv, idx), (lv + 1, idx + 1));
      }
      if lv + 2 <= distance { // replace
        spec.add_epsilon_transition((lv, idx), (lv + 2, idx + 1));
      }
    }
    let length = word.len();
    for lv in 0..=distance {
      spec.add_accept_state((lv, length));
    }
  }
  spec
}

pub struct Machine {
  spec: crate::nfa::Spec<(usize, usize), u8>,
}

impl Machine {
  pub fn new<'a>(word: impl Into<&'a str>, distance: usize) -> Self {
    Self {
      spec: create(word, distance),
    }
  }
  pub fn contains<'a>(&self, word: impl Into<&'a str>) -> bool {
    let word = word.into();
    let word = word.as_bytes();
    let mut machine = crate::nfa::Machine::from_spec(self.spec.clone());
    for byte in word {
      let byte = *byte;
      machine.step(byte);
    }
    machine.in_accept_state()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use super::super::def::distance;

  #[test]
  fn basic() {
    assert_eq!(2, distance("zoi", "koi"));
    let m = Machine::new("zoi", 2);
    assert!(m.contains("zoi"));
    assert!(m.contains("koi"));
    assert!(!m.contains("ko"));
  }

  #[test]
  fn all_kind() {
    let m = Machine::new("java", 3);
    assert!(m.contains("ja"));
    assert!(m.contains("kav"));
    assert!(m.contains("kava"));
    assert!(!m.contains("kara"));
  }
}
