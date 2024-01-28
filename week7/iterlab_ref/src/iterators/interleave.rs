
pub struct Interleave<I, J> {
    first_iter: I,
    second_iter: J,
}

impl<I, J> Interleave<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    pub fn new(first_iter: I, second_iter: J) -> Self {
        Self { first_iter, second_iter }
    }
}


impl<I, J> Iterator for Interleave<I, J> where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        todo!()
    }
}
