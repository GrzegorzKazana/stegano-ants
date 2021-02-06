/// Iterator extension allowing for creating an iterator implementing
/// ExactSizeIterator from chaining two ExactSizeIterator iterators.
///
/// It also required and implements DoubleEndedIterator to support `.rev` operations
/// but is not required for implementing ExactSizeIterator trait.
///
/// Reason std::iter::Chain does not support it by default is that
/// adding lengths of two iterators may end up overflowing
///
/// ExactSizedChain will panic in that case, so make sure you are using it safely.
///
/// usage example:
/// ```
/// use somewhere::ExactChainExt;
///
/// let chained = (0..5).chain_exact(0..5);
///
/// assert_eq!(chained.len(), 10)
/// ```

pub struct ExactSizedChain<A, B>
where
    A: ExactSizeIterator + DoubleEndedIterator,
    B: ExactSizeIterator<Item = A::Item> + DoubleEndedIterator,
{
    underlying: std::iter::Chain<A, B>,
    len_a: usize,
    len_b: usize,
}

impl<A, B> ExactSizedChain<A, B>
where
    A: ExactSizeIterator + DoubleEndedIterator,
    B: ExactSizeIterator<Item = A::Item> + DoubleEndedIterator,
{
    pub fn new(a: A, b: B) -> Self {
        let len_a = a.len();
        let len_b = b.len();

        Self {
            underlying: a.chain(b),
            len_a,
            len_b,
        }
    }
}

impl<A, B> Iterator for ExactSizedChain<A, B>
where
    A: ExactSizeIterator + DoubleEndedIterator,
    B: ExactSizeIterator<Item = A::Item> + DoubleEndedIterator,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.underlying.size_hint()
    }
}

impl<A, B> ExactSizeIterator for ExactSizedChain<A, B>
where
    A: ExactSizeIterator + DoubleEndedIterator,
    B: ExactSizeIterator<Item = A::Item> + DoubleEndedIterator,
{
    fn len(&self) -> usize {
        self.len_a
            .checked_add(self.len_b)
            .expect("ExactSizedChained panicked by total length exceeding `usize::MAX`")
    }
}

impl<A, B> DoubleEndedIterator for ExactSizedChain<A, B>
where
    A: ExactSizeIterator + DoubleEndedIterator,
    B: ExactSizeIterator<Item = A::Item> + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.underlying.next_back()
    }
}

pub trait ExactChainExt: ExactSizeIterator + DoubleEndedIterator {
    fn chain_exact<I: ExactSizeIterator<Item = Self::Item> + DoubleEndedIterator>(
        self,
        other: I,
    ) -> ExactSizedChain<Self, I>
    where
        Self: Sized,
    {
        ExactSizedChain::new(self, other)
    }
}

impl<I: ExactSizeIterator + DoubleEndedIterator> ExactChainExt for I {}
