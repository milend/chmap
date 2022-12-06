// Copyright (c) 2022 Milen Dzhumerov

use crate::binary::{types::*, utility::Packable};

const MAGIC_NATIVE_ENDIAN: MagicType = 0x68_6D_61_70; // 'hmap' (in big endian)
const MAGIC_NON_NATIVE_ENDIAN: MagicType = MAGIC_NATIVE_ENDIAN.swap_bytes();

/// Represents the headermap file header.
///
/// It implements v1 of the Clang header map format. For reference, see the
/// following files in the Clang source tree:
///  - HeaderMap.h
///  - HeaderMap.cpp
///  - HeaderMapTypes.h
#[derive(Debug)]
pub struct DataHeader {
    pub magic: MagicType,
    pub version: VersionType,
    pub reserved: ReservedType,
    pub string_section_offset: StringSectionOffsetType,
    pub string_count: StringCountType,
    pub bucket_count: BucketCountType,
    pub max_value_length: MaxValueLength,
}

impl Packable for DataHeader {
    fn packed_size() -> usize {
        std::mem::size_of::<MagicType>()
            + std::mem::size_of::<VersionType>()
            + std::mem::size_of::<ReservedType>()
            + std::mem::size_of::<StringSectionOffsetType>()
            + std::mem::size_of::<StringCountType>()
            + std::mem::size_of::<BucketCountType>()
            + std::mem::size_of::<MaxValueLength>()
    }
}

use crate::binary::{byte_decoder::ByteDecoder, utility::ByteSwappable};
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum DataHeaderError {
    #[error("Magic value is unknown, found`{0}`")]
    UnknownMagic(MagicType),
    #[error("Magic value missing")]
    MissingMagic,
    #[error("Version value missing")]
    MissingVersion,
    #[error("Unsupported version, found `{0}`")]
    UnsupportedVersion(VersionType),
    #[error("Reserved value missing")]
    MissingReservedValue,
    #[error("Unsupported reserved value, found `{0}`")]
    UnsupportedReserved(ReservedType),
    #[error("String section offset value missing")]
    MissingStringSectionOffsetValue,
    #[error("String count value missing")]
    MissingStringCountValue,
    #[error("Bucket count value missing")]
    MissingBucketCountValue,
    #[error("Bucket count is not a power of two, found `{0}`")]
    BucketCountNotPowerOfTwo(BucketCountType),
    #[error("Max value length value missing")]
    MissingMaxValueLengthValue,
    #[error("String section is out of bounds, found `{0}`")]
    StringSectionOffsetOutOfBounds(StringSectionOffsetType),
}

#[derive(Debug)]
pub struct DataHeaderParseResult {
    pub header: DataHeader,
    /// If the endianness of the binary data is different,
    /// then `swap_bytes` would be `true`.
    ///
    /// Indicates that all values needs to be byte swapped
    /// when deserializing.
    pub swap_bytes: bool,
}

pub fn parse_header(bytes: &[u8]) -> anyhow::Result<DataHeaderParseResult> {
    let mut decoder = ByteDecoder { bytes, offset: 0 };
    let magic = decoder
        .advance::<MagicType>()
        .ok_or(DataHeaderError::MissingMagic)?;

    if magic != MAGIC_NATIVE_ENDIAN && magic != MAGIC_NON_NATIVE_ENDIAN {
        anyhow::bail!(DataHeaderError::UnknownMagic(magic));
    }

    let swap_bytes = magic == MAGIC_NON_NATIVE_ENDIAN;

    let version = decoder
        .advance::<VersionType>()
        .ok_or(DataHeaderError::MissingVersion)?
        .swap_bytes_if(swap_bytes);
    if version != VERSION_1 {
        anyhow::bail!(DataHeaderError::UnsupportedVersion(version));
    }

    let reserved = decoder
        .advance::<ReservedType>()
        .ok_or(DataHeaderError::MissingReservedValue)?
        .swap_bytes_if(swap_bytes);
    if reserved != RESERVED {
        anyhow::bail!(DataHeaderError::UnsupportedReserved(reserved));
    }

    let string_section_offset = decoder
        .advance::<StringSectionOffsetType>()
        .ok_or(DataHeaderError::MissingStringSectionOffsetValue)?
        .swap_bytes_if(swap_bytes);
    if bytes.len() < (string_section_offset as usize) {
        anyhow::bail!(DataHeaderError::StringSectionOffsetOutOfBounds(
            string_section_offset
        ));
    }

    let string_count = decoder
        .advance::<StringCountType>()
        .ok_or(DataHeaderError::MissingStringCountValue)?
        .swap_bytes_if(swap_bytes);

    let bucket_count = decoder
        .advance::<BucketCountType>()
        .ok_or(DataHeaderError::MissingBucketCountValue)?
        .swap_bytes_if(swap_bytes);
    if !bucket_count.is_power_of_two() {
        anyhow::bail!(DataHeaderError::BucketCountNotPowerOfTwo(bucket_count));
    }

    let max_value_length = decoder
        .advance::<MaxValueLength>()
        .ok_or(DataHeaderError::MissingMaxValueLengthValue)?
        .swap_bytes_if(swap_bytes);

    let header = DataHeader {
        magic: MAGIC_NATIVE_ENDIAN,
        version,
        reserved,
        string_section_offset,
        string_count,
        bucket_count,
        max_value_length,
    };

    Ok(DataHeaderParseResult { header, swap_bytes })
}
