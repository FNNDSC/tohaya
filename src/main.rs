use tohaya::{CitationFormat, tohaya};

fn main() -> color_eyre::Result<()> {
    let input = read_in()?;
    let yaml = tohaya(input, CitationFormat::Pubmed)?;
    println!("{yaml}");
    Ok(())
}

fn read_in() -> std::io::Result<String> {
    let stdin = std::io::stdin();
    std::io::read_to_string(stdin)
}
