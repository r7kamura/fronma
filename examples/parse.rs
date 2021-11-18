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
