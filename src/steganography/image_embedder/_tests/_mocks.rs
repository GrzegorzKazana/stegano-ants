use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;
use crate::steganography::data::Data;

pub fn mock_data() -> Data {
    // message is `hello world` (88 bits)
    Data::new(vec![
        104, // 01101000 h
        101, // 01100101 e
        108, // 01101100 l
        108, // 01101100 l
        111, // 01101111 o
        32,  // 00100000 ' '
        119, // 01110111 w
        111, // 01101111 o
        114, // 01110010 r
        108, // 01101100 l
        100, // 01100100 d
    ])
}

pub fn mock_transport_image() -> PixelMap {
    PixelMap::new(
        3,
        2,
        vec![
            Pixel::new(0, 0, 0b01111000, 0b00100101, 0b00101010),
            Pixel::new(1, 0, 0b11000010, 0b01110100, 0b10000100),
            Pixel::new(2, 0, 0b11011001, 0b00101000, 0b10001110),
            Pixel::new(0, 1, 0b11001011, 0b10010111, 0b10111000),
            Pixel::new(1, 1, 0b00001011, 0b11101110, 0b01100001),
            Pixel::new(2, 1, 0b00111011, 0b00001000, 0b01011000),
        ],
    )
}

pub fn mock_mask_image() -> PixelMap {
    PixelMap::new(
        3,
        2,
        vec![
            Pixel::new(0, 0, 46, 59, 6),  // 5, 5, 2 bits
            Pixel::new(1, 0, 25, 52, 15), // 4, 5, 4 bits
            Pixel::new(2, 0, 2, 21, 60),  // 1, 4, 5 bits
            Pixel::new(0, 1, 4, 32, 12),  // 2, 5, 3 bits
            Pixel::new(1, 1, 50, 34, 22), // 5, 5, 4 bits
            Pixel::new(2, 1, 61, 14, 65), // 5, 3, 6 bits, 73 bits total
        ],
    )
}

pub fn expected_steganogram() -> PixelMap {
    PixelMap::new(
        3,
        2,
        vec![
            Pixel::new(0, 0, 0b01101101, 0b00100001, 0b00101010), // 0b011xxxxx + 0b01101, 0b001xxxxx + 0b00001, 0b001010xx + 0b10
            Pixel::new(1, 0, 0b11000101, 0b01101101, 0b10001000), // 0b1100xxxx + 0b0101,  0b011xxxxx + 0b01101, 0b1000xxxx + 0b1000
            Pixel::new(2, 0, 0b11011001, 0b00101011, 0b10000011), // 0b1101100x + 0b1,     0b0010xxxx + 0b1011,  0b100xxxxx + 0b00011
            Pixel::new(0, 1, 0b11001001, 0b10011100, 0b10111100), // 0b110010xx + 0b01,    0b100xxxxx + 0b11100, 0b10111xxx + 0b100
            Pixel::new(1, 1, 0b00000001, 0b11111011, 0b01101011), // 0b000xxxxx + 0b00001, 0b111xxxxx + 0b11011, 0b0110xxxx + 0b1011
            Pixel::new(2, 1, 0b00101111, 0b00001011, 0b01100100), // 0b001xxxxx + 0b01111, 0b00001xxx + 0b011,   0b01xxxxxx + 0b100100
        ],
    )
}

pub fn mock_data_very_short() -> Data {
    // very_short - meaning that whole message + message end token fits completely
    // message is `hello` (40 bits)
    Data::new(vec![
        104, // 01101000 h
        101, // 01100101 e
        108, // 01101100 l
        108, // 01101100 l
        111, // 01101111 o
    ])
}

pub fn expected_steganogram_very_short() -> PixelMap {
    PixelMap::new(
        3,
        2,
        vec![
            Pixel::new(0, 0, 0b01101101, 0b00100001, 0b00101010), // 0b011xxxxx + 0b01101, 0b001xxxxx + 0b00001, 0b001010xx + 0b10
            Pixel::new(1, 0, 0b11000101, 0b01101101, 0b10001000), // 0b1100xxxx + 0b0101,  0b011xxxxx + 0b01101, 0b1000xxxx + 0b1000
            Pixel::new(2, 0, 0b11011001, 0b00101011, 0b10000011), // 0b1101100x + 0b1,     0b0010xxxx + 0b1011,  0b100xxxxx + 0b00011
            Pixel::new(0, 1, 0b11001001, 0b10011111, 0b10111111), // 0b110010xx + 0b01,    0b100xxxyy + 0b11111, 0b10111yyy + 0b111     y - stands for message end token
            Pixel::new(1, 1, 0b00011111, 0b11101110, 0b01100001), // 0b000yyy11 + 0b111__, 0b11101110, 0b01100001
            Pixel::new(2, 1, 0b00111011, 0b00001000, 0b01011000), // 0b00111011, 0b00001000, 0b01011000
        ],
    )
}

pub fn mock_data_short() -> Data {
    // short - meaning that whole message + part of end token fits
    // message is `hello worl` (72 bits)
    Data::new(vec![
        104, // 01101000 h
        101, // 01100101 e
        108, // 01101100 l
        108, // 01101100 l
        111, // 01101111 o
        32,  // 00100000 ' '
        119, // 01110111 w
        111, // 01101111 o
        114, // 01110010 r
    ])
}

pub fn expected_steganogram_short() -> PixelMap {
    PixelMap::new(
        3,
        2,
        vec![
            Pixel::new(0, 0, 0b01101101, 0b00100001, 0b00101010), // 0b011xxxxx + 0b01101, 0b001xxxxx + 0b00001, 0b001010xx + 0b10
            Pixel::new(1, 0, 0b11000101, 0b01101101, 0b10001000), // 0b1100xxxx + 0b0101,  0b011xxxxx + 0b01101, 0b1000xxxx + 0b1000
            Pixel::new(2, 0, 0b11011001, 0b00101011, 0b10000011), // 0b1101100x + 0b1,     0b0010xxxx + 0b1011,  0b100xxxxx + 0b00011
            Pixel::new(0, 1, 0b11001001, 0b10011100, 0b10111100), // 0b110010xx + 0b01,    0b100xxxxx + 0b11100, 0b10111xxx + 0b100
            Pixel::new(1, 1, 0b00000001, 0b11111011, 0b01101011), // 0b000xxxxx + 0b00001, 0b111xxxxx + 0b11011, 0b0110xxxx + 0b1011
            Pixel::new(2, 1, 0b00101111, 0b00001011, 0b01100101), // 0b001xxxxx + 0b01111, 0b00001xxx + 0b011,   0b01xxxxxy + 0b100101      y - stands for message end token
        ],
    )
}
