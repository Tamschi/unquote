#![doc(html_root_url = "https://docs.rs/unquote/0.0.2")]
#![warn(clippy::pedantic)]

#[cfg(doctest)]
mod readme {
	doc_comment::doctest!("../README.md");
}

// This crate is quite obviously intended as inverse of the `quote` crate.
// However, I frankly don't understand how that one works, so it's proc-macro time.

use proc_macro2::{Delimiter, Literal, Span, TokenStream, TokenTree};
use quote::quote_spanned;
use syn::{
	parse::{ParseStream, Parser},
	Error, Expr, Ident, Result, Token,
};

#[proc_macro]
pub fn unquote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	Parser::parse2(unquote_inner, input.into())
		.unwrap_or_else(|error| error.to_compile_error())
		.into()
}

macro_rules! hygienic_spanned {
	($input_span:expr => $($output:tt)*) => {
		quote_spanned!($input_span.resolved_at(Span::mixed_site())=> $($output)*)
	};
}

macro_rules! grammar_todo {
	($token:ident) => {
		grammar_todo!($token, stringify!($token))
	};
	($token:ident, $name:expr) => {
		return Err(Error::new_spanned(
			$token,
			format_args!("Not yet implemented: {}", $name),
			));
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
				Delimiter::Parenthesis => grammar_todo!(group, "()"),
				Delimiter::Brace => grammar_todo!(group, "{}"),
				Delimiter::Bracket => grammar_todo!(group, "[]"),
				Delimiter::None => Parser::parse2(unquote_inner, group.stream())?,
			},
			TokenTree::Ident(ident) => grammar_todo!(ident),
			TokenTree::Punct(punct) => match punct.as_char() {
				'#' => {
					let (placeholder, next) = input
						.ident()
						.ok_or_else(|| Error::new(input.span(), "Expected identifier"))?;
					input = next;
					hygienic_spanned! {punct.span().join(placeholder.span()).unwrap_or_else(|| placeholder.span())=>
						#placeholder = #input_ident.parse()?;
					}
				}
				_char => hygienic_spanned! {punct.span()=>
					//TODO?: Spacing
					let punct: syn::Token![#punct] = #input_ident.parse()?;
				},
			},
			TokenTree::Literal(literal) => {
				let message = Literal::string(&format!("Expected `{}`", literal.to_string()));
				hygienic_spanned! {literal.span()=>
					if #input_ident.parse::<syn::Lit>()? != syn::parse2(quote!(#literal)).unwrap() {
						return Err(syn::Error::new(#input_ident.cursor().span(), #message));
					}
				}
			}
		};
		output.extend(step);
	}

	// Catch up input.
	//TODO: This should be handled more nicely...
	input_stream.parse::<TokenStream>().unwrap();

	Ok(quote_spanned!(Span::mixed_site()=> { #output }))
}
