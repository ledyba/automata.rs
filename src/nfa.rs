use std::hash::Hash;

pub struct Spec<Stat, Token>
  where
    Stat: Eq + Hash + Clone,
    Token: Eq + Hash + Clone,
{

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
