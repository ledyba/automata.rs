use std::{collections::{HashSet, HashMap}, hash::Hash};

pub struct Spec<T> {
  pub initial: T,
  pub states: HashSet<T>,
  pub accept_states: HashSet<T>,
  pub transition: HashMap<T, T>,
}

pub struct SpecBuilder <T> {
  pub initial: T,
  pub states: HashSet<T>,
  pub accept_states: HashSet<T>,
  pub transition: HashMap<T, T>,
}

impl <T> SpecBuilder<T>
  where T: Eq + Hash + Clone
{
  pub fn new(initial: &T) -> Self {
    Self {
      initial: initial.clone(),
      states: HashSet::from([initial.clone()]),
      accept_states: HashSet::new(),
      transition: HashMap::new(),
    }
  }

  pub fn add_state(&mut self, state: T) -> &mut Self {
    self.states.insert(state);
    self
  }

  pub fn add_states(&mut self, states: &[T]) -> &mut Self {
    self.states.extend(states.iter().map(|it| it.clone()));
    self
  }

  pub fn add_accept_state(&mut self, state: T) -> &mut Self {
    self.states.insert(state.clone());
    self.accept_states.insert(state);
    self
  }

  pub fn add_accept_states(&mut self, states: &[T]) -> &mut Self {
    self.add_states(states);
    self.accept_states.extend(states.iter().map(|it| it.clone()));
    self
  }

  pub fn add_transition(&mut self, from: T, to: T) -> &mut Self {
    self.add_state(from.clone());
    self.add_state(to.clone());
    self.transition.insert(from, to);
    self
  }

  pub fn build(self) -> Spec<T> {
    Spec::<T> {
      initial: self.initial,
      states: self.states,
      accept_states: self.accept_states,
      transition: self.transition,
    }
  }
}
