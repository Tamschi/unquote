#![doc(html_root_url = "https://docs.rs/unquote/0.0.1")]
#![warn(clippy::pedantic)]

#[cfg(doctest)]
mod readme {
	doc_comment::doctest!("../README.md");
}

// This crate is quite obviously intended as inverse of the `quote` crate.
// However, I frankly don't understand how that one works, so it's proc-macro time.

use call2_for_syn::call2;
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use quote::quote_spanned;
use syn::{parse::ParseStream, Error, Expr, Ident, Result, Token};

#[proc_macro]
pub fn unquote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	call2(input.into(), |input| unquote_inner(input))
		.unwrap_or_else(|error| error.to_compile_error())
		.into()
}

macro_rules! step_at {
	($input_span:expr => $($output:tt)*) => {
		quote_spanned! {$input_span.resolved_at(Span::mixed_site())=> {
			$($output)*
		}}
	};
}

fn unquote_inner(input: ParseStream) -> Result<TokenStream> {
	let parse_stream: Expr = input.parse()?;

	let mut output = TokenStream::new();

	let input_ident = Ident::new("input", Span::mixed_site());
	output.extend(quote_spanned! {Span::mixed_site()=>
		let #input_ident = #parse_stream;
	});

	input.parse::<Token![,]>()?;

	let input_stream = input;
	let mut input = input_stream.cursor();
	while let Some((token, next)) = input.token_tree() {
		input = next;

		let step: TokenStream = match token {
			TokenTree::Group(group) => match group.delimiter() {
				Delimiter::Parenthesis => todo!("parenthesis"),
				Delimiter::Brace => todo!("brace"),
				Delimiter::Bracket => todo!("bracket"),
				Delimiter::None => call2(group.stream(), unquote_inner)?,
			},
			TokenTree::Ident(_ident) => todo!("ident"),
			TokenTree::Punct(punct) => match punct.as_char() {
				'#' => {
					let (placeholder, next) = input
						.ident()
						.ok_or_else(|| Error::new(input.span(), "Expected identifier"))?;
					input = next;
					step_at! {punct.span().join(placeholder.span()).unwrap_or_else(|| placeholder.span())=>
						#placeholder = #input_ident.parse()?;
					}
				}
				char => step_at! {punct.span()=>
					//TODO?: Spacing
					let punct: syn::Token![#punct] = #input_ident
						.parse()
						.map_err(|error| syn::Error::new(error.span(), format_args!("Expected `{}`", #char)))?;
				},
			},
			TokenTree::Literal(_) => todo!("literal"),
		};
		output.extend(step);
	}

	// Catch up input.
	//TODO: This should be handled more nicely...
	input_stream.parse::<TokenStream>().unwrap();

	Ok(quote_spanned!(Span::mixed_site()=> { #output }))
}

fn _scratchpad(input: ParseStream) -> Result<()> {
	input.parse::<Token![#]>()?;

	let punct: proc_macro2::Punct = input
		.parse()
		.map_err(|error| syn::Error::new(error.span(), format_args!("Expected `{}`", '<')))?;
	if punct.as_char() != '<' {
		return Err(syn::Error::new(
			punct.span(),
			format_args!("Expected `{}`", '<'),
		));
	}
	Ok(())
}
