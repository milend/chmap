// Copyright (c) 2022 Milen Dzhumerov

mod bucket;
mod byte_decoder;
mod entry;
mod header;
mod types;
mod utility;

pub use entry::Entry;

/// Enumerates the entries in the headermap. If the header cannot be parsed,
/// returns an error. If `fail_on_bucket_error` is `true`, then on the first
/// bucket parsing error, the method will return with an error. Otherwise,
/// enumeration continues.
///
/// `enumerator` returns a boolean whether to continue enumerating.
pub fn headermap_enumerate_entries<'a, F>(
    bytes: &'a [u8],
    fail_on_bucket_error: bool,
    mut enumerator: F,
) -> anyhow::Result<()>
where
    F: FnMut(Entry<'a>) -> bool,
{
    let parse_result = header::parse_header(bytes)?;

    for bucket_index in 0..parse_result.header.bucket_count {
        let entry_result = bucket::parse_entry_at_bucket_index(
            bytes,
            &parse_result.header,
            bucket_index,
            parse_result.swap_bytes,
        );

        match entry_result {
            Ok(maybe_entry) => {
                // If `maybe_entry` is `None`, it means the hash bucket was empty,
                // it's not an error condition.
                if let Some(entry) = maybe_entry {
                    let continue_enumerating = enumerator(entry);
                    if !continue_enumerating {
                        break;
                    }
                }
            }
            Err(parse_error) => {
                if fail_on_bucket_error {
                    return Err(parse_error);
                }
            }
        }
    }

    Ok(())
}

/// Parses a headermap and returns a list of entries.
/// If `fail_on_bucket_error` is `true`, then if there's
/// a single bucket error, the method will return an error.
/// Otherwise, any bucket errors are ignored and partial
/// results would be returned.
pub fn parse_headermap(bytes: &[u8], fail_on_bucket_error: bool) -> anyhow::Result<Vec<Entry>> {
    let mut accumulator = Vec::new();
    headermap_enumerate_entries(bytes, fail_on_bucket_error, |entry| {
        accumulator.push(entry);
        true
    })?;
    Ok(accumulator)
}

/// Prints the headermap entries, one per line in the format `key -> prefix + suffix`.
pub fn print_headermap<W, P>(writer: &mut W, path: P) -> anyhow::Result<()>
where
    W: std::io::Write,
    P: AsRef<std::path::Path>,
{
    let file_bytes = std::fs::read(path.as_ref())?;
    let mut entries = parse_headermap(&file_bytes, true)?;
    entries.sort_by(|lhs, rhs| lhs.key.cmp(rhs.key));
    for entry in entries {
        writeln!(writer, "{} -> {}{}", entry.key, entry.prefix, entry.suffix)?;
    }
    Ok(())
}
