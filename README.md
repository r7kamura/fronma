# fronma

[![test](https://github.com/r7kamura/fronma/actions/workflows/test.yml/badge.svg)](https://github.com/r7kamura/fronma/actions/workflows/test.yml)

[Front Matter](https://jekyllrb.com/docs/front-matter/) parser for Rust.

## Usage

Add this crate as a dependency:

```toml
[dependencies]
fronma = "~0.2"
```

then use `fronma::parser::parse` to parse text that has YAML Front Matter:

```rust
use fronma::parser::parse;
use serde::Deserialize;

#[derive(Deserialize)]
struct Headers {
  title: String,
}

fn main() {
    let text = r#"---
title: A
---
B
"#;

    let data = parse::<Headers>(text).unwrap();
    assert_eq!(data.headers.title, "A");
    assert_eq!(data.body, "B\n");
}
```

### Other formats

This library supports the following Front Matter formats:

- YAML (supported by `default` feature)
- TOML (requires `toml` feature)
- JSON (requires `json` feature)

For example, when you want to use TOML format, add this crate with toml feature:

```toml
[dependencies]
fronma = { version = "~0.1", features = ["toml"] }
```

then use `fronma::parser::parse_with_engine` like this:

```rust
use fronma::engines::Toml;
use fronma::parser::parse_with_engine;
use serde::Deserialize;

#[derive(Deserialize)]
struct Headers {
  title: String,
}

fn main() {
    let text = r#"---
title = "dummy_title"
---
dummy_body
"#;
    let result = parse_with_engine::<Headers, Toml>(text).unwrap();
    assert_eq!(result.headers.title, "dummy_title");
    assert_eq!(result.body, "dummy_body\n");
}
```
