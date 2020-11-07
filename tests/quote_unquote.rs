use call2_for_syn::call2;
use quote::quote;
use syn::{LitStr, Result};
use unquote::unquote;

#[test]
fn html_comment() -> Result<()> {
	let tokens = quote!(<!-- "Hello!" -->);

	let mut reparsed: Option<LitStr> = None;

	call2(tokens, |input| unquote!(input, <!-- #reparsed -->))?;

	assert_eq!(reparsed.unwrap().value(), "Hello!");

	Ok(())
}
