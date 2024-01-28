use std::fmt;

use snafu::Backtrace;

use crate::error::InvalidIndex;

pub(super) struct MapTryIntoUsize<I, T>
where
    I: Iterator<Item = T>,
{
    inner: I,
}

impl<I, T> MapTryIntoUsize<I, T>
where
    I: Iterator<Item = T>,
{
    pub(super) fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I, T> ExactSizeIterator for MapTryIntoUsize<I, T>
where
    I: ExactSizeIterator<Item = T>,
    T: TryInto<usize> + fmt::Debug + fmt::Display,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<I, T> Iterator for MapTryIntoUsize<I, T>
where
    I: Iterator<Item = T>,
    T: TryInto<usize> + fmt::Debug + fmt::Display,
{
    type Item = Result<usize, InvalidIndex<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.inner.next()?;
        Some(index.try_into().map_err(|_| InvalidIndex {
            index,
            backtrace: Backtrace::capture(),
        }))
    }
}
