use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum TransitionError {
  NoSuchTransition,
}

impl Display for TransitionError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match *self {
      TransitionError::NoSuchTransition => f.write_str("No such transition in the table."),
    }
  }
}

impl Error for TransitionError {

}