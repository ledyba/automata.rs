use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use super::spec::{Spec, Input};

pub struct Machine<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone + Debug,
{
  spec: Spec<Stat, Token>,
  current: HashSet<Stat>,
}

impl <Stat, Token> Machine<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone + Debug,
{
  pub fn from_spec(spec: Spec<Stat, Token>) -> Self {
    let mut current = HashSet::from([spec.initial_state().clone()]);
    let mut machine = Self {
      spec,
      current,
    };
    machine.step_by_epsilon();
    machine
  }

  pub fn step(&mut self, by: Token) {
    let mut next = HashSet::<Stat>::new();
    for stat in self.current.iter() {
      next.extend(self.spec.transitions_by_any(stat));
      next.extend(self.spec.transitions_by_token(stat, &by));
    }
    self.step_by_epsilon();
  }

  fn step_by_epsilon(&mut self) {

  }
}
