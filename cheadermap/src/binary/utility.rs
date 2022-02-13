// Copyright (c) 2022 Milen Dzhumerov

/// Trait for compositve types which can be serialized without padding.
pub trait Packable {
    /// Returns the packed size of a type (i.e., without any padding)
    fn packed_size() -> usize;
}

/// Convenience trait for types which can be byte swapped.
pub trait ByteSwappable: Copy {
    /// Swaps `self` if `swapped` is true, otherwise returns `self`
    fn swap_bytes_if(self, swapped: bool) -> Self;
}

impl ByteSwappable for u16 {
    fn swap_bytes_if(self, swapped: bool) -> u16 {
        if swapped {
            self.swap_bytes()
        } else {
            self
        }
    }
}
impl ByteSwappable for u32 {
    fn swap_bytes_if(self, swapped: bool) -> u32 {
        if swapped {
            self.swap_bytes()
        } else {
            self
        }
    }
}

/// Trait for primitive types which can be decoded from raw bytes.
pub trait DecodablePrimitive: Sized {
    /// Decodes without performing any endian adjustments (i.e., reinterprets bytes).
    fn decode_from_bytes(bytes: &[u8]) -> Option<Self>;
}

// TODO: Look into eliminating duplication once const generics make it into stable

impl DecodablePrimitive for u16 {
    fn decode_from_bytes(bytes: &[u8]) -> Option<Self> {
        let x = bytes
            .get(0..std::mem::size_of::<Self>())
            .and_then(|s| <&[u8; std::mem::size_of::<Self>()]>::try_from(s).ok())?;
        Some(Self::from_ne_bytes(*x))
    }
}

impl DecodablePrimitive for u32 {
    fn decode_from_bytes(bytes: &[u8]) -> Option<Self> {
        let x = bytes
            .get(0..std::mem::size_of::<Self>())
            .and_then(|s| <&[u8; std::mem::size_of::<Self>()]>::try_from(s).ok())?;
        Some(Self::from_ne_bytes(*x))
    }
}
