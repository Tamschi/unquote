# unquote

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/unquote)
[![Crates.io](https://img.shields.io/crates/v/unquote)](https://crates.io/crates/unquote)
[![Docs.rs](https://docs.rs/unquote/badge.svg)](https://docs.rs/crates/unquote)

![Rust 1.40.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.40.0&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/unquote.svg?branch=develop)](https://travis-ci.com/Tamschi/unquote/branches)
![Crates.io - License](https://img.shields.io/crates/l/unquote/0.0.1)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/unquote)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/unquote)](https://github.com/Tamschi/unquote/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/unquote)](https://github.com/Tamschi/unquote/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/unquote.svg)](https://web.crev.dev/rust-reviews/crate/unquote/)

Reverse quote macros... that is: Macros to parse input from a ParseStream according to a given pattern.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add unquote
```

## Example

```rust
use call2_for_syn::call2;
use quote::quote;
use syn::{LitStr, parse::ParseStream, Result};
use unquote::unquote;

# fn main() -> Result<()> {
// Sample input
let tokens = quote!(<!-- "Hello!" -->);

// Analogous to a parser implementation with `syn`:
fn parser_function(input: ParseStream) -> Result<LitStr> {
  // Declare bindings ahead of time.
  // Rust can usually infer the type.
  let parsed;

  // This uses the ? operator internally -
  // It needs to be inside a `Result`-returning function.
  unquote!(input, <!-- #parsed -->);

  Ok(parsed)
}

let parsed = call2(tokens, parser_function)?;
assert_eq!(parsed.value(), "Hello!");
# Ok(())
# }
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`unquote` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
