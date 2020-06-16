pub const ALPHABET_ENCODINGS: [u64; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 1, 255, 255, 255, 2, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 3, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 0, 255, 1, 255, 255, 255, 2, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    3, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255,
];

#[inline]
pub fn encode_char(c: char) -> u64 {
    ALPHABET_ENCODINGS[c as usize]
}

#[cfg(test)]
mod test {
    use crate::alphabet::encode_char;

    #[test]
    fn test_encode_char() {
        for (index, character) in ['A', 'C', 'G', 'T'].iter().enumerate() {
            let encoded = encode_char(*character);
            assert_eq!(encoded, index as u64, "encode {}", character);
        }

        for (index, character) in ['a', 'c', 'g', 't'].iter().enumerate() {
            let encoded = encode_char(*character);
            assert_eq!(encoded, index as u64, "encode {}", character);
        }
    }

    #[test]
    fn test_encode_invalid_char() {
        for character in ['F', 'K', 'z', 'y'].iter() {
            let encoded = encode_char(*character);
            assert_eq!(encoded, 255);
        }
    }
}
