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

  let mut prev_len = 0;
  while prev_len != r.len() {
    prev_len = r.len();
    for token in &tokens {
      for state_set in r.all_states().iter() {
        let mut set = HashSet::<Stat>::new();
        for state in state_set {
          set.extend(spec.transitions_by_token(state, token));
          set.extend(spec.transitions_by_any(state));
        }
        if !set.is_empty() {
          let is_acceptable = state_set.iter().any(|it| spec.is_acceptable_state(it));
          let new_state = set_to_vec(set);
          if is_acceptable {
            r.add_accept_state(new_state.clone());
          }
          r.add_transition(state_set.clone(), token.clone(), new_state);
        }
      }
    }
  }
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
    let spec = {
      let mut spec: nfa::Spec<usize, char> = nfa::Spec::new(0);
      spec
        .add_epsilon_transition(0, 1)
        .add_token_transition(0, 'a', 2);
      translate_to_dfa(&spec)
    };
    assert_set_eq([0, 1], spec.initial_state());
    assert_sets_eq([vec![0], vec![1]], spec.all_states());
  }

  fn assert_set_eq<T: Eq + Hash + Debug, V: IntoIterator<Item=T>, const N: usize>(left: [T; N], right: V) {
    assert_eq!(HashSet::<T>::from(left), HashSet::from_iter(right.into_iter()));
  }

  fn assert_sets_eq<T: Eq + Hash + Ord + PartialOrd + Clone + Debug, V: IntoIterator<Item=Vec<T>>, const N: usize>(left: [Vec<T>; N], right: V) {
    assert_eq!(
      HashSet::<Vec<T>>::from_iter(left.iter().map(|it| {
        let mut v = it.clone();
        v.sort();
        v
      })),
      HashSet::<Vec<T>>::from_iter(right.into_iter().map(|mut it: Vec<T>| {
        it.sort();
        it
      })),
    );
  }
}
