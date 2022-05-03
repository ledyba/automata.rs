use std::cmp::min;
use std::ops::{Index, IndexMut};

pub fn distance<'a>(a: impl Into<&'a str>, b: impl Into<&'a str>) -> usize {
  let a = a.into().as_bytes();
  let b = b.into().as_bytes();
  let mut table = Table::<usize>::new(a.len() + 1, b.len() + 1);
  for x in 1..=a.len() {
    table[(x, 0)] = x;
  }
  for y in 1..=b.len() {
    table[(0, y)] = y;
  }
  for y in 1..table.height() {
    for x in 1..table.width() {
      let d = if a[x-1] == b[y-1] {
        min(
          min(table[(x-1, y-1)], table[(x-1, y)] + 1),
          table[(x, y-1)] + 1
        )
      } else {
        min(
          min(table[(x-1, y-1)] + 2, table[(x-1, y)] + 1),
          table[(x, y-1)] + 1
        )
      };
      table[(x,y)] = d;
    }
  }
  table[(a.len(), b.len())]
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
  fn identity() {
    assert_eq!(0, distance("koi", "koi"))
  }
  #[test]
  fn replace() {
    assert_eq!(2, distance("koi", "zoi"))
  }
  #[test]
  fn insert() {
    assert_eq!(1, distance("koi", "kooi"))
  }
  #[test]
  fn delete() {
    assert_eq!(1, distance("koi", "ki"))
  }
}
