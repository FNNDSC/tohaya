mod path_arg;

use crate::path_arg::PathArg;
use clap::Parser;
use std::io::Write;
use tohaya::{CitationFormat, tohaya};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Append to output file
    #[clap(short, long)]
    append: bool,
    /// Output file where to write hayagriva YAML
    #[clap(short, long, default_value = "-")]
    output_file: PathArg,
    /// Input citation file
    #[clap(required = true)]
    input_files: Vec<PathArg>,
}

fn main() -> color_eyre::Result<()> {
    let args = Cli::parse();
    let mut out = args
        .output_file
        .open(args.append)
        .map(std::io::BufWriter::new)?;
    let inputs: Vec<_> = args
        .input_files
        .into_iter()
        .map(|s| s.read())
        .collect::<Result<_, _>>()?;
    let output = tohaya(inputs, CitationFormat::Pubmed)?;
    out.write_all(output.as_ref())?;
    Ok(())
}
