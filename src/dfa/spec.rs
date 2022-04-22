use std::{collections::{HashSet, HashMap}, hash::Hash};

#[derive(Clone)]
pub struct Spec<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone,
{
  initial_state: Stat,
  all_states: HashSet<Stat>,
  accept_states: HashSet<Stat>,
  transitions: HashMap<(Stat, Token), Stat>,
}

impl <Stat, Token> Spec<Stat, Token>
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

  pub fn add_state(&mut self, state: Stat) -> &mut Self {
    self.all_states.insert(state);
    self
  }

  pub fn add_states<const N: usize>(&mut self, states: [Stat; N]) -> &mut Self {
    self.all_states.extend(states);
    self
  }

  pub fn add_accept_state(&mut self, state: Stat) -> &mut Self {
    self.all_states.insert(state.clone());
    self.accept_states.insert(state);
    self
  }

  pub fn add_accept_states<const N: usize>(&mut self, states: [Stat; N]) -> &mut Self {
    self.add_states(states.clone());
    self.accept_states.extend(states);
    self
  }

  pub fn add_transition(&mut self, from: Stat, by: Token, to: Stat) -> &mut Self {
    self.add_state(from.clone());
    self.add_state(to.clone());
    self.transitions.insert((from, by), to);
    self
  }

  pub fn initial_state(&self) -> Stat {
    self.initial_state.clone()
  }

  pub fn transition_of(&self, state: Stat, token: Token) -> Option<Stat> {
    self.transitions.get(&(state, token)).map(|it| it.clone())
  }

  pub fn has_transition(&self, state: Stat, token: Token) -> bool {
    self.transitions.contains_key(&(state, token))
  }

  pub fn is_acceptable_state(&self, state: &Stat) -> bool {
    self.accept_states.contains(state)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn build_spec() {
    let mut spec = Spec::<u32, char>::new(0);
    spec
        .add_state(1)
        .add_states([1, 2, 3])
        .add_accept_states([3, 4])
        .add_transition(0, '1', 1)
        .add_transition(1, '2', 2)
        .add_transition(2, '2', 2)
        .add_transition(2, 'e', 4);
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
