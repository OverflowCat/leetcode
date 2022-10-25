use std::default::Default;
use std::iter::Fuse;

pub trait MyIterTools: Iterator {
    fn zip_default<J>(self, other: J) -> ZipDefault<Self, J::IntoIter>
    where
        J: IntoIterator,
        Self: Sized,
    {
        ZipDefault::new(self, other.into_iter())
    }
}

#[derive(Clone, Debug)]
pub struct ZipDefault<I, J> {
    i: Fuse<I>,
    j: Fuse<J>,
}

impl<I, J> ZipDefault<I, J>
where
    I: Iterator,
    J: Iterator,
{
    fn new(i: I, j: J) -> Self {
        Self {
            i: i.fuse(),
            j: j.fuse(),
        }
    }
}

impl<T, U, A, B> Iterator for ZipDefault<T, U>
where
    T: Iterator<Item = A>,
    U: Iterator<Item = B>,
    A: Default,
    B: Default,
{
    type Item = (A, B);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.i.next(), self.j.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            (Some(a), None) => Some((a, B::default())),
            (None, Some(b)) => Some((A::default(), b)),
            (None, None) => None,
        }
    }
}

impl<T: ?Sized> MyIterTools for T where T: Iterator {}

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let x = word1.iter().map(|s| s.bytes()).flatten();
        let y = word2.iter().map(|s| s.bytes()).flatten();
        x.zip_default(y).all(|(a, b)| a == b && a != 0 && b != 0)
    }
}
