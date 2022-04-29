use std::ops::{Index, IndexMut};

pub fn distance<'a>(a: impl Into<&'a str>, b: impl Into<&'a str>) -> usize {
  let a = a.into();
  let b = b.into();
  let mut table = Table::<usize>::new(a.len(), b.len());
  for x in 0..a.len() {
    table[(x, 0)] = x;
  }
  for y in 0..b.len() {
    table[(0, y)] = y;
  }
  for y in 1..=table.height() {
    for x in 1..=table.width() {

    }
  }
  0
}

struct Table<T>
  where
    T: Default,
{
  storage: Vec<T>,
  width: usize,
  height: usize,
}

impl <T: Default + Clone> Table<T> {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      storage: vec![T::default(); width * height],
      width,
      height,
    }
  }
  pub fn len(&self) -> usize {
    self.width * self.height
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }
}

impl <T: Default + Clone> Index<(usize,usize)> for Table<T> {
  type Output = T;

  fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
    &self.storage[y * self.width + x]
  }
}

impl <T: Default + Clone> IndexMut<(usize,usize)> for Table<T> {
  fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
    &mut self.storage[y * self.width + x]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  #[should_panic]
  fn table_test() {
    let mut table = Table::<usize>::new(1, 1);
    table[(10, 1)] = 100;
  }

  #[test]
  fn basic() {
    assert_eq!(2, distance("zoi", "koi"))
  }
}
