mod error;
mod pubmed;

use crate::error::TohayaError;
use crate::pubmed::parse_pubmed;
pub use error::ParseError;

/// Citation file format
pub enum CitationFormat {
    /// BibTeX format
    Bibtex,
    /// PubMed `.nbib` file format
    Pubmed,
    /// Any/unspecified format
    Any,
}

pub fn tohaya<S: AsRef<str>>(input: S, format: CitationFormat) -> Result<String, TohayaError> {
    let library = match format {
        CitationFormat::Pubmed => parse_pubmed(input),
        CitationFormat::Bibtex => unimplemented!(),
        CitationFormat::Any => unimplemented!(),
    }?;
    let yaml = hayagriva::io::to_yaml_str(&library)?;
    Ok(yaml)
}
