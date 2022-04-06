use std::{collections::{HashSet, HashMap}, hash::Hash};

#[derive(Clone)]
pub struct Spec<Stat, Tran>
  where
    Stat: Eq + Hash + Clone,
    Tran: Eq + Hash + Clone,
{
  pub initial: Stat,
  pub all_states: HashSet<Stat>,
  pub accept_states: HashSet<Stat>,
  pub transitions: HashMap<(Stat, Tran), Stat>,
}

pub struct SpecBuilder <Stat, Tran>
  where
    Stat: Eq + Hash + Clone,
    Tran: Eq + Hash + Clone,
{
  initial: Stat,
  states: HashSet<Stat>,
  accept_states: HashSet<Stat>,
  transition: HashMap<(Stat, Tran), Stat>,
}

impl <Stat, Tran> SpecBuilder<Stat, Tran>
  where
    Stat: Eq + Hash + Clone,
    Tran: Eq + Hash + Clone,
{
  pub fn new(initial: Stat) -> Self {
    Self {
      initial: initial.clone(),
      states: HashSet::from([initial]),
      accept_states: HashSet::new(),
      transition: HashMap::new(),
    }
  }

  pub fn add_state(mut self, state: Stat) -> Self {
    self.states.insert(state);
    self
  }

  pub fn add_states<const N: usize>(mut self, states: [Stat; N]) -> Self {
    self.states.extend(states);
    self
  }

  pub fn add_accept_state(mut self, state: Stat) -> Self {
    self.states.insert(state.clone());
    self.accept_states.insert(state);
    self
  }

  pub fn add_accept_states<const N: usize>(mut self, states: [Stat; N]) -> Self {
    self = self.add_states(states.clone());
    self.accept_states.extend(states);
    self
  }

  pub fn add_transition(mut self, from: Stat, by: Tran, to: Stat) -> Self {
    self = self.add_state(from.clone());
    self = self.add_state(to.clone());
    self.transition.insert((from, by), to);
    self
  }

  pub fn build(self) -> Spec<Stat, Tran> {
    Spec::<Stat, Tran> {
      initial: self.initial,
      all_states: self.states,
      accept_states: self.accept_states,
      transitions: self.transition,
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
    assert_eq!(0, spec.initial);
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
