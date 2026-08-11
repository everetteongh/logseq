# logseq

Simple library to read/write to Logseq note graphs in Rust, built with [comrak](https://lib.rs/crates/comrak).

## Usage

Read the [docs](https://everette.codeberg.page/logseq/latest/logseq/) online, or see the [tests](./tests/) for example code.

## Crate Features

- `regex-lite`: use the [regex-lite](https://lib.rs/crates/regex-lite) crate as the regular expression backend (**enabled by default**)
- `regex`: use the [regex](https://lib.rs/crates/regex) crate as the regular expression backend
- `serde`: enable support for [serde](https://serde.rs) serialization & deserialization of certain types

## Support

Currently, the latest version of [Logseq OG](https://github.com/Logseq/OG) (file-based) is supported, with some caveats:

- "Timetracking" should be **disabled**
- "Preferred workflow" should be **TODO/DOING**
- Graph reindexing is currently TODO, so journal entry files created through the library aren't yet visible in Logseq ([tracking issue](https://codeberg.org/everette/logseq/issues/4))

Support for the DB version of Logseq is also TODO ([tracking issue](https://codeberg.org/everette/logseq/issues/1))

## Contributing

Contributions are hugely appreciated! The repository on [Codeberg](https://codeberg.org/everette/logseq) is ideal, but the [GitHub mirror](https://github.com/everetteongh/logseq) works too :>
