# alleged-lib

Simple library to read/write to Logseq note graphs in Rust, built with [comrak](https://lib.rs/crates/comrak).

## Logseq Support

Currently, the latest version of [Logseq OG](https://github.com/Logseq/OG) (read: file-based) is supported, with some caveats:

- "Timetracking" should be **disabled**
- "Preferred workflow" should be **TODO/DOING**
- Graph reindexing is currently TODO, so newly-created journal entry files aren't yet visible in Logseq ([tracking issue](https://codeberg.org/0xstel/alleged/issues/4))

Logseq DB version support is TODO -- see [the tracking issue](https://codeberg.org/0xstel/alleged/issues/1)

## Crate Features

- `regex-lite`: use the [regex-lite](https://lib.rs/crates/regex-lite) crate as the regular expression backend (**enabled by default**)
- `regex`: use the [regex](https://lib.rs/crates/regex) crate as the regular expression backend
- `serde`: enable support for [serde](https://serde.rs) serialization & deserialization of certain types

## Contributing

Contributions -- code or issues -- are welcome! The repository on [Codeberg](https://codeberg.org/0xstel/alleged) is ideal, but contributions are also accepted on the [GitHub mirror](https://github.com/0xstel-contrib/alleged) :>
