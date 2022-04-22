use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use super::spec::Spec;

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
    let current = HashSet::from([spec.initial_state().clone()]);
    let mut machine = Self {
      spec,
      current,
    };
    machine.evaluate_epsilons();
    machine
  }

  pub fn step(&mut self, by: Token) {
    let mut next = HashSet::<Stat>::new();
    for stat in self.current.iter() {
      next.extend(self.spec.transitions_by_any(stat));
      next.extend(self.spec.transitions_by_token(stat, &by));
    }
    self.evaluate_epsilons();
  }

  fn evaluate_epsilons(&mut self) {
    loop {
      let mut next = self.current.clone();
      for stat in &self.current {
        next.extend(self.spec.transitions_by_epsilon(stat));
      }
      if &next == &self.current {
        return;
      }
      self.current = next;
    }
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
      .add_epsilon_transition(0, 1);
    let m = Machine::from_spec(spec);
    assert_eq!(HashSet::from([0, 1]), m.current)
  }
}
