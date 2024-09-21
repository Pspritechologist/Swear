use proc_macro2::TokenStream;

use super::*;

pub fn swear_object(input: ItemStruct) -> TokenStream {
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

	output
}
