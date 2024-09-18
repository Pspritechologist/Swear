#![feature(let_chains)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
// use regex::Regex;
use syn::{parse_macro_input, Expr, ItemStruct, Lit, Meta, MetaNameValue, ExprLit};

#[proc_macro_attribute]
pub fn swear_object(_: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as ItemStruct);

	let name = input.ident.clone();

	// The portion of functions used to redirect Object conversion.
	let convert_fns = quote! {
		#[inline(always)]
		fn to_chars(&self) -> Chars {
			self.to_swear_chars()
		}
		#[inline(always)]
		fn to_count(&self) -> Count {
			self.to_swear_count()
		}
		#[inline(always)]
		fn to_state(&self) -> State {
			self.to_swear_state()
		}
		#[inline(always)]
		fn to_deck(&self) -> Deck {
			self.to_swear_deck()
		}
		#[inline(always)]
		fn to_map(&self) -> Map {
			self.to_swear_map()
		}

		#[inline(always)]
		fn get_functions(&self) -> HashMap<String, FunctionInfo> {
			self.get_functions()
		}
	};

	// The get_info function.
	let desc: String = input.attrs.iter().filter_map(|attr| {
		match attr {
			syn::Attribute { meta: Meta::NameValue(MetaNameValue { path, value: Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }), .. }), .. } if path.is_ident("doc") =>
				Some(lit.value().trim().to_string()),
			_ => None
		}
	})
		.collect::<Vec<_>>()
		.join("\n");

	let get_info_fn = quote! {
		fn get_info(&self) -> ObjectInfo {
			ObjectInfo::from_str(stringify!(#name))
				.with_description_str(#desc)
		}
	};

	let output = quote! {
		#input

		impl IObject for #name {
			#convert_fns
			#get_info_fn
		}
	};

    output.into()
}

// fn new_attr(name: &str, value: &str) -> Attribute {
//     Attribute {
//         bracket_token: Default::default(),
//         pound_token: Default::default(),
//         style: syn::AttrStyle::Outer,
//         meta: MetaNameValue {
//             eq_token: Default::default(),
//             path: parse_str(name).unwrap(),
//             value: Expr::Lit(ExprLit {
//                 attrs: Default::default(),
//                 lit: Lit::Str(LitStr::new(value, Span::call_site())),
//             }),
//         }.into(),
//     }
// }

// #[derive(serde::Deserialize, Debug, Clone)]
// struct ParsedDocs {
//     pub stacky_docs: StackyDocs,
// }

// #[derive(serde::Deserialize, Debug, Clone)]
// struct StackyDocs {
//     pub name: Option<String>,
//     pub desc: Option<String>,
//     #[serde(default)]
//     pub args: Vec<(String, String, String)>,
//     pub example: Option<String>,
// }
