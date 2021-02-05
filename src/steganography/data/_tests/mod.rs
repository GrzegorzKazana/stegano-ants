#[cfg(test)]
mod data_tests {
    use super::super::{Bit, Data};

    fn bits(bits_vec: Vec<u8>) -> Vec<Bit> {
        bits_vec.into_iter().map(Bit).collect()
    }

    #[test]
    fn allows_for_iterating_over_bits() {
        let data = Data::from_bytes(b"az");
        let result = data.iter_bits().collect::<Vec<_>>();

        let expected = bits(vec![
            0, 1, 1, 0, 0, 0, 0, 1, // ascii code for `a` is 97, which is 01100001
            0, 1, 1, 1, 1, 0, 1, 0, // `z` is 122, which is 01111010
        ]);

        assert_eq!(result, expected)
    }

    #[test]
    fn allows_for_creating_from_bits() {
        let input = bits(vec![
            0, 1, 1, 0, 0, 0, 0, 1, // ascii code for `a` is 97, which is 01100001
            0, 1, 1, 1, 1, 0, 1, 0, // `z` is 122, which is 01111010
        ]);
        let result = Data::from_bits(&input);
        let expected = Data::new(vec![97, 122]);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_converts_bytes_to_bits() {
        assert_eq!(
            Data::byte_to_bits_iter(97).collect::<Vec<_>>(),
            bits(vec![0, 1, 1, 0, 0, 0, 0, 1])
        );
    }

    #[test]
    fn it_correctly_yield_bits() {
        let data = Data::from_bytes(b"az");
        let mut bits_iter = data.iter_bits();

        let bits_a = bits_iter.by_ref().take(3).collect::<Vec<_>>();
        let bits_b = bits_iter.by_ref().take(5).collect::<Vec<_>>();

        assert_eq!(
            bits_a,
            vec![0u8, 1, 1].into_iter().map(Bit).collect::<Vec<_>>()
        );
        assert_eq!(bits_b, bits(vec![0u8, 0, 0, 0, 1]));
    }
}
