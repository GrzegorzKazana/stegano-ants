mod _mocks;

#[cfg(test)]
mod image_embedder_tests {
    use proptest::prelude::*;

    use super::super::{EmbedInImage, MaskImageEmbedder};
    use super::_mocks as mocks;

    use crate::images::image::Pixel;
    use crate::images::pixel_map::PixelMap;
    use crate::steganography::data::Data;

    #[test]
    fn it_should_correctly_embed_the_data() {
        let transport = mocks::mock_transport_image();
        let mask = mocks::mock_mask_image();
        let data = mocks::mock_data();
        let expected = mocks::expected_steganogram();

        let result = MaskImageEmbedder::new(&mask).embed(&mut data.iter_bits(), &transport);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_extract_embedded_data() {
        let steganogram = mocks::expected_steganogram();
        let mask = mocks::mock_mask_image();
        let data = mocks::mock_data();

        let result = MaskImageEmbedder::new(&mask).extract(&steganogram);
        // only first 9 bytes fit into the image
        let expected = Data::from_bytes(&data.bytes()[..9]);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_correctly_embed_the_data_if_image_has_capacity_large_enough_to_fit_the_end_token()
    {
        let transport = mocks::mock_transport_image();
        let mask = mocks::mock_mask_image();
        let data = mocks::mock_data_very_short();
        let expected = mocks::expected_steganogram_very_short();

        let result = MaskImageEmbedder::new(&mask).embed(&mut data.iter_bits(), &transport);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_correctly_extract_the_data_if_image_has_capacity_large_enough_to_fit_the_end_token(
    ) {
        let steganogram = mocks::expected_steganogram_very_short();
        let mask = mocks::mock_mask_image();
        let expected = mocks::mock_data_very_short();

        let result = MaskImageEmbedder::new(&mask).extract(&steganogram);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_correctly_embed_the_data_if_image_has_capacity_that_fits_data_but_part_of_end_token(
    ) {
        let transport = mocks::mock_transport_image();
        let mask = mocks::mock_mask_image();
        let data = mocks::mock_data_short();
        let expected = mocks::expected_steganogram_short();

        let result = MaskImageEmbedder::new(&mask).embed(&mut data.iter_bits(), &transport);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_correctly_extract_the_data_if_image_has_capacity_that_fits_data_but_part_of_end_token(
    ) {
        let steganogram = mocks::expected_steganogram_short();
        let mask = mocks::mock_mask_image();
        let expected = mocks::mock_data_short();

        let result = MaskImageEmbedder::new(&mask).extract(&steganogram);

        assert_eq!(result, expected);
    }

    proptest! {
        #[test]
        fn embedding_and_extraction_is_reversible(transport_pixel_bytes: [u8; 27], mask_pixel_bytes: [u8; 27], bytes: Vec<u8>) {
            // 27 transport bytes allow for embedding at most 27 bytes of information
            // generating Vec provides vectors of length 0..100
            // this means we cover both cases when whole data fits into the transport image
            // and when it does not fit fully
            let transport_pixels = transport_pixel_bytes
                .iter()
                .map(|byte| byte & 127) // ascii uses only 7 bits
                .collect::<Vec<_>>()
                .chunks_exact(3)
                .enumerate()
                .map(|(idx, pixel_bytes)| Pixel::new(0, idx, pixel_bytes[0], pixel_bytes[1], pixel_bytes[2]))
                .collect::<Vec<_>>();

            let mask_pixels = mask_pixel_bytes
                .iter()
                .map(|byte| byte & 127)
                .collect::<Vec<_>>()
                .chunks_exact(3)
                .enumerate()
                .map(|(idx, pixel_bytes)| Pixel::new(0, idx, pixel_bytes[0], pixel_bytes[1], pixel_bytes[2]))
                .collect::<Vec<_>>();

            let transport = PixelMap::new(1, 9, transport_pixels);
            let mask = PixelMap::new(1, 9, mask_pixels);
            let data = Data::new(bytes);

            let embedder = MaskImageEmbedder::new(&mask);
            let steganogram = embedder.embed(&mut data.iter_bits(), &transport);
            let extracted_data = embedder.extract(&steganogram);

            // we cannot guarantee that whole data will fit into the transport image
            // therefore we only check if the data that did fit matches
            let input_data_capped = Data::from_bytes(&data.bytes()[..extracted_data.bytes().len()]);

            assert_eq!(input_data_capped, extracted_data);
        }
    }
}
