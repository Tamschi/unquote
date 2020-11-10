use call2_for_syn::call2_allow_incomplete;
use quote::quote;
use syn::{parse2, Ident, Lit, LitStr, Result};
use unquote::unquote;

//FIXME: These tests should also evaluate failures, but `call2` currently panics if not all input was parsed.
// This should be fixed in the next version of call2-for-syn...

#[test]
fn html_comment() -> Result<()> {
	let tokens = quote!(<!-- "Hello!" -->);

	call2_allow_incomplete(tokens, |input| {
		let reparsed: LitStr;
		unquote!(input, <!-- #reparsed -->);
		assert_eq!(reparsed.value(), "Hello!");
		Result::Ok(())
	})
}

#[test]
fn multipunct() -> Result<()> {
	let tokens = quote!(=>);

	call2_allow_incomplete(tokens, |input| {
		unquote!(input, =>);
		Result::Ok(())
	})
}

#[test]
fn literals() -> Result<()> {
	let tokens = quote! (1 2.0 "drei" 4_i32 5_usize);

	call2_allow_incomplete(tokens, |input| {
		let five: Lit;
		unquote!(input, 1 2.0 "drei" 4_i32 #five);
		assert_eq!(five, Lit::Int(parse2(quote!(5_usize))?));
		Result::Ok(())
	})
}

#[test]
fn literal_mismatch() -> Result<()> {
	let tokens = quote! (1 2.0 "drei" 4_i32 5_usize);

	call2_allow_incomplete(tokens, |input| {
		unquote!(input, 2);
		Result::Ok(())
	})
	.unwrap_err();

	Ok(())
}

#[test]
fn idents() -> Result<()> {
	let tokens = quote! (static for okay);

	call2_allow_incomplete(tokens, |input| {
		let okay: Ident;
		unquote!(input, static for #okay);
		assert_eq!(okay, parse2::<Ident>(quote!(okay))?);
		Result::Ok(())
	})
}

#[test]
fn ident_mismatch() -> Result<()> {
	let tokens = quote! (static for okay);

	call2_allow_incomplete(tokens.clone(), |input| {
		unquote!(input, for static okay);
		Result::Ok(())
	})
	.unwrap_err();

	call2_allow_incomplete(tokens, |input| {
		unquote!(input, static for mismatched);
		Result::Ok(())
	})
	.unwrap_err();

	Ok(())
}
