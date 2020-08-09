# dirty-json-rs

[![GitHub Actions](https://github.com/messense/dirty-json-rs/workflows/CI/badge.svg)](https://github.com/messense/dirty-json-rs/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/dirty-json.svg)](https://crates.io/crates/dirty-json)
[![docs.rs](https://docs.rs/dirty-json/badge.svg)](https://docs.rs/dirty-json)

Fix up dirty JSON string

## Features

`dirty-json` allows numberic object keys.

```rust
fn main() {
    let json = r#"{1: "foo", 2 : "bar"}"#;
    let fixed = fix(json);
    assert_eq!(fixed, r#"{"1":"foo","2":"bar"}"#);
}
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.