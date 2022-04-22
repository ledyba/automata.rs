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
    let current = HashSet::from([spec.initial_state().clone()]);
    Self {
      spec,
      current,
    }
  }

}
