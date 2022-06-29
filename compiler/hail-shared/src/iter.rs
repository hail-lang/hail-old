/// A better `.enumerate()` implementation for Rust.
#[derive(Clone, Debug, PartialEq)]
pub struct BetterIter<T: Iterator> {
    /// The iterator that this struct wraps.
    iter: T,

    /// The current index of the enumerate iterator.
    idx: usize,
}

impl<T: Iterator> BetterIter<T> {
    /// Returns the current index of the enumerate iterator.
    pub fn idx(&self) -> usize {
        self.idx
    }
}

impl<T: Iterator + Clone> BetterIter<T> {
    /// Peeks the next item in the iterator, without iterating it.
    pub fn peek(&self) -> Option<<BetterIter<T> as Iterator>::Item> {
        self.iter.clone().next()
    }

    /// Peeks `n` amount of items ahead in the iterator.
    pub fn peeki(&self, n: usize) -> Option<<BetterIter<T> as Iterator>::Item> {
        let mut iter = self.iter.clone();

        let mut i = 0;
        while i < n {
            if let Some(item) = iter.next() {
                if i + 1 == n {
                    return Some(item)
                }

                i += 1;
                continue;
            }

            return None;
        }

        None
    }
}

impl<T: Iterator> Iterator for BetterIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.iter.next()
    }
}

pub trait IntoBetterIterator {
    fn better_iter(self) -> BetterIter<Self> 
    where Self: std::iter::Iterator + Sized
    {
        BetterIter {
            iter: self,
            idx: 0,
        }
    }
}

impl<T: Iterator> IntoBetterIterator for T {}