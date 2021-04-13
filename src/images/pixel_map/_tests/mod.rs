#[cfg(test)]
mod images_pixel_map_tests {
    use super::super::Pixel;
    use super::super::PixelMap;
    use super::super::{PixelMapWindows, WindowOffsets};

    fn mock_image() -> PixelMap {
        PixelMap::new(
            4,
            4,
            vec![
                // using blue channel as primitive form of id
                Pixel::new(0, 0, 0, 0, 10),
                Pixel::new(1, 0, 0, 0, 20),
                Pixel::new(2, 0, 0, 0, 30),
                Pixel::new(3, 0, 0, 0, 40),
                Pixel::new(0, 1, 0, 0, 50),
                Pixel::new(1, 1, 0, 0, 60),
                Pixel::new(2, 1, 0, 0, 70),
                Pixel::new(3, 1, 0, 0, 80),
                Pixel::new(0, 2, 0, 0, 90),
                Pixel::new(1, 2, 0, 0, 100),
                Pixel::new(2, 2, 0, 0, 110),
                Pixel::new(3, 2, 0, 0, 120),
                Pixel::new(0, 3, 0, 0, 130),
                Pixel::new(1, 3, 0, 0, 140),
                Pixel::new(2, 3, 0, 0, 150),
                Pixel::new(3, 3, 0, 0, 150),
            ],
        )
    }

    #[test]
    fn it_returns_4_neightbours_from_center() {
        let map = mock_image();
        let neighbours = map
            .get_neighbours_4(1, 1)
            .map(|px| px.b)
            .collect::<Vec<_>>();

        let expected = [20, 50, 70, 100];

        assert_eq!(neighbours.len(), 4);
        assert!(expected.iter().all(|id| neighbours.contains(id)))
    }

    #[test]
    fn it_returns_8_neightbours_from_center() {
        let map = mock_image();
        let neighbours = map
            .get_neighbours_8(1, 1)
            .map(|px| px.b)
            .collect::<Vec<_>>();

        let expected = [10, 20, 30, 50, 70, 90, 100, 110];

        assert_eq!(neighbours.len(), 8);
        assert!(expected.iter().all(|id| neighbours.contains(id)))
    }

    #[test]
    fn it_returns_4_neightbours_from_edge() {
        let map = mock_image();
        let neighbours = map
            .get_neighbours_4(3, 1)
            .map(|px| px.b)
            .collect::<Vec<_>>();

        let expected = [40, 70, 120];

        assert_eq!(neighbours.len(), 3);
        assert!(expected.iter().all(|id| neighbours.contains(id)))
    }

    #[test]
    fn it_returns_8_neightbours_from_edge() {
        let map = mock_image();
        let neighbours = map
            .get_neighbours_8(3, 1)
            .map(|px| px.b)
            .collect::<Vec<_>>();

        let expected = [30, 40, 70, 110, 120];

        assert_eq!(neighbours.len(), 5);
        assert!(expected.iter().all(|id| neighbours.contains(id)))
    }

    #[test]
    fn it_allows_for_mapping() {
        let map = mock_image();
        let result = map.map(|pixel| Pixel::new(pixel.x, pixel.y, pixel.r, pixel.g, pixel.b + 1));

        let expected = PixelMap::new(
            4,
            4,
            vec![
                // using blue channel as primitive form of id
                Pixel::new(0, 0, 0, 0, 11),
                Pixel::new(1, 0, 0, 0, 21),
                Pixel::new(2, 0, 0, 0, 31),
                Pixel::new(3, 0, 0, 0, 41),
                Pixel::new(0, 1, 0, 0, 51),
                Pixel::new(1, 1, 0, 0, 61),
                Pixel::new(2, 1, 0, 0, 71),
                Pixel::new(3, 1, 0, 0, 81),
                Pixel::new(0, 2, 0, 0, 91),
                Pixel::new(1, 2, 0, 0, 101),
                Pixel::new(2, 2, 0, 0, 111),
                Pixel::new(3, 2, 0, 0, 121),
                Pixel::new(0, 3, 0, 0, 131),
                Pixel::new(1, 3, 0, 0, 141),
                Pixel::new(2, 3, 0, 0, 151),
                Pixel::new(3, 3, 0, 0, 151),
            ],
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn it_allows_for_iterating_windows_of_eql_size() {
        let map = mock_image();
        let map_windows = map.windows(2, 2);
        let mut windows = map_windows.iter();

        assert_eq!(
            windows.next().unwrap().1,
            PixelMap::new(
                2,
                2,
                vec![
                    Pixel::new(0, 0, 0, 0, 10),
                    Pixel::new(1, 0, 0, 0, 20),
                    Pixel::new(0, 1, 0, 0, 50),
                    Pixel::new(1, 1, 0, 0, 60),
                ]
            )
        );
        assert_eq!(
            windows.next().unwrap().1,
            PixelMap::new(
                2,
                2,
                vec![
                    Pixel::new(0, 0, 0, 0, 30),
                    Pixel::new(1, 0, 0, 0, 40),
                    Pixel::new(0, 1, 0, 0, 70),
                    Pixel::new(1, 1, 0, 0, 80),
                ]
            )
        );
        assert_eq!(
            windows.next().unwrap().1,
            PixelMap::new(
                2,
                2,
                vec![
                    Pixel::new(0, 0, 0, 0, 90),
                    Pixel::new(1, 0, 0, 0, 100),
                    Pixel::new(0, 1, 0, 0, 130),
                    Pixel::new(1, 1, 0, 0, 140),
                ]
            )
        );
        assert_eq!(
            windows.next().unwrap().1,
            PixelMap::new(
                2,
                2,
                vec![
                    Pixel::new(0, 0, 0, 0, 110),
                    Pixel::new(1, 0, 0, 0, 120),
                    Pixel::new(0, 1, 0, 0, 150),
                    Pixel::new(1, 1, 0, 0, 150),
                ]
            )
        );
        assert_eq!(windows.next(), None);
    }

    #[test]
    fn it_should_generate_even_offsets_if_possible() {
        let offsets = PixelMapWindows::generate_offsets(10, 1, 5, 1).collect::<Vec<_>>();
        let expected: Vec<WindowOffsets> = vec![
            (0, (0, 1, 0), (0, 2, 1)),
            (1, (0, 1, 0), (2, 2, 3)),
            (2, (0, 1, 0), (4, 2, 5)),
            (3, (0, 1, 0), (6, 2, 7)),
            (4, (0, 1, 0), (8, 2, 9)),
        ];

        assert_eq!(offsets, expected);
    }

    #[test]
    fn it_should_handle_uneven_chunks() {
        let offsets = PixelMapWindows::generate_offsets(10, 1, 3, 1).collect::<Vec<_>>();
        let expected: Vec<WindowOffsets> = vec![
            (0, (0, 1, 0), (0, 3, 2)),
            (1, (0, 1, 0), (3, 3, 5)),
            (2, (0, 1, 0), (6, 4, 9)),
        ];

        assert_eq!(offsets, expected);
    }

    #[test]
    fn it_should_handle_uneven_chunks_by_returning_correct_len() {
        let offsets = PixelMapWindows::generate_offsets(200, 1, 30, 1).collect::<Vec<_>>();
        let expected = 30;

        assert_eq!(offsets.len(), expected);
    }
}
