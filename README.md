# tohaya

Convert citation files to [hayagriva](https://github.com/typst/hayagriva) YAML.

Supported file formats:

- [Pubmed](https://pubmed.ncbi.nlm.nih.gov/help/#pubmed-format) (`*.nbib`)
- BibLaTeX

## Usage

`tohaya` can be used as a web application at https://fnndsc.github.io/tohaya,
or as a command-line program.

### Examples

```shell
# Convert file and create output file
tohaya citation.nbib --output bibliography.yml

# Alternatively, using pipes
cat citation.nbib | tohaya - >> bibliography.yml
```

## Installation

- **Direct download** from https://github.com/FNNDSC/tohaya/releases/latest
- **[Nix](https://nixos.org/) flakes**: figure it out yourself
- **Compile from source**: `cargo install tohaya`
- [**cargo-binstall**](https://github.com/cargo-bins/cargo-binstall): `cargo binstall tohaya`

## Roadmap

- [ ] Installation from PyPi using [maturin](https://github.com/PyO3/maturin)
- [ ] Installation using [pixi](https://pixi.sh/)/[mamba](https://mamba.readthedocs.io)/[conda](https://conda.io)
- [ ] Possible upstream integration with hayagriva, see https://github.com/typst/hayagriva/issues/329
