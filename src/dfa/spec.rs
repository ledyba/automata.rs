use std::{collections::{HashSet, HashMap}, hash::Hash};

#[derive(Clone)]
pub struct Spec<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone,
{
  pub initial_state: Stat,
  pub all_states: HashSet<Stat>,
  pub accept_states: HashSet<Stat>,
  pub transitions: HashMap<(Stat, Token), Stat>,
}

pub struct SpecBuilder <Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone,
{
  initial_state: Stat,
  all_states: HashSet<Stat>,
  accept_states: HashSet<Stat>,
  transitions: HashMap<(Stat, Token), Stat>,
}

impl <Stat, Token> SpecBuilder<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone,
{
  pub fn new(initial: Stat) -> Self {
    Self {
      initial_state: initial.clone(),
      all_states: HashSet::from([initial]),
      accept_states: HashSet::new(),
      transitions: HashMap::new(),
    }
  }

  pub fn add_state(mut self, state: Stat) -> Self {
    self.all_states.insert(state);
    self
  }

  pub fn add_states<const N: usize>(mut self, states: [Stat; N]) -> Self {
    self.all_states.extend(states);
    self
  }

  pub fn add_accept_state(mut self, state: Stat) -> Self {
    self.all_states.insert(state.clone());
    self.accept_states.insert(state);
    self
  }

  pub fn add_accept_states<const N: usize>(mut self, states: [Stat; N]) -> Self {
    self = self.add_states(states.clone());
    self.accept_states.extend(states);
    self
  }

  pub fn add_transition(mut self, from: Stat, by: Token, to: Stat) -> Self {
    self = self.add_state(from.clone());
    self = self.add_state(to.clone());
    self.transitions.insert((from, by), to);
    self
  }

  pub fn build(self) -> Spec<Stat, Token> {
    if !self.accept_states.is_subset(&self.all_states) {
      panic!("BUG. Accept states is not subset of all states.")
    }
    if !self.all_states.contains(&self.initial_state) {
      panic!("BUG. The initial state is not an element of all states.")
    }
    let trans_keys: HashSet<Stat> = self.transitions.keys().map(|(k, _v)| k.clone()).collect();
    if !trans_keys.is_subset(&self.all_states) {
      panic!("BUG. All states in transition table is not subset of all states.")
    }
    Spec::<Stat, Token> {
      initial_state: self.initial_state,
      all_states: self.all_states,
      accept_states: self.accept_states,
      transitions: self.transitions,
    }
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
        .build();
    assert_eq!(0, spec.initial_state);
    assert!(spec.all_states.eq(&HashSet::from([0, 1, 2, 3, 4])));
    assert!(spec.accept_states.eq(&HashSet::from([3, 4])));
    assert!(spec.transitions.eq(&HashMap::from([
      ((0, '1'), 1),
      ((1, '2'), 2),
      ((2, '2'), 2),
      ((2, 'e'), 4),
    ])));
  }
}
