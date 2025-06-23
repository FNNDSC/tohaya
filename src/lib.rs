mod error;
mod pubmed;
mod guess;

use crate::error::TohayaError;
use crate::pubmed::parse_pubmed;
pub use error::ParseError;
use itertools::Itertools;
use crate::guess::guess_format;

/// Citation file formats supported by _tohaya_.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CitationFormat {
    /// BibTeX format
    Bibtex,
    /// PubMed `.nbib` file format
    Pubmed,
}

pub fn tohaya<S: AsRef<str>, I: IntoIterator<Item = S>>(
    inputs: I,
    format: Option<CitationFormat>,
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
    format: Option<CitationFormat>,
) -> Result<hayagriva::Library, ParseError> {
    let text = input.as_ref();
    let format = if let Some(f) = format {
        f
    } else {
        guess_format(text).ok_or_else(|| ParseError("Unknown format".to_string()))?
    };
    match format {
        CitationFormat::Pubmed => parse_pubmed(input),
        CitationFormat::Bibtex => from_bibtex(input),
    }
}

fn from_bibtex<S: AsRef<str>>(input: S) -> Result<hayagriva::Library, ParseError> {
    hayagriva::io::from_biblatex_str(input.as_ref()).map_err(|e| {
        let msg = e.into_iter().map(|e| e.to_string()).join(", ");
        ParseError(msg)
    })
}
