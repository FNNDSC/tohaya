mod error;
mod pubmed;

use crate::error::TohayaError;
use crate::pubmed::parse_pubmed;
pub use error::ParseError;
use itertools::Itertools;

/// Citation file format
#[derive(Copy, Clone)]
pub enum CitationFormat {
    /// BibTeX format
    Bibtex,
    /// PubMed `.nbib` file format
    Pubmed,
    /// Any/unspecified format
    Any,
}

pub fn tohaya<S: AsRef<str>, I: IntoIterator<Item = S>>(
    inputs: I,
    format: CitationFormat,
) -> Result<String, TohayaError> {
    let library = inputs
        .into_iter()
        .map(|s| to_library(s, format).map(|l| l.into_iter()))
        .flatten_ok()
        .try_collect()?;
    let yaml = hayagriva::io::to_yaml_str(&library)?;
    Ok(yaml)
}

fn to_library<S: AsRef<str>>(
    input: S,
    format: CitationFormat,
) -> Result<hayagriva::Library, ParseError> {
    match format {
        CitationFormat::Pubmed => parse_pubmed(input),
        CitationFormat::Bibtex => unimplemented!(),
        CitationFormat::Any => unimplemented!(),
    }
}
