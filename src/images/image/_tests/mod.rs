#[cfg(test)]
mod image_tests {
    use std::cmp::Ordering;

    use super::super::Pixel;

    #[test]
    fn it_is_ordered_by_coords_correctly() {
        assert_eq!(
            Pixel::cmp_by_coords(&Pixel::black(1, 1), &Pixel::black(1, 1)),
            Ordering::Equal
        );
        assert_eq!(
            Pixel::cmp_by_coords(&Pixel::black(1, 1), &Pixel::black(1, 0)),
            Ordering::Greater
        );
        assert_eq!(
            Pixel::cmp_by_coords(&Pixel::black(1, 1), &Pixel::black(1, 2)),
            Ordering::Less
        );
        assert_eq!(
            Pixel::cmp_by_coords(&Pixel::black(1, 1), &Pixel::black(0, 1)),
            Ordering::Greater
        );
        assert_eq!(
            Pixel::cmp_by_coords(&Pixel::black(1, 1), &Pixel::black(2, 1)),
            Ordering::Less
        );
    }
}
