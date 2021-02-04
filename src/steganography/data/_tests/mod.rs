#[cfg(test)]
mod data_tests {
    use super::super::Data;

    #[test]
    fn allows_for_iterating_over_bits() {
        let data = Data::from_bytes(b"az");
        let result = data.iter_bits().collect::<Vec<_>>();

        let expected: Vec<u8> = vec![
            0, 1, 1, 0, 0, 0, 0, 1, // ascii code for `a` is 97, which is 01100001
            0, 1, 1, 1, 1, 0, 1, 0, // `z` is 122, which is 01111010
        ];

        assert_eq!(result, expected)
    }

    #[test]
    fn allows_for_creating_from_bits() {
        let input = vec![
            0, 1, 1, 0, 0, 0, 0, 1, // ascii code for `a` is 97, which is 01100001
            0, 1, 1, 1, 1, 0, 1, 0, // `z` is 122, which is 01111010
        ];
        let result = Data::from_bits(&input);
        let expected = Data::new(vec![97, 122]);

        assert_eq!(result, expected);
    }
}
