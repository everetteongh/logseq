# logseq

Simple library to read/write to Logseq note graphs in Rust, built with [comrak](https://lib.rs/crates/comrak).

## Usage

First, add the crate to your `Cargo.toml`:

```toml
logseq = { git = "https://codeberg.org/everette/logseq", tag = "v0.1.2" }
```

Then, use it in your code:

```rust
use logseq::prelude::*;

fn main() {
    let graph = Graph::builder().dir("/path/to/your/graph/".into()).build().unwrap();
    let today = graph.today().unwrap();
    
    let blocks: Vec<&Block> = today.blocks().collect();
    let content = today.document.to_string();

    println!("The file content for today's journal entry:\n---\n{}\n---", content.trim());

    if !blocks.is_empty() {
        println!("Blocks:");
    }

    for block in blocks {
        println!("---\n* ID: {}\n* Content (plain): {}\n---", block.properties.id, block.plain());
    }
}
```

See the [tests](./tests/) for more examples, or read the [docs](https://everette.codeberg.page/logseq/latest/logseq/) online.

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
