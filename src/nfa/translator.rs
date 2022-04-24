use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use crate::dfa;
use crate::nfa;

pub fn translate_to_dfa<Stat, Token>(spec: &nfa::Spec<Stat, Token>) -> dfa::Spec<Vec<Stat>, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone + Debug,
{
  let mut initial = HashSet::from([spec.initial_state()]);
  initial = eval_epsilon(spec, initial);
  let mut r: dfa::Spec<Vec<Stat>, Token> = dfa::Spec::new(set_to_vec(initial));
  let tokens = spec.tokens();

  r
}

fn set_to_vec<Stat>(set: HashSet<Stat>) -> Vec<Stat>
  where
    Stat: Eq + PartialEq + Hash
{
  let mut v = Vec::from_iter(set);
  v.sort_by_key(|it| {
    let mut hasher = DefaultHasher::default();
    it.hash(&mut hasher);
    hasher.finish()
  });
  v
}

fn eval_epsilon<Stat, Token>(spec: &nfa::Spec<Stat, Token>, mut seed: HashSet<Stat>) -> HashSet<Stat>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone + Debug,
{
  loop {
    let mut next = seed.clone();
    for stat in &seed {
      next.extend(spec.transitions_by_epsilon(stat));
    }
    if &next == &seed {
      return next;
    }
    seed = next;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn basic() {
    let mut spec: nfa::Spec<usize, char> = nfa::Spec::new(0);
    spec.add_epsilon_transition(0, 1);
    assert_eq!(0, spec.tokens().len());
    spec.add_token_transition(0, 'a', 1);
    assert_eq!(1, spec.tokens().len());
  }
}
