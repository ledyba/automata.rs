use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Input<Token>
  where
   Token: Eq + Hash + Clone + Debug,
{
  Epsilon,
  Any,
  Token(Token),
}

#[derive(Clone)]
pub struct Spec<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone + Debug,
{
  initial_state: Stat,
  all_states: HashSet<Stat>,
  accept_states: HashSet<Stat>,
  transitions: HashMap<(Stat, Input<Token>), HashSet<Stat>>,
}

impl <Stat, Token> Spec<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone + Debug,
{
  pub fn new(initial_state: Stat) -> Self {
    Self {
      initial_state,
      all_states: HashSet::new(),
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
    self.all_states.extend(states.clone());
    self.accept_states.extend(states);
    self
  }

  fn add_transition(&mut self, from: Stat, by: Input<Token>, to: Stat) {
    self.add_state(from.clone());
    self.add_state(to.clone());
    let set = self.transitions
      .entry((from, by))
      .or_insert_with(|| { HashSet::new() });
    set.insert(to);
  }

  pub fn add_transitions<const N: usize>(&mut self, from: Stat, by: Input<Token>, to_states: [Stat; N]) {
    self.add_state(from.clone());
    self.add_states(to_states.clone());
    let set = self.transitions
      .entry((from, by))
      .or_insert_with(|| { HashSet::new() });
    set.extend(to_states);
  }

  pub fn add_any_transition(&mut self, from: Stat, to: Stat) -> &mut Self {
    self.add_transition(from, Input::Any, to);
    self
  }

  pub fn add_any_transitions<const N: usize>(&mut self, from: Stat, to_states: [Stat; N]) -> &mut Self {
    self.add_transitions(from, Input::Any, to_states);
    self
  }

  pub fn add_epsilon_transition(&mut self, from: Stat, to: Stat) -> &mut Self {
    self.add_transition(from, Input::Epsilon, to);
    self
  }

  pub fn add_epsilon_transitions<const N: usize>(&mut self, from: Stat, to_states: [Stat; N]) -> &mut Self {
    self.add_transitions(from, Input::Epsilon, to_states);
    self
  }

  pub fn add_token_transition(&mut self, from: Stat, by: Token, to: Stat) -> &mut Self {
    self.add_transition(from, Input::Token(by), to);
    self
  }

  pub fn add_token_transitions<const N: usize>(&mut self, from: Stat, by: Token, to_states: [Stat; N]) -> &mut Self {
    self.add_transitions(from, Input::Token(by), to_states);
    self
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn basic() {
    let mut spec: Spec<usize, char> = Spec::new(0);
    spec
      .add_accept_state(1)
      .add_any_transition(0, 1)
      .add_token_transition(0, 'a', 1);
    assert_eq!(0, spec.initial_state);
    assert_eq!(HashSet::from([1]), spec.accept_states);
  }
  fn empty() {
    let spec: Spec<usize, usize> = Spec::new(0);
    assert_eq!(HashSet::from([0]), spec.all_states);
  }
}
