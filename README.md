# tohaya

Quick-n-dirty converter of PubMed .nbib format to [hayagriva](https://github.com/typst/hayagriva).
Currently a working WIP.

For the most part, `tohaya` is a command-line front-end to [biblib](https://github.com/AliAzlanDev/biblib).

Usage

```shell
tohaya < citation.nbib >> bibliography.yml
```

## Installation

- **Direct download** from https://github.com/FNNDSC/tohaya/releases/latest
- **[Nix](https://nixos.org/) flakes**: figure it out yourself
- **Compile from source**: `cargo install tohaya`
- [**cargo-binstall**](https://github.com/cargo-bins/cargo-binstall): `cargo binstall tohaya`

## Known Issues

Some fields, such as title, are truncated. See https://github.com/AliAzlanDev/biblib/pull/5

## Roadmap

- Installation from PyPi using [maturin](https://github.com/PyO3/maturin)
- Installation using [pixi](https://pixi.sh/)/[mamba](https://mamba.readthedocs.io)/[conda](https://conda.io)
- More flexible CLI usage
- Web front-end using wasm_bindgen + BibTeX support, see https://github.com/JonasLoos/bibtex-to-hayagriva-webapp
- Possible upstream integration with hayagriva, see https://github.com/typst/hayagriva/issues/329

