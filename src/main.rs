use anyhow::Result;
use clap::Parser;

use query_def_safety::models::{QueryParam, QueryTemplate};
use query_def_safety::template::render_template;
use query_def_safety::validator::validate_param;

#[derive(Parser, Debug)]
#[command(name = "query-def-safety", about = "SQL query template engine with safety checks")]
struct Cli {
    #[arg(long, help = "SQL template with {{param}} placeholders")]
    template: String,

    #[arg(long, help = "Parameters as JSON object, e.g. '{\"name\":\"value\"}'")]
    params: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let params_map: std::collections::HashMap<String, String> =
        serde_json::from_str(&cli.params)?;

    let params: Vec<QueryParam> = params_map
        .into_iter()
        .map(|(k, v)| QueryParam::text(k, v))
        .collect();

    for param in &params {
        validate_param(param)?;
    }

    let template = QueryTemplate::new("cli_query", &cli.template, params);
    let result = render_template(&template)?;

    println!("{}", result);

    Ok(())
}
