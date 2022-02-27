use crate::binary::Entry;

// NB: Avoid pulling in additional crate deps to just print out a JSON dict
pub fn print_json_entries<W>(writer: &mut W, entries: &[Entry], tab_size: u8) -> anyhow::Result<()>
where
    W: std::io::Write,
{
    writeln!(writer, "{{")?;

    for (index, entry) in entries.iter().enumerate() {
        for _ in 0..tab_size {
            write!(writer, " ")?;
        }

        write!(writer, "\"")?;
        write_json_escaped_string(writer, entry.key)?;
        write!(writer, "\": ")?;

        write!(writer, "\"")?;
        write_json_escaped_string(writer, entry.prefix)?;
        write_json_escaped_string(writer, entry.suffix)?;
        write!(writer, "\"")?;

        if index != entries.len() - 1 {
            write!(writer, ",")?;
        }

        writeln!(writer)?;
    }

    writeln!(writer, "}}")?;

    Ok(())
}

fn write_json_escaped_string<W>(writer: &mut W, string: &str) -> anyhow::Result<()>
where
    W: std::io::Write,
{
    for byte in string.as_bytes() {
        if *byte < 0x20 {
            write!(writer, "{:04X}", *byte)?;
        } else if *byte == b'\\' || *byte == b'"' {
            writer.write_all(&[b'\\', *byte])?;
        } else {
            writer.write_all(&[*byte])?;
        }
    }
    Ok(())
}
