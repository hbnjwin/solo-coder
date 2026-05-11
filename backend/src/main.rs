use anyhow::Result;
use clap::Parser;

mod models;
mod paginator;
mod exporter;

pub use models::*;
pub use paginator::*;
pub use exporter::*;

#[derive(Parser, Debug)]
#[command(name = "export-tool", about = "Large dataset export with retry capability")]
struct Cli {
    #[arg(long, default_value = "100")]
    page_size: usize,

    #[arg(long)]
    resume_from: Option<String>,

    #[arg(long, default_value = "offset")]
    strategy: String,

    #[arg(long, default_value = "output.json")]
    output: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let dataset = models::generate_sample_dataset(500);

    let request = models::ExportRequest {
        page_size: cli.page_size,
        strategy: if cli.strategy == "cursor" {
            models::PaginationStrategy::Cursor
        } else {
            models::PaginationStrategy::Offset
        },
        output_path: cli.output.clone(),
    };

    let result = match cli.resume_from {
        Some(ref state_path) => exporter::resume_export(&dataset, &request, state_path)?,
        None => exporter::export_all(&dataset, &request)?,
    };

    println!("Export complete: {} records across {} pages", result.total_records, result.pages_written);
    Ok(())
}
