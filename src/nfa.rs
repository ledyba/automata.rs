use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Spec<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone,
{
  initial_state: Stat,
  all_states: HashSet<Stat>,
  accept_states: HashSet<Stat>,
  transitions: HashMap<(Stat, Option<Token>), HashSet<Stat>>
}

impl <Stat, Token> Spec <Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone,
{
  pub fn to_dfa_spec(self) -> crate::dfa::Spec<Stat, Token> {
    todo!()
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn basic() {
  }
}
