use call2_for_syn::call2;
use quote::quote;
use syn::{LitStr, Result};
use unquote::unquote;

#[test]
fn html_comment() -> Result<()> {
	let tokens = quote!(<!-- "Hello!" -->);

	call2(tokens, |input| {
		let reparsed: LitStr;
		unquote!(input, <!-- #reparsed -->);
		assert_eq!(reparsed.value(), "Hello!");
		Result::Ok(())
	})?;

	Ok(())
}
