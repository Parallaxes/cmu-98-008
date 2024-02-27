/// Creates an iterator that interleaves elements from two provided iterators.
///
/// This iterator alternates between the first and second iterators until both are exhausted.
#[derive(Clone)]
pub struct Interleave<I, J> {
    first_iter: I,
    second_iter: J,
    switch: bool,
}

impl<I, J> Interleave<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    /// Creates a new `Interleave` iterator from the given two iterators.
    ///
    /// Both iterators must have the same element type.
    pub fn new(first_iter: I, second_iter: J) -> Self {
        Self {
            first_iter,
            second_iter,
            switch: true,
        }
    }
}

/// Implement the `Iterator` trait for `Interleave`!
impl<I, J> Iterator for Interleave<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    /// What should iterated type be?
    type Item = <I as Iterator>::Item;

    /// Advances the iterator and returns the next interleaved element.
    ///
    /// This alternates between taking elements from the first and second iterators.
    /// If one iterator is exhausted, the remaining elements will be taken from the other iterator.
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        self.switch = !self.switch;

        if !self.switch {
            // Take from first iterator if flag is false
            match self.first_iter.next() {
                None => self.second_iter.next(),
                r => r,
            }
        } else {
            // Take from second iterator if flag is true
            match self.second_iter.next() {
                None => self.first_iter.next(),
                r => r,
            }
        }
    }
}