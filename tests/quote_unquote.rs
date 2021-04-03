use call2_for_syn::{call2_allow_incomplete, call2_strict};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::ParseStream, parse2, Attribute, Ident, Lit, LitStr, Result, Token};
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
fn literal_mismatch() {
	let tokens = quote! (1 2.0 "drei" 4_i32 5_usize);

	call2_allow_incomplete(tokens, |input| {
		unquote!(input, 2);
		Result::Ok(())
	})
	.unwrap_err();
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
fn ident_mismatch() {
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
}

#[test]
fn span_snapshot() -> Result<()> {
	let tokens = quote!();

	let (_, _): (Span, _) = call2_strict(tokens, |input| {
		let span_1;
		let span_2;
		unquote!(input, #'span_1 #'span_2);
		Result::Ok((span_1, span_2))
	})
	.unwrap()?;

	Ok(())
}

#[test]
fn number_sign_escape() -> Result<()> {
	let tokens = quote!(#);

	call2_strict(tokens, |input| {
		unquote!(input, ##);
		Result::Ok(())
	})
	.unwrap()
}

#[derive(Debug)]
struct Attributes(Vec<Attribute>);
impl Attributes {
	fn parse_outer(input: ParseStream) -> Result<Self> {
		Attribute::parse_outer(input).map(Self)
	}
}
impl ToTokens for Attributes {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		for attr in self.0.iter() {
			attr.to_tokens(tokens)
		}
	}
}

#[test]
fn r#do() -> Result<()> {
	let tokens = quote! {
		#[some_attribute]
		#[another_attribute]
	};

	let attrs = call2_strict(tokens, |input| {
		let attr;
		unquote!(input, #do Attributes::parse_outer => attr);
		Result::Ok(attr)
	})
	.unwrap()?;

	assert_eq!(attrs.0.len(), 2);

	Ok(())
}

#[test]
fn r#let() -> Result<()> {
	let tokens = quote!(.);

	let _: Token![.] = call2_strict(tokens, |input| {
		unquote!(input, #let dot);
		Result::Ok(dot)
	})
	.unwrap()?;

	Ok(())
}

#[test]
fn do_let() -> Result<()> {
	let tokens = quote! {
		#[some_attribute]
		#[another_attribute]
	};

	let attrs = call2_strict(tokens, |input| {
		unquote!(input, #do let Attributes::parse_outer => attr);
		Result::Ok(attr)
	})
	.unwrap()?;

	assert_eq!(attrs.0.len(), 2);

	Ok(())
}
