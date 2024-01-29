

use snafu::Backtrace;

use crate::error::InvalidIndex;

pub(super) struct MapTryIntoUsize<I>
where
    I: Iterator<Item = i32>,
{
    inner: I,
}

impl<I> MapTryIntoUsize<I>
where
    I: Iterator<Item = i32>,
{
    pub(super) fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I> ExactSizeIterator for MapTryIntoUsize<I>
where
    I: ExactSizeIterator<Item = i32>,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<I> Iterator for MapTryIntoUsize<I>
where
    I: Iterator<Item = i32>,
{
    type Item = Result<usize, InvalidIndex>;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.inner.next()?;
        Some(index.try_into().map_err(|_| InvalidIndex {
            index,
            backtrace: Backtrace::capture(),
        }))
    }
}
