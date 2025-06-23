mod path_arg;

use crate::path_arg::PathArg;
use clap::{Args, Parser};
use std::io::Write;
use tohaya::{tohaya, CitationFormat};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(flatten)]
    format: FormatArg,

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

#[derive(Args, Copy, Clone)]
#[group(required = false, multiple = false)]
struct FormatArg {
    /// Specify input file is BibLaTeX format (.bib file)
    #[clap(short, long)]
    bibtex: bool,
    /// Specify input file is PubMed format, (.nbib file)
    #[clap(short, long)]
    pubmed: bool,
}

impl FormatArg {
    fn resolve(self) -> Option<CitationFormat> {
        if self.bibtex {
            Some(CitationFormat::Bibtex)
        } else if self.pubmed {
            Some(CitationFormat::Pubmed)
        } else {
            None
        }
    }
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
    let output = tohaya(inputs, args.format.resolve())?;
    out.write_all(output.as_ref())?;
    Ok(())
}
