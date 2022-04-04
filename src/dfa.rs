use std::{collections::{HashSet, HashMap}, hash::Hash};

pub struct Definition<T> {
  pub initial: T,
  pub states: HashSet<T>,
  pub accept_states: HashSet<T>,
  pub transition: HashMap<T, T>,
}

impl <T> Definition <T>
where T: Eq + Hash + Clone
{
  pub fn new(initial: T, states_arr: &[T], accept_states_arr: &[T]) -> Self {
    let mut states = HashSet::new();
    let mut accept_states = HashSet::new();
    for state in states_arr {
      states.insert(state.clone());
    }
    for state in accept_states_arr {
      accept_states.insert(state.clone());
    }
    Self {
      initial,
      states,
      accept_states,
    }
  }
}
