use std::hash::Hash;
use crate::dfa::spec::Spec;
use crate::errors::TransitionError;

pub struct Machine <Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone,
{
  spec: Spec<Stat, Token>,
  current: Stat,
}

impl <Stat, Token> Machine<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone,
{
  pub fn from_spec(spec: Spec<Stat, Token>) -> Self {
    let current = spec.initial_state();
    Self {
      spec,
      current,
    }
  }

  pub fn step(&mut self, token: Token) -> Result<(), TransitionError> {
    if let Some(next) = self.spec.transition_of(self.current.clone(), token) {
      self.current = next.clone();
    } else {
      return Err(TransitionError::NoSuchTransition);
    }
    Ok(())
  }

  pub fn in_accept_states(&self) -> bool {
    self.spec.is_acceptable_state(&self.current)
  }

  pub fn has_transition(&self, token: Token) -> bool {
    self.spec.has_transition(self.current.clone(), token)
  }
}

#[cfg(test)]
mod test {
  use crate::dfa::spec::*;
  use crate::errors::TransitionError::NoSuchTransition;
  use super::*;
  #[test]
  fn basic() {
    let mut spec = Spec::new(0);
      spec
        .add_transition(0, 'a', 0)
        .add_accept_states([0]);
    let mut machine = Machine::from_spec(spec);
    assert!(machine.has_transition('a'));
    assert_eq!(Ok(()), machine.step('a'));
    assert_eq!(0, machine.current);
    assert!(machine.in_accept_states());
  }
  #[test]
  fn no_transition() {
    let mut spec = Spec::new(0);
    spec
      .add_transition(0, 'a', 0)
      .add_accept_states([0]);
    let mut machine = Machine::from_spec(spec);
    assert!(!machine.has_transition('0'));
    assert_eq!(Err(NoSuchTransition), machine.step('0'));
  }
}
