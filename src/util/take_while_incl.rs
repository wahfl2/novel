pub trait MyIterator : Iterator {
    fn take_while_inclusive<F>(self, accept: F) -> TakeWhileInclusive<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        TakeWhileInclusive::new(self, accept)
    }
}

impl<T: ?Sized> MyIterator for T where T: Iterator { }

// The majority of this code was copied from Itertools::take_while_inclusive
// However, the lifetime is unnecessary, at least for my usecase
pub struct TakeWhileInclusive<I, F> {
    iter: I,
    predicate: F,
    done: bool,
}

impl<I, F> TakeWhileInclusive<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    pub fn new(iter: I, predicate: F) -> Self {
        Self { iter, predicate, done: false}
    }
}

impl<I, F> Iterator for TakeWhileInclusive<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.iter.next().map(|item| {
                if !(self.predicate)(&item) {
                    self.done = true;
                }
                item
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            (0, Some(0))
        } else {
            (0, self.iter.size_hint().1)
        }
    }
}