/// This a iterator extension that allows for mapping with accumulator
/// using pure function.
/// This differs from `.scan`, as it relies on mutable reference. MapAccum relies on
/// the actual value.
///
/// Function passed as a mapper is expected to return tuple of (new_accumulator, mapped_item).
///
/// usage example:
/// ```
/// use somewhere::MapAccumExt;
///
/// let mapped = (1..5).map_accum(0, |acc, curr| (acc + curr, acc + curr));
///
/// assert_eq!(mapped.next(), Option::Some(1))
/// assert_eq!(mapped.next(), Option::Some(3))
/// assert_eq!(mapped.next(), Option::Some(6))
/// assert_eq!(mapped.next(), Option::Some(10))
/// assert_eq!(mapped.next(), Option::None)
/// ```

pub struct MapAccum<A, B, C, F>
where
    A: Iterator,
    F: Fn(B, A::Item) -> (B, C),
{
    underlying: A,
    accumulator: Option<B>,
    mapper: F,
}

impl<A, B, C, F> MapAccum<A, B, C, F>
where
    A: Iterator,
    F: Fn(B, A::Item) -> (B, C),
{
    pub fn new(underlying: A, accumulator: B, mapper: F) -> Self {
        MapAccum {
            underlying,
            // we use Option as an reference container, so that we can
            // move out of it and replace its value
            accumulator: Option::Some(accumulator),
            mapper,
        }
    }

    fn calculate_value_and_update_accumulator(&mut self, item: A::Item) -> C {
        // this unwrap is safe, since `self.accumulator` is always some
        // we just use it as a container we can move out of
        let accumulator_value = self.accumulator.take().unwrap();

        let (new_accumulator, transformed_item) = (self.mapper)(accumulator_value, item);
        self.accumulator.replace(new_accumulator);

        transformed_item
    }
}

impl<A, B, C, F> Iterator for MapAccum<A, B, C, F>
where
    A: Iterator,
    F: Fn(B, A::Item) -> (B, C),
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying
            .next()
            .map(|item| self.calculate_value_and_update_accumulator(item))
    }
}

impl<A, B, C, F> ExactSizeIterator for MapAccum<A, B, C, F>
where
    A: ExactSizeIterator,
    F: Fn(B, A::Item) -> (B, C),
{
    fn len(&self) -> usize {
        self.underlying.len()
    }
}

impl<A, B, C, F> DoubleEndedIterator for MapAccum<A, B, C, F>
where
    A: DoubleEndedIterator,
    F: Fn(B, A::Item) -> (B, C),
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.underlying
            .next_back()
            .map(|item| self.calculate_value_and_update_accumulator(item))
    }
}

pub trait MapAccumExt: Iterator {
    fn map_accum<B, C, F: Fn(B, Self::Item) -> (B, C)>(
        self,
        init: B,
        mapper: F,
    ) -> MapAccum<Self, B, C, F>
    where
        Self: Sized,
    {
        MapAccum::new(self, init, mapper)
    }
}

impl<A: Iterator> MapAccumExt for A {}
