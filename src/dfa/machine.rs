use std::hash::Hash;
use crate::dfa::spec::Spec;
use crate::errors::TransitionError;

pub struct Machine <Stat, Tran>
  where
    Stat: Eq + Hash + Clone,
    Tran: Eq + Hash + Clone,
{
  spec: Spec<Stat, Tran>,
  current: Stat,
}

impl <Stat, Tran> Machine<Stat, Tran>
  where
    Stat: Eq + Hash + Clone,
    Tran: Eq + Hash + Clone,
{
  pub fn from_spec(spec: Spec<Stat, Tran>) -> Self {
    let current = spec.initial.clone();
    Self {
      spec,
      current,
    }
  }
  pub fn step(&mut self, token: Tran) -> Result<(), TransitionError> {
    if let Some(next) = self.spec.transitions.get(&(self.current.clone(), token)) {
      self.current = next.clone();
    } else {
      return Err(TransitionError::NoSuchTransition);
    }
    Ok(())
  }

  pub fn in_accept_states(&self) -> bool {
    self.spec.accept_states.contains(&self.current)
  }

  pub fn has_transition(&self, token: Tran) -> bool {
    self.spec.transitions.contains_key(&(self.current.clone(), token))
  }
}

#[cfg(test)]
mod test {
  use crate::dfa::spec::*;
  use crate::errors::TransitionError::NoSuchTransition;
  use super::*;
  #[test]
  fn basic() {
    let spec =
      SpecBuilder::new(0)
        .add_transition(0, 'a', 0)
        .add_accept_states([0])
        .build();
    let mut machine = Machine::from_spec(spec);
    assert!(machine.has_transition('a'));
    assert_eq!(Ok(()), machine.step('a'));
    assert_eq!(0, machine.current);
    assert!(machine.in_accept_states());
  }
  #[test]
  fn no_transition() {
    let spec =
      SpecBuilder::new(0)
        .add_transition(0, 'a', 0)
        .add_accept_states([0])
        .build();
    let mut machine = Machine::from_spec(spec);
    assert!(!machine.has_transition('0'));
    assert_eq!(Err(NoSuchTransition), machine.step('0'));
  }
}
