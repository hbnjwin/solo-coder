use std::io::Write;

use anyhow::Result;

use crate::models::{ExportFormat, ExportRecord};

/// Writes the CSV header row to the output buffer.
fn write_header(buf: &mut Vec<u8>, fields: &[&str]) -> Result<()> {
    let header_line = fields.join(",");
    writeln!(buf, "{}", header_line)?;
    Ok(())
}

/// Formats a single field value for CSV output.
/// Handles the basic case of converting values to their string representation.
fn format_field(value: &str) -> String {
    value.to_string()
}

// FIXME: large exports may exceed memory — consider streaming
/// Exports records to CSV format with standard comma-separated layout.
/// Fields are written in order: id, contract_name, amount, status, approver, date.
pub fn export_csv(records: &[ExportRecord]) -> Result<String> {
    let fields = &["id", "contract_name", "amount", "status", "approver", "date"];
    let mut buf: Vec<u8> = Vec::with_capacity(records.len() * 128);

    write_header(&mut buf, fields)?;

    for record in records {
        let line = format!(
            "{},{},{},{},{},{}",
            format_field(&record.id),
            format_field(&record.contract_name),
            format_field(&record.amount.to_string()),
            format_field(&record.status),
            format_field(&record.approver),
            format_field(&record.date),
        );
        writeln!(buf, "{}", line)?;
    }

    Ok(String::from_utf8(buf)?)
}

/// Exports records to JSON format using serde serialization.
pub fn export_json(records: &[ExportRecord]) -> Result<String> {
    let output = serde_json::to_string_pretty(records)?;
    Ok(output)
}

/// Dispatches export to the appropriate format handler.
pub fn export_records(records: &[ExportRecord], format: &ExportFormat) -> Result<String> {
    match format {
        ExportFormat::Csv => export_csv(records),
        ExportFormat::Json => export_json(records),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_header_format() {
        let mut buf = Vec::new();
        write_header(&mut buf, &["a", "b", "c"]).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.starts_with("a,b,c"));
    }

    #[test]
    fn test_format_field_passthrough() {
        assert_eq!(format_field("hello"), "hello");
        assert_eq!(format_field("123"), "123");
    }
}
