# unquote

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/unquote)
[![Crates.io](https://img.shields.io/crates/v/unquote)](https://crates.io/crates/unquote)
[![Docs.rs](https://docs.rs/unquote/badge.svg)](https://docs.rs/crates/unquote)

![Rust 1.45.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.45.0&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/unquote.svg?branch=develop)](https://travis-ci.com/Tamschi/unquote/branches)
![Crates.io - License](https://img.shields.io/crates/l/unquote/0.0.4)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/unquote)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/unquote)](https://github.com/Tamschi/unquote/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/unquote)](https://github.com/Tamschi/unquote/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/unquote.svg)](https://web.crev.dev/rust-reviews/crate/unquote/)

A reverse quote macro... that is: A macro to parse input from a [`ParseStream`] according to a given pattern.

[`ParseStream`]: https://docs.rs/syn/1/syn/parse/type.ParseStream.html

> Note: This library is a work in progress. While I don't expect large breaking changes to the syntax, there are missing features and error messages aren't always great yet.
>
> This macro currently requires `syn` to be available in the current namespace with at least the `"extra-traits"` and `"parsing"` features enabled. This should get fixed with the next larger refactor.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add unquote
```

## Example

```rust
use call2_for_syn::call2_strict;
use quote::quote;
use std::error::Error;
use syn::{LitStr, parse::ParseStream};
use unquote::unquote;

fn main() -> Result<(), Box<dyn Error>> {
  // Sample input
  let tokens = quote!(<!-- "Hello!" -->);

  // Analogous to a parser implementation with `syn`:
  fn parser_function(input: ParseStream) -> syn::Result<LitStr> {
    // Declare bindings ahead of time.
    // Rust can usually infer the type.
    let parsed;

    // This uses the ? operator internally -
    // It needs to be inside a `Result`-returning function.
    unquote!(input, <!-- #parsed -->);

    Ok(parsed)
  }

  let parsed = call2_strict(tokens, parser_function)??;
  assert_eq!(parsed.value(), "Hello!");
  Ok(())
}
```

## Implementation Status

(roughly in order of priority)

| Tokens |  |
|-|-|
| Punct | 🗸³ |
| Ident | ✔ |
| Literal | ✔ |

| Bindings |  |
|-|-|
| `#binding` | ✔ |
| `##`-escapes | ✔ |

| Groups |  |
|-|-|
| `()` |  |
| `{}` |  |
| `[]` |  |

| Variadics¹ |  |
|-|-|
| `#(#binding)?` |  |
| `#(#binding)*` |  |
| `#(#binding),*` |  |
| `#(#binding)+`² |  |
| `#(#binding),+`² |  |

| Span Captures |  |
|-|-|
| `#'span`⁴ | ✔ |
| `#^'span`⁴ | ✔ |
| `#$'span`⁴ | ✔ |

| Positional Bindings...?⁵ |  |
|-|-|
| `#0` |  |

| Utility Macros |  |
|-|-|
| `Unquotable`-derive⁶ |  |

| Helpers |  |
|-|-|
| `Keyword` |  |
| `AnyIdent` |  |

¹  Note that all variadics are eager beyond the first [`TokenTree`] and only do very shallow speculative parsing! In practice, this means that for example parsing `++` as `#(+-)?++` will fail, as the first `+` "locks in" the optional phrase.

[`TokenTree`]: https://docs.rs/proc-macro2/1/proc_macro2/enum.TokenTree.html

² Not specifically present in [`quote`], but [required variadics are great.]

[`quote`]: https://github.com/dtolnay/quote#rust-quasi-quoting
[required variadics are great.]: https://blog.berkin.me/variadics-in-rant-4-and-why-i-think-theyre-better-ckgmrxa2200t9o9s10v7o0dh2

³ Currently without distinction regarding combinations like `=>` vs. `= >` and such. This *will* change eventually, along with a breaking semver change.

⁴ Denoting [`Span`]s as lifetimes is a bit unusual, but nicely highlights them with a different colour.

[`Span`]: https://docs.rs/proc-macro2/1.0.24/proc_macro2/struct.Span.html

⁵ This would come in handy when using the macro for example in `if let` conditions (since the positional bindings would be returned only by value in the macro expression's result), and wouldn't interfere with named bindings. It's definitely more of a bonus feature though, in case it can indeed be added cleanly.

⁶ This should work on `struct`s and implement `syn::Parse` and a custom trait that checks the first token.

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
