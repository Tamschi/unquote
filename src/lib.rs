#![doc(html_root_url = "https://docs.rs/unquote/0.0.7")]
#![warn(clippy::pedantic)]

#[cfg(doctest)]
mod readme {
	doc_comment::doctest!("../README.md");
}

// This crate is quite obviously intended as inverse of the `quote` crate.
// However, I frankly don't understand how that one works, so it's proc-macro time.

use call2_for_syn::call2_strict;
use proc_macro2::{Delimiter, Literal, Spacing, Span, TokenStream, TokenTree};
use quote::quote_spanned;
use std::collections::HashSet;
use syn::{
	parse::{ParseStream, Parser},
	spanned::Spanned,
	Error, Expr, Ident, Result, Token,
};

#[proc_macro]
pub fn unquote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	Parser::parse2(unquote_outer, input.into())
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
		))
	};
}

fn unquote_outer(input: ParseStream) -> Result<TokenStream> {
	let parse_stream: Expr = input.parse()?;

	input.parse::<Token![,]>()?;

	let input_ident = Ident::new("input", Span::mixed_site());

	let mut declare_up_front = HashSet::new();
	let output = unquote_inner(input, &input_ident, &mut declare_up_front)?;
	let declare_up_front = declare_up_front.into_iter();
	Ok(quote_spanned!(Span::mixed_site()=>
		let #input_ident = #parse_stream;
		#(let #declare_up_front;)*
		#output
	))
}

//TODO: Make errors specific even if the token type is mismatched outright.
#[allow(clippy::too_many_lines)]
fn unquote_inner(
	input: ParseStream,
	input_ident: &Ident,
	declare_up_front: &mut HashSet<Ident>,
) -> Result<TokenStream> {
	let mut output = TokenStream::new();

	while !input.is_empty() {
		let step: TokenStream = match input.parse().unwrap() {
			TokenTree::Group(group) => match group.delimiter() {
				Delimiter::Parenthesis => grammar_todo!(group, "()"),
				Delimiter::Brace => grammar_todo!(group, "{}"),
				Delimiter::Bracket => grammar_todo!(group, "[]"),
				Delimiter::None =>
					call2_strict(group.stream(), |input| unquote_inner(input,input_ident, declare_up_front))
					.unwrap_or_else(|_| Err(Error::new(group.span_close(), "Unexpected end of undelimited group")))?,
			},
			TokenTree::Ident(ident) => {
				let message = Literal::string(&format!("Expected `{ident}`"));
				hygienic_spanned! {ident.span()=>
					if #input_ident.call(<syn::Ident as syn::ext::IdentExt>::parse_any)? != stringify!(#ident) {
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
							"Unexpected end of macro input: Expected Parse identifier, Span identifier written as lifetime, joined `#` or `let(pattern)`",
						)
					})? {
						TokenTree::Ident(r#do) if r#do == "do" => {
							let r#let: Option<Token![let]> = input.parse()?;
							let parser_function: Expr = input.parse()?;
							let fat_arrow: Token![=>] = input.parse()?;
							let placeholder: Ident = input.parse()?;
							if r#let.is_some() {
								declare_up_front.insert(placeholder.clone());
							}
							hygienic_spanned! {
								punct.span()
								.join(r#do.span())
								.and_then(|s| s.join(fat_arrow.span()))
								.and_then(|s| s.join(placeholder.span()))
								.unwrap_or_else(|| r#do.span())
								=>
								#placeholder = #input_ident.call(#parser_function)?;
							}
						}
						TokenTree::Ident(r#let) if r#let == "let" => {
							let placeholder: Ident = input.parse()?;
							declare_up_front.insert(placeholder.clone());
							hygienic_spanned! {punct.span().join(r#let.span()).and_then(|s| s.join(placeholder.span())).unwrap_or_else(|| r#let.span())=>
								#placeholder = #input_ident.parse()?;
							}
						}
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
				let message = Literal::string(&format!("Expected `{literal}`"));
				hygienic_spanned! {literal.span()=>
					let parsed = #input_ident.parse::<syn::Lit>()?;
					if parsed != syn::parse2(quote!(#literal)).unwrap() {
						return Err(syn::Error::new(#input_ident.cursor().span(), #message));
					}
				}
			}
		};
		output.extend(step);
	}

	Ok(output)
}
