// Copyright (c) 2022 Milen Dzhumerov

use crate::binary::utility::*;

pub struct ByteDecoder<'a> {
    pub bytes: &'a [u8],
    pub offset: usize,
}

impl<'a> ByteDecoder<'a> {
    pub fn advance<T: DecodablePrimitive>(&mut self) -> Option<T> {
        let slice = self.bytes.get(self.offset..)?;

        let decoded_value = T::decode_from_bytes(slice);
        if decoded_value.is_some() {
            self.offset += std::mem::size_of::<T>();
        }

        decoded_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let bytes = &[0xAB, 0xCD];

        let mut byte_decoder = ByteDecoder { bytes, offset: 0 };

        let decoded_value = byte_decoder.advance::<u16>().unwrap();
        assert_eq!(decoded_value, u16::from_ne_bytes(*bytes));
        assert_eq!(byte_decoder.offset, std::mem::size_of::<u16>());

        let empty_value = byte_decoder.advance::<u16>();
        assert!(empty_value.is_none());
    }

    #[test]
    fn test_empty_bytes() {
        let bytes = &[];

        let mut byte_decoder = ByteDecoder { bytes, offset: 0 };

        let empty_value = byte_decoder.advance::<u16>();
        assert!(empty_value.is_none());
    }

    #[test]
    fn test_out_of_bounds() {
        let bytes = &[0xAB, 0xCD];

        let mut byte_decoder = ByteDecoder {
            bytes,
            offset: 1024,
        };

        let empty_value = byte_decoder.advance::<u16>();
        assert!(empty_value.is_none());
    }
}
