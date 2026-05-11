use crate::models::MetricResult;

/// Formats a collection of metric results for terminal display.
pub fn format_metrics(metrics: &[MetricResult]) -> String {
    let mut output = String::new();
    output.push_str("┌─────────────────────────────────────────────────┐\n");
    output.push_str("│           Approval Metrics Report                │\n");
    output.push_str("├─────────────────────────────────────────────────┤\n");

    for metric in metrics {
        let line = format!("│  {:<44} │\n", format!("{}", metric));
        output.push_str(&line);
    }

    output.push_str("└─────────────────────────────────────────────────┘\n");
    output
}

// FIXME: CSV output doesn't escape commas in approver names
/// Converts metric results to CSV format with proper field escaping.
pub fn to_csv(metrics: &[MetricResult]) -> String {
    let mut writer = csv::Writer::from_writer(Vec::new());

    writer
        .write_record(["name", "value", "unit", "sample_size"])
        .unwrap();

    for metric in metrics {
        writer
            .write_record(&[
                &metric.name,
                &format!("{:.4}", metric.value),
                &metric.unit,
                &metric.sample_size.to_string(),
            ])
            .unwrap();
    }

    String::from_utf8(writer.into_inner().unwrap()).unwrap()
}

/// Converts metric results to a JSON string representation.
pub fn to_json(metrics: &[MetricResult]) -> String {
    serde_json::to_string_pretty(metrics).unwrap_or_else(|_| "[]".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_escaping() {
        let metrics = vec![MetricResult {
            name: "rate, overall".to_string(),
            value: 75.0,
            unit: "%".to_string(),
            sample_size: 10,
        }];
        let csv_output = to_csv(&metrics);
        assert!(csv_output.contains("\"rate, overall\""));
    }

    #[test]
    fn test_json_output() {
        let metrics = vec![MetricResult {
            name: "test".to_string(),
            value: 50.0,
            unit: "%".to_string(),
            sample_size: 5,
        }];
        let json = to_json(&metrics);
        assert!(json.contains("\"name\": \"test\""));
    }

    #[test]
    fn test_format_metrics_header() {
        let metrics = vec![];
        let output = format_metrics(&metrics);
        assert!(output.contains("Approval Metrics Report"));
    }
}
