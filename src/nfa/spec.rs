use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Input<Token>
  where
   Token: Eq + Hash + Clone + Debug,
{
  Epsilon,
  Any,
  Token(Token),
}

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

impl <Stat, Token> Spec <Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone + Debug,
{
  pub fn to_dfa_spec(self) -> crate::dfa::Spec<Stat, Token> {
    todo!()
  }
}

pub struct SpecBuilder<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone + Debug,
{
  initial_state: Stat,
  all_states: HashSet<Stat>,
  accept_states: HashSet<Stat>,
  transitions: HashMap<(Stat, Input<Token>), HashSet<Stat>>,
}

impl <Stat, Token> SpecBuilder<Stat, Token>
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
    self.all_states.extend(states.clone());
    self.accept_states.extend(states);
    self
  }

  pub fn add_token_transition(mut self, from: Stat, by: Token, to: Stat) -> Self {
    self = self.add_state(from.clone());
    self = self.add_state(to.clone());
    let set = self.transitions.entry((from, by)).or_insert_with(|| { HashSet::new() });
    set.insert(Input::Token(to));
    self
  }

  pub fn add_token_transitions<const N: usize>(mut self, from: Stat, by: Token, to_states: [Stat; N]) -> Self {
    self = self.add_state(from.clone());
    self = self.add_states(to_states.clone());
    let set = self.transitions
      .entry((from, Input::Token(by)))
      .or_insert_with(|| { HashSet::new() });
    set.extend(to_states);
    self
  }

  pub fn build(self) -> Spec<Stat, Token> {
    Spec::<Stat, Token> {
      initial_state: self.initial_state,
      all_states: self.all_states,
      accept_states: self.accept_states,
      transitions: self.transitions,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn basic() {
    let spec: Spec<usize, char> = SpecBuilder::new(0 as usize)
      .add_accept_state(1)
      .add_transition(0, Input::Any, 1)
      .add_transition(0, Input::Token('a'), 1)
      .build();
    assert_eq!(0, spec.initial_state);
    assert_eq!(HashSet::from([1]), spec.accept_states);
  }
}
