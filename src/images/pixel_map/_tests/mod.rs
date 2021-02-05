#[cfg(test)]
mod images_pixel_map_tests {
    use super::super::Pixel;
    use super::super::PixelMap;

    fn mock_image() -> PixelMap {
        PixelMap::new(
            3,
            3,
            vec![
                // using blue channel as primitive form of id
                Pixel::new(0, 0, 0, 0, 10),
                Pixel::new(1, 0, 0, 0, 20),
                Pixel::new(2, 0, 0, 0, 30),
                Pixel::new(0, 1, 0, 0, 40),
                Pixel::new(1, 1, 0, 0, 50),
                Pixel::new(2, 1, 0, 0, 60),
                Pixel::new(0, 2, 0, 0, 70),
                Pixel::new(1, 2, 0, 0, 80),
                Pixel::new(2, 2, 0, 0, 90),
            ],
        )
    }

    #[test]
    fn it_returns_4_neightbours_from_center() {
        let map = mock_image();
        let neighbours = map
            .get_neighbours_4(1, 1)
            .iter()
            .map(|px| px.b)
            .collect::<Vec<_>>();

        let expected = [20, 40, 60, 80];

        assert_eq!(neighbours.len(), 4);
        assert!(expected.iter().all(|id| neighbours.contains(id)))
    }

    #[test]
    fn it_returns_8_neightbours_from_center() {
        let map = mock_image();
        let neighbours = map
            .get_neighbours_8(1, 1)
            .iter()
            .map(|px| px.b)
            .collect::<Vec<_>>();

        let expected = [10, 20, 30, 40, 60, 70, 80, 90];

        assert_eq!(neighbours.len(), 8);
        assert!(expected.iter().all(|id| neighbours.contains(id)))
    }

    #[test]
    fn it_returns_4_neightbours_from_edge() {
        let map = mock_image();
        let neighbours = map
            .get_neighbours_4(2, 1)
            .iter()
            .map(|px| px.b)
            .collect::<Vec<_>>();

        let expected = [30, 50, 90];

        assert_eq!(neighbours.len(), 3);
        assert!(expected.iter().all(|id| neighbours.contains(id)))
    }

    #[test]
    fn it_returns_8_neightbours_from_edge() {
        let map = mock_image();
        let neighbours = map
            .get_neighbours_8(2, 1)
            .iter()
            .map(|px| px.b)
            .collect::<Vec<_>>();

        let expected = [20, 30, 50, 80, 90];

        assert_eq!(neighbours.len(), 5);
        assert!(expected.iter().all(|id| neighbours.contains(id)))
    }

    #[test]
    fn it_allows_for_mapping() {
        let map = mock_image();
        let result = map.map(|pixel| Pixel::new(pixel.x, pixel.y, pixel.r, pixel.g, pixel.b * 2));

        let expected = PixelMap::new(
            3,
            3,
            vec![
                // using blue channel as primitive form of id
                Pixel::new(0, 0, 0, 0, 20),
                Pixel::new(1, 0, 0, 0, 40),
                Pixel::new(2, 0, 0, 0, 60),
                Pixel::new(0, 1, 0, 0, 80),
                Pixel::new(1, 1, 0, 0, 100),
                Pixel::new(2, 1, 0, 0, 120),
                Pixel::new(0, 2, 0, 0, 140),
                Pixel::new(1, 2, 0, 0, 160),
                Pixel::new(2, 2, 0, 0, 180),
            ],
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn it_allows_for_scan_mapping() {
        let map = mock_image();
        let result = map.scan_map(0, |pixel, acc| {
            (
                Pixel::new(pixel.x, pixel.y, pixel.r, pixel.g, pixel.b * 2),
                acc + 1,
            )
        });

        let expected = (
            PixelMap::new(
                3,
                3,
                vec![
                    // using blue channel as primitive form of id
                    Pixel::new(0, 0, 0, 0, 20),
                    Pixel::new(1, 0, 0, 0, 40),
                    Pixel::new(2, 0, 0, 0, 60),
                    Pixel::new(0, 1, 0, 0, 80),
                    Pixel::new(1, 1, 0, 0, 100),
                    Pixel::new(2, 1, 0, 0, 120),
                    Pixel::new(0, 2, 0, 0, 140),
                    Pixel::new(1, 2, 0, 0, 160),
                    Pixel::new(2, 2, 0, 0, 180),
                ],
            ),
            9,
        );

        assert_eq!(result, expected);
    }
}
