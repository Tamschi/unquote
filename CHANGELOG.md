# unquote Changelog

<!-- markdownlint-disable no-trailing-punctuation -->

## 0.0.7

2023-08-26

* **Breaking changes**
  * Removed [`Span`] for the time being.
    > The previous implementation caused more issues than it solved, since it retrieved [`Span`]s of previously parsed tokens.
* Features
  * [`quote`] now doesn't have to be in scope to unquote literal identifiers.
* Fixed
  * Missing `syn/printing` dependency feature.
* Revisions
  * Updated Syn dependency to version `2.0.29`.
  * Fixed a few warnings.

[`Span`]: https://docs.rs/proc-macro2/1/proc_macro2/struct.Span.html
[`quote`]: https://docs.rs/quote/1.0.9/quote/macro.quote.html

## 0.0.6

2020-12-05

* Use `#let ident` to declare `ident`.
* Use `#do parser_function => binding` to parse `binding` via `parser_function`.
* Use `#do let parser_function => binding` to declare `binding` and parse it via `parser_function`.

## 0.0.5

2020-11-14

* Fixed
  * `#$'span` now correctly captures a [`Span`] *up to but not past* this expression. (broken since 0.0.4)
  * Fixed group recursion. (broken since 0.0.1)
* Revisions
  * Revised CHANGELOG

[`Span`]: https://docs.rs/proc-macro2/1/proc_macro2/struct.Span.html

## 0.0.4

2020-11-13

* Features:
  * Implemented [`Span`] range captures (best effort)
* Revisions:
  * Ticked basic [`Span`] captures in README
  * Readded call2-for-syn dependence due better type inference

[`Span`]: https://docs.rs/proc-macro2/1/proc_macro2/struct.Span.html

## 0.0.3

2020-11-10

* Features:
  * Implemented identifiers
  * Implemented basic [`Span`] captures
  * Implemented `#` escape
* Revisions:
  * Better errors
  * Better tests
  * Removed call2-for-syn dependence (except for testing)

[`Span`]: https://docs.rs/proc-macro2/1/proc_macro2/struct.Span.html

## 0.0.2

2020-11-08

* Features:
  * Implemented literals
* Revisions:
  * README and CHANGELOG fixes

## 0.0.1

2020-11-07

Initial unstable release
