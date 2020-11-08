use call2_for_syn::call2;
use quote::quote;
use syn::{parse2, Lit, LitStr, Result};
use unquote::unquote;

//FIXME: These tests should also evaluate failures, but `call2` currently panics if not all input was parsed.
// This should be fixed in the next version of call2-for-syn...

#[test]
fn html_comment() -> Result<()> {
	let tokens = quote!(<!-- "Hello!" -->);

	call2(tokens, |input| {
		let reparsed: LitStr;
		unquote!(input, <!-- #reparsed -->);
		assert_eq!(reparsed.value(), "Hello!");
		Result::Ok(())
	})
}

#[test]
fn multipunct() -> Result<()> {
	let tokens = quote!(=>);

	call2(tokens, |input| {
		unquote!(input, =>);
		Result::Ok(())
	})
}

#[test]
fn literals() -> Result<()> {
	let tokens = quote! (1 2.0 "drei" 4_i32 5_usize);

	call2(tokens, |input| {
		let five: Lit;
		unquote!(input, 1 2.0 "drei" 4_i32 #five);
		assert_eq!(five, Lit::Int(parse2(quote!(5_usize))?));
		Result::Ok(())
	})
}
