use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
	parse_macro_input, parse_quote_spanned, spanned::Spanned, visit_mut::VisitMut, Data,
	DeriveInput, Meta,
};

struct FieldVisitor {
	updated: bool,
	mappings: Vec<proc_macro2::TokenStream>,
}

impl FieldVisitor {
	fn new() -> Self {
		Self {
			updated: false,
			mappings: Vec::new(),
		}
	}
}

impl VisitMut for FieldVisitor {
	fn visit_field_mut(&mut self, i: &mut syn::Field) {
		let num_attrs = i.attrs.len();
		i.attrs.retain(|x| {
			!x.path()
				.get_ident()
				.is_some_and(|x| x == "intern")
		});

		let span = i.ty.span();

		let ty = i.ty.clone();

		let name = i
			.ident
			.clone()
			.expect("The derive macro for Intern can not deal with tuple structs yet.");

		// The field contained an #[intern] attribute
		if i.attrs.len() < num_attrs {
			self.updated = true;
			i.ty = parse_quote_spanned!(span => ::treentern::Interned<<#ty as ::treentern::Intern>::InternedType>);

			self.mappings.push(quote! {
				let #name = <#ty as ::treentern::Intern>::intern(#name);
			});
		} else {
			i.ty = parse_quote_spanned! {span => &'static #ty};
		}
	}
}

struct StructVisitor<'a> {
	input: Ident,
	output: Ident,
	unpack: proc_macro2::TokenStream,
	pack: proc_macro2::TokenStream,
	field_visitor: &'a mut FieldVisitor,
}

impl<'a> StructVisitor<'a> {
	fn new(input: Ident, output: Ident, field_visitor: &'a mut FieldVisitor) -> Self {
		Self {
			input,
			output,
			unpack: proc_macro2::TokenStream::new(),
			pack: proc_macro2::TokenStream::new(),
			field_visitor,
		}
	}

	fn get_implementation(&mut self) -> proc_macro2::TokenStream {
		let unpack = &self.unpack;

		let mappings = self
			.field_visitor
			.mappings
			.drain(..);

		let pack = &self.pack;

		quote! {
			#unpack

			#(#mappings)*

			ARENA.insert_owned(#pack)
		}
	}
}

impl<'a> VisitMut for StructVisitor<'a> {
	fn visit_fields_mut(&mut self, i: &mut syn::Fields) {
		let names = i
			.iter()
			.enumerate()
			.map(|(index, x)| {
				x.ident
					.clone()
					.unwrap_or_else(|| Ident::new(&format!("value{}", index), x.span()))
			})
			.collect::<Vec<_>>();

		let input = &self.input;
		let output = &self.output;

		self.field_visitor
			.visit_fields_mut(i);

		self.unpack = quote! {
			let #input { #(#names),* } = self;
		};

		self.pack = quote! {
			#output { #(#names),* }
		};
	}
}

#[proc_macro_derive(Intern, attributes(intern, intern_derive))]
pub fn derive_intern(input: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(input as DeriveInput);

	if let Some(attr) = input
		.attrs
		.iter_mut()
		.find(|x| {
			x.path()
				.get_ident()
				.is_some_and(|x| x == "intern_derive")
		}) {
		if let Ok(mut list) = attr
			.meta
			.require_list()
			.cloned()
		{
			list.path = Ident::new("derive", list.path.span()).into();
			attr.meta = Meta::List(list);
		}
	}

	let name = input.ident.clone();
	let const_name = Ident::new(
		&format!("_INTERN_IMPL_{}", name).to_uppercase(),
		name.span(),
	);

	let mut target_name = Ident::new(&format!("Interned{}", name), name.span());

	let mut field_visitor = FieldVisitor::new();

	let mut implementation = match &mut input.data {
		Data::Struct(i) => {
			let mut visitor =
				StructVisitor::new(name.clone(), target_name.clone(), &mut field_visitor);
			visitor.visit_data_struct_mut(i);
			visitor.get_implementation()
		}
		Data::Enum(_) => quote! { compile_error!("Cannot derive Intern for Enum yet.") },
		Data::Union(_) => quote! { compile_error!("Cannot derive Intern for Union.") },
	};

	if !field_visitor.updated {
		implementation = quote! { ARENA.insert(self) };
		target_name = name.clone();
	}

	input.ident = target_name.clone();

	let target_type = field_visitor
		.updated
		.then_some(input);

	quote! {
		#target_type

		const #const_name: () = {
			use ::std::sync::LazyLock;
			use ::treentern::{arena::Arena, Intern, Interned};
			static ARENA: LazyLock<Arena<#target_name>> = LazyLock::new(Arena::new);
			impl Intern for #name {
				type InternedType = #target_name;

				fn intern(&'static self) -> Interned<Self::InternedType> {
					#implementation
				}
			}
		};
	}
	.into()
}
