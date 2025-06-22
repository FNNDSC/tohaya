use biblib::CitationParser;
use hayagriva::types::{
    Date, EntryType, FormatString, MaybeTyped, Person, Publisher, QualifiedUrl,
};
use std::sync::LazyLock;
use unic_langid_impl::LanguageIdentifier;

pub(crate) fn parse_pubmed<S: AsRef<str>>(
    input: S,
) -> Result<hayagriva::Library, crate::ParseError> {
    let parser = biblib::PubMedParser::new();
    let citations = parser.parse(input.as_ref())?;
    citations.into_iter().map(to_entry).collect()
}

fn to_entry(citation: biblib::Citation) -> Result<hayagriva::Entry, crate::ParseError> {
    let mut errors = Vec::with_capacity(10);
    let key = key_of(&citation);
    let entry_type = entry_type_of(&citation)?;
    let mut entry = hayagriva::Entry::new(&key, entry_type);
    entry.set_title(FormatString::with_value(citation.title));
    entry.set_authors(citation.authors.into_iter().map(to_person).collect());

    if let Some(journal) = citation
        .journal_abbr
        .as_deref()
        .or(citation.journal.as_deref())
    {
        let mut parent = hayagriva::Entry::new(journal, EntryType::Periodical);
        if let Some(value) = citation.journal {
            let short = citation.journal_abbr.unwrap_or_else(|| value.to_string());
            let title = FormatString::with_short(value, short);
            parent.set_title(title);
        } else if let Some(value) = citation.journal_abbr {
            let title = FormatString::with_value(value);
            parent.set_title(title);
        };
        if let Some(volume) = citation.volume {
            parent.set_volume(MaybeTyped::String(volume));
        }
        if let Some(issue) = citation.issue {
            parent.set_issue(MaybeTyped::String(issue));
        }
        if let Some(publisher) = citation.publisher {
            let name = FormatString::with_value(publisher);
            parent.set_publisher(Publisher::new(Some(name), None));
        }
        entry.set_parents(vec![parent]);
    }

    if let Some(date) = citation.date {
        entry.set_date(Date {
            year: date.year,
            month: date.month,
            day: date.day,
            approximate: false,
        });
    }
    if let Some(pages) = citation.pages {
        entry.set_page_range(MaybeTyped::String(pages));
    }
    if let Some(issn) = citation.issn.into_iter().next() {
        entry.set_issn(issn);
    }
    if let Some(doi) = citation.doi {
        entry.set_doi(doi);
    }
    if let Some(pmid) = citation.pmid {
        entry.set_pmid(pmid);
    }
    if let Some(pmcid) = citation.pmc_id {
        entry.set_pmcid(pmcid);
    }
    if let Some(abstract_) = citation.abstract_text {
        entry.set_abstract_(FormatString::with_value(abstract_));
    }
    if let Some(url) = citation.urls.first() {
        match url::Url::parse(url) {
            Ok(value) => {
                entry.set_url(QualifiedUrl::new(value, None));
            }
            Err(e) => {
                errors.push(e);
            }
        }
    }
    if let Some(language) = &citation.language {
        if language == "eng" {
            entry.set_language(ENGLISH.clone());
        }
    }
    Ok(entry)
}

fn to_person(author: biblib::Author) -> Person {
    Person {
        name: author.family_name,
        given_name: Some(author.given_name),
        prefix: None,
        suffix: None,
        alias: None,
    }
}

fn key_of(citation: &biblib::Citation) -> String {
    if let Some(author) = citation.authors.first() {
        let name = author
            .family_name
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "");
        if let Some(date) = &citation.date {
            format!("{name}{}", date.year)
        } else {
            name
        }
    } else {
        "key".to_string()
    }
}

fn entry_type_of(citation: &biblib::Citation) -> Result<EntryType, crate::ParseError> {
    if citation
        .citation_type
        .iter()
        .any(|s| s == "Journal Article")
    {
        Ok(EntryType::Article)
    } else {
        Err(crate::ParseError(format!(
            "Unsupported citation types: {}",
            citation.citation_type.join(", ")
        )))
    }
}

static ENGLISH: LazyLock<LanguageIdentifier> =
    LazyLock::new(|| LanguageIdentifier::from_bytes(b"en").unwrap());
