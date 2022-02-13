// Copyright (c) 2022 Milen Dzhumerov
use crate::binary::{
    byte_decoder::ByteDecoder,
    entry::Entry,
    header::DataHeader,
    types::{BucketCountType, StringSectionOffsetType, STRING_SECTION_OFFSET_RESERVED},
    utility::{ByteSwappable, Packable},
};

/// A bucket represents three offsets into the headermap file,
/// each offset pointing to the beginning of a NULL-terminated
/// UTF-8 string.
///
/// Offsets are relative to the string section offset defined
/// in the headermap data header.
#[derive(Debug)]
struct Bucket {
    key_offset: StringSectionOffsetType,
    prefix_offset: StringSectionOffsetType,
    suffix_offset: StringSectionOffsetType,
}

#[derive(thiserror::Error, Debug)]
enum BucketParseError {
    #[error("Could not parse string offset for bucket at index `{0}`")]
    OffsetParseError(BucketCountType),
    #[error("Invalid string offset (`{0}`), overlaps the preamble section")]
    InvalidStringOffsetOverlapsPreambleSection(usize),
    #[error("Invalid string offset (`{0}`), out of bounds")]
    InvalidStringOffsetOutOfBounds(usize),
    #[error("Invalid string offset (`{0}`), no NULL terminating byte found")]
    NoStringNullTerminatingByteFound(usize),
    #[error("Invalid string offset (`{0}`), UTF-8 parsing error: {1}")]
    InvalidUtf8String(usize, std::str::Utf8Error),
    #[error("Invalid string offset (`{0}`), unknown internal error")]
    InternalError(usize),
}

impl Packable for Bucket {
    fn packed_size() -> usize {
        3 * std::mem::size_of::<StringSectionOffsetType>()
    }
}

/// Tries to find a NULL-terminated string at a particular string offset.
fn get_string_slice_at_offset<'a>(
    bytes: &'a [u8],
    data_header: &DataHeader,
    relative_string_start_offset: StringSectionOffsetType,
) -> anyhow::Result<&'a str> {
    let absolute_tring_start_offset =
        (data_header.string_section_offset + relative_string_start_offset) as usize;
    let headermap_preamble_size = DataHeader::packed_size() + Bucket::packed_size();

    if absolute_tring_start_offset < headermap_preamble_size {
        anyhow::bail!(
            BucketParseError::InvalidStringOffsetOverlapsPreambleSection(
                absolute_tring_start_offset
            )
        );
    }

    let string_search_bytes = bytes.get(absolute_tring_start_offset..).ok_or(
        BucketParseError::InvalidStringOffsetOutOfBounds(absolute_tring_start_offset),
    )?;
    let null_byte_offset = string_search_bytes.iter().position(|&x| x == 0x0).ok_or(
        BucketParseError::NoStringNullTerminatingByteFound(absolute_tring_start_offset),
    )?;

    let string_bytes = string_search_bytes
        .get(..null_byte_offset)
        .ok_or(BucketParseError::InternalError(absolute_tring_start_offset))?;
    std::str::from_utf8(string_bytes).map_err(|utf8_error| {
        anyhow::Error::new(BucketParseError::InvalidUtf8String(
            absolute_tring_start_offset,
            utf8_error,
        ))
    })
}

/// Returns the headermap entry for a particular bucket. If the bucket is empty, it returns `None`.
/// Returns an error if parsing failed at any point.
pub fn parse_entry_at_bucket_index<'a>(
    bytes: &'a [u8],
    data_header: &DataHeader,
    bucket_index: BucketCountType,
    swap_bytes: bool,
) -> anyhow::Result<Option<Entry<'a>>> {
    let maybe_bucket = Bucket::new_at_index(bytes, bucket_index, swap_bytes)?;
    maybe_bucket
        .map(|bucket| bucket.to_entry(bytes, data_header))
        .transpose()
}

/// Parses a `StringSectionOffsetType` at a particular `bucket_index`.
/// If it encounters a reserved offset (meaning empty), it returns
/// `Option::None`.
fn parse_string_section_offset(
    decoder: &mut ByteDecoder,
    bucket_index: BucketCountType,
    swap_bytes: bool,
) -> anyhow::Result<Option<StringSectionOffsetType>> {
    let offset = decoder
        .advance::<StringSectionOffsetType>()
        .ok_or(BucketParseError::OffsetParseError(bucket_index))?
        .swap_bytes_if(swap_bytes);

    if offset == STRING_SECTION_OFFSET_RESERVED {
        Ok(None)
    } else {
        Ok(Some(offset))
    }
}

impl Bucket {
    /// Returns a `Bucket` if all offsets contains values.
    fn try_new(
        maybe_key_offset: Option<StringSectionOffsetType>,
        maybe_prefix_offset: Option<StringSectionOffsetType>,
        maybe_suffix_offset: Option<StringSectionOffsetType>,
    ) -> Option<Bucket> {
        let key_offset = maybe_key_offset?;
        let prefix_offset = maybe_prefix_offset?;
        let suffix_offset = maybe_suffix_offset?;
        Some(Bucket {
            key_offset,
            prefix_offset,
            suffix_offset,
        })
    }

    /// Parses they string offsets for a particular `bucket_index`
    /// and returns a `Bucket` if all offsets contain non-reserved values.
    fn new_at_index(
        bytes: &[u8],
        bucket_index: BucketCountType,
        swap_bytes: bool,
    ) -> anyhow::Result<Option<Bucket>> {
        let offset = DataHeader::packed_size() + (bucket_index as usize) * Bucket::packed_size();
        let mut decoder = ByteDecoder { bytes, offset };

        let maybe_key_offset = parse_string_section_offset(&mut decoder, bucket_index, swap_bytes)?;
        let maybe_prefix_offset =
            parse_string_section_offset(&mut decoder, bucket_index, swap_bytes)?;
        let maybe_suffix_offset =
            parse_string_section_offset(&mut decoder, bucket_index, swap_bytes)?;

        Ok(Bucket::try_new(
            maybe_key_offset,
            maybe_prefix_offset,
            maybe_suffix_offset,
        ))
    }

    /// Converts a `Bucket` into an `Entry` by trying to convert each string offset into a string slice.
    fn to_entry<'a>(&self, bytes: &'a [u8], data_header: &DataHeader) -> anyhow::Result<Entry<'a>> {
        let key = get_string_slice_at_offset(bytes, data_header, self.key_offset)?;
        let prefix = get_string_slice_at_offset(bytes, data_header, self.prefix_offset)?;
        let suffix = get_string_slice_at_offset(bytes, data_header, self.suffix_offset)?;

        Ok(Entry {
            key,
            prefix,
            suffix,
        })
    }
}
