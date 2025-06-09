use biblib::CitationParser;
use hayagriva::types::{
    Date, EntryType, FormatString, MaybeTyped, Person, Publisher, QualifiedUrl,
};
use std::sync::LazyLock;
use unic_langid_impl::LanguageIdentifier;

fn main() -> color_eyre::Result<()> {
    let input = read_in()?;
    let parser = biblib::PubMedParser::new();
    let citations = parser.parse(&input)?;
    let entries = citations.into_iter().map(to_entry);
    let library = hayagriva::Library::from_iter(entries);
    let yaml = hayagriva::io::to_yaml_str(&library)?;
    println!("{yaml}");
    Ok(())
}

fn read_in() -> std::io::Result<String> {
    let stdin = std::io::stdin();
    std::io::read_to_string(stdin)
}

fn to_entry(citation: biblib::Citation) -> hayagriva::Entry {
    let mut errors = Vec::with_capacity(10);
    let key = key_of(&citation);
    let entry_type = entry_type_of(&citation);
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

    if let Some(year) = citation.year {
        entry.set_date(Date::from_year(year));
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
    entry
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
        let name = author.family_name.to_lowercase();
        if let Some(year) = citation.year {
            format!("{name}{year}")
        } else {
            name
        }
    } else {
        "key".to_string()
    }
}

fn entry_type_of(citation: &biblib::Citation) -> EntryType {
    if citation
        .citation_type
        .iter()
        .any(|s| s == "Journal Article")
    {
        EntryType::Article
    } else {
        unimplemented!(
            "Unsupported citation types: {}",
            citation.citation_type.join(", ")
        )
    }
}

static ENGLISH: LazyLock<LanguageIdentifier> =
    LazyLock::new(|| LanguageIdentifier::from_bytes(b"en").unwrap());
