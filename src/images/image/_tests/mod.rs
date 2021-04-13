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

    #[test]
    fn it_converts_rgb_to_xyz() {
        let rgb = Pixel::new(0, 0, 255, 255, 22);
        let xyz = rgb.to_xyz();

        assert_delta!(xyz.x, 0.7714801);
        assert_delta!(xyz.y, 0.9284041);
        assert_delta!(xyz.z, 0.1461503);
    }

    #[test]
    fn it_converts_rgb_to_lab() {
        let rgb = Pixel::new(0, 0, 255, 255, 22);
        let lab = rgb.to_lab();

        assert_delta!(lab.l, 97.1627998);
        assert_delta!(lab.a, -21.3609707);
        assert_delta!(lab.b, 92.7035375);
    }
}
