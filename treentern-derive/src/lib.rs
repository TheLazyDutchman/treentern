use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Intern)]
pub fn derive_intern(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;
	let const_name = Ident::new(
		&format!("_INTERN_IMPL_{}", name).to_uppercase(),
		name.span(),
	);

	quote! {
		const #const_name: () = {
			use ::std::sync::LazyLock;
			use ::treentern::{arena::Arena, Intern, Interned};
			static ARENA: LazyLock<Arena<#name>> = LazyLock::new(Arena::new);
			impl Intern for #name {
				type InternedType = Self;

				fn intern(&'static self) -> Interned<Self> {
					ARENA.insert(self)
				}
			}
		};
	}
	.into()
}
