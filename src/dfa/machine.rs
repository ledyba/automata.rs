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
    if let Some(next) = self.spec.transition.get(&(self.current.clone(), token)) {
      self.current = next.clone();
    } else {
      return Err(TransitionError::NoSuchTransition);
    }
    Ok(())
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
    assert_eq!(Ok(()), machine.step('a'));
    assert_eq!(0, machine.current);
  }
  #[test]
  fn no_transition() {
    let spec =
      SpecBuilder::new(0)
        .add_transition(0, 'a', 0)
        .add_accept_states([0])
        .build();
    let mut machine = Machine::from_spec(spec);
    assert_eq!(Err(NoSuchTransition), machine.step('0'));
  }
}