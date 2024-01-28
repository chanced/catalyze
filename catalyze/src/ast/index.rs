use snafu::Backtrace;

use crate::error::InvalidIndex;

pub(super) struct Iter<I: Iterator> {
    inner: I,
}

impl<I> Iter<I>
where
    I: Iterator,
{
    pub(super) fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I> ExactSizeIterator for Iter<I>
where
    I: ExactSizeIterator<Item = i32>,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<I> Iterator for Iter<I>
where
    I: Iterator<Item = i32>,
{
    type Item = Result<usize, InvalidIndex>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        let next = next.try_into().map_err(|_| InvalidIndex {
            backtrace: Backtrace::capture(),
            index: next,
        });
        Some(next)
    }
}
