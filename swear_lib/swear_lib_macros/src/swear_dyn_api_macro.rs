use super::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, parse_quote, punctuated::Punctuated, FnArg, Ident, ImplItem, PatType, Type};

pub fn swear_dyn_api(mut input: ItemImpl) -> TokenStream {
	let name = input.self_ty.clone();

	input.items = input.items.iter().cloned().map(|item| {
		let ImplItem::Fn(mut func) = item else {
			return item;
		};

		let inputs = &func.sig.inputs.clone();
		// func.sig.variadic
		func.sig.inputs.clear();
		if let Some(FnArg::Typed(PatType { ty, pat, .. })) = inputs.first() && ty.into_token_stream().to_string() == "Self" {
			func.sig.inputs = parse_quote!(#pat: ObjectRef);
		} else {
			func.sig.inputs = parse_quote!(_: ObjectRef);
		}

		func.into()
	}).collect();

	let output = quote! {
		#input
	};

	output
}
