#![feature(let_chains)]

mod swear_object_macro;
mod swear_dyn_api_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, ItemImpl, ItemStruct, Lit, Meta, MetaNameValue};

#[proc_macro_attribute]
pub fn swear_object(_: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as ItemStruct);
    swear_object_macro::swear_object(input).into()
}

#[proc_macro_attribute]
pub fn sewar_dyn_api(_: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as ItemImpl);
	swear_dyn_api_macro::swear_dyn_api(input).into()
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
