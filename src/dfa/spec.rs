use std::{collections::{HashSet, HashMap}, hash::Hash};

pub struct Spec<S, K> {
  pub initial: S,
  pub states: HashSet<S>,
  pub accept_states: HashSet<S>,
  pub transition: HashMap<(S, K), S>,
}

pub struct SpecBuilder <S, K> {
  initial: S,
  states: HashSet<S>,
  accept_states: HashSet<S>,
  transition: HashMap<(S, K), S>,
}

impl <S, K> SpecBuilder<S, K>
  where
    S: Eq + Hash + Clone,
    K: Eq + Hash + Clone,
{
  pub fn new(initial: S) -> Self {
    Self {
      initial: initial.clone(),
      states: HashSet::from([initial]),
      accept_states: HashSet::new(),
      transition: HashMap::new(),
    }
  }

  pub fn add_state(mut self, state: S) -> Self {
    self.states.insert(state);
    self
  }

  pub fn add_states<const N: usize>(mut self, states: [S; N]) -> Self {
    self.states.extend(states);
    self
  }

  pub fn add_accept_state(mut self, state: S) -> Self {
    self.states.insert(state.clone());
    self.accept_states.insert(state);
    self
  }

  pub fn add_accept_states<const N: usize>(mut self, states: [S; N]) -> Self {
    self = self.add_states(states.clone());
    self.accept_states.extend(states);
    self
  }

  pub fn add_transition(mut self, from: S, by: K, to: S) -> Self {
    self = self.add_state(from.clone());
    self = self.add_state(to.clone());
    self.transition.insert((from, by), to);
    self
  }

  pub fn build(self) -> anyhow::Result<Spec<S, K>> {
    Ok(Spec::<S, K> {
      initial: self.initial,
      states: self.states,
      accept_states: self.accept_states,
      transition: self.transition,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn build_spec() {
    let spec =
      SpecBuilder::<u32, char>::new(0)
        .add_state(1)
        .add_states([1, 2, 3])
        .add_accept_states([3, 4])
        .add_transition(0, '1', 1)
        .add_transition(1, '2', 2)
        .add_transition(2, '2', 2)
        .add_transition(2, 'e', 4)
        .build().expect("Failed to create spec");
    assert_eq!(0, spec.initial);
    assert!(spec.states.eq(&HashSet::from([0, 1, 2, 3, 4])));
    assert!(spec.accept_states.eq(&HashSet::from([3, 4])));
    assert!(spec.transition.eq(&HashMap::from([
      ((0, '1'), 1),
      ((1, '2'), 2),
      ((2, '2'), 2),
      ((2, 'e'), 4),
    ])));
  }
}
