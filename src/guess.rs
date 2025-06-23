use std::cell::LazyCell;
use regex_lite::Regex;
use crate::CitationFormat;

const PUBMED: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^PMID ?- +\d+").unwrap()
});

const BIBTEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"@[a-z]+\{").unwrap()
});

pub(crate) fn guess_format(text: &str) -> Option<CitationFormat> {
    for line in text.split('\n') {
        if PUBMED.is_match(line) {
            return Some(CitationFormat::Pubmed)
        } else if BIBTEX.is_match(line) {
            return Some(CitationFormat::Bibtex)
        }
    };
    None
}

#[cfg(test)]
mod tests {
    use crate::CitationFormat;
    use crate::guess::guess_format;

    #[test]
    fn test_guess_bibtex() {
        let text = r#"@article{lander1966counterexample,
  title={Counterexample to Euler's conjecture on sums of like powers},
  author={Lander, LJ and Parkin, TR},
  year={1966}
}"#;
        assert_eq!(guess_format(text), Some(CitationFormat::Bibtex));
    }

    #[test]
    fn test_guess_pubmed() {
        let text = r#"PMID- 31181385
OWN - NLM
STAT- MEDLINE
TI  - Fantastic yeasts and where to find them"#;
        assert_eq!(guess_format(text), Some(CitationFormat::Pubmed));
    }

    #[test]
    fn test_guess_unknown() {
        let text = "something";
        assert_eq!(guess_format(text), None);
    }
}