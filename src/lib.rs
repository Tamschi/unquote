#![doc(html_root_url = "https://docs.rs/unquote/0.0.2")]
#![warn(clippy::pedantic)]

#[cfg(doctest)]
mod readme {
	doc_comment::doctest!("../README.md");
}

// This crate is quite obviously intended as inverse of the `quote` crate.
// However, I frankly don't understand how that one works, so it's proc-macro time.

use proc_macro2::{Delimiter, Literal, Spacing, Span, TokenStream, TokenTree};
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

//TODO: Make errors specific even if the token type is mismatched outright.
fn unquote_inner(input: ParseStream) -> Result<TokenStream> {
	let parse_stream: Expr = input.parse()?;

	let mut output = TokenStream::new();

	let input_ident = Ident::new("input", Span::mixed_site());
	output.extend(quote_spanned! {Span::mixed_site()=>
		let #input_ident = #parse_stream;
	});

	input.parse::<Token![,]>()?;

	while !input.is_empty() {
		let step: TokenStream = match input.parse().unwrap() {
			TokenTree::Group(group) => match group.delimiter() {
				Delimiter::Parenthesis => grammar_todo!(group, "()"),
				Delimiter::Brace => grammar_todo!(group, "{}"),
				Delimiter::Bracket => grammar_todo!(group, "[]"),
				Delimiter::None => Parser::parse2(unquote_inner, group.stream())?,
			},
			TokenTree::Ident(ident) => {
				let message = Literal::string(&format!("Expected `{}`", ident.to_string()));
				hygienic_spanned! {ident.span()=>
					if #input_ident.call(<syn::Ident as syn::ext::IdentExt>::parse_any)?
						!= syn::parse::Parser::parse2(<syn::Ident as syn::ext::IdentExt>::parse_any, quote!(#ident)).unwrap()
					{
						return Err(syn::Error::new(#input_ident.cursor().span(), #message));
					}
				}
			}
			TokenTree::Punct(punct) => match punct.as_char() {
				'#' => {
					#[allow(clippy::map_err_ignore)]
					match input.parse().map_err(|_| {
						Error::new(
							input.span(),
							"Unexpected end of macro input: Expected Parse identifier, Span identifier written as lifetime or joined `#`",
						)
					})? {
						TokenTree::Ident(placeholder) => {
							hygienic_spanned! {punct.span().join(placeholder.span()).unwrap_or_else(|| placeholder.span())=>
								#placeholder = #input_ident.parse()?;
							}
						}
						TokenTree::Punct(number_sign) if punct.spacing() == Spacing::Joint && number_sign.as_char() == '#' => {
							hygienic_spanned! {punct.span().join(number_sign.span()).unwrap_or_else(||number_sign.span())=>
								#input_ident.parse::<syn::Token![#]>()?;
							}
						}
						TokenTree::Punct(apostrophe)
							if punct.spacing() == Spacing::Alone
								&& apostrophe.as_char() == '\'' && apostrophe.spacing()
								== Spacing::Joint =>
						{
							let placeholder: Ident = input.parse()?;
							hygienic_spanned! {punct.span().join(placeholder.span()).unwrap_or_else(||placeholder.span())=>
								#placeholder = #input_ident.span();
							}
						}
						other => {
							return Err(Error::new_spanned(
								other,
								"Expected Parse identifier, Span identifier written as lifetime or joined `#`",
							))
						}
					}
				}
				_char => hygienic_spanned! {punct.span()=>
					//TODO: Spacing
					#input_ident.parse::<syn::Token![#punct]>()?;
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

	Ok(quote_spanned!(Span::mixed_site()=> { #output }))
}
