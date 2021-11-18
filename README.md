# fronma

[![test](https://github.com/r7kamura/fronma/actions/workflows/test.yml/badge.svg)](https://github.com/r7kamura/fronma/actions/workflows/test.yml)

[Front Matter](https://jekyllrb.com/docs/front-matter/) parser for Rust.

## Usage

```rust
use fronma::parse;
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
