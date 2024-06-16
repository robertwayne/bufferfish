extern crate proc_macro;

use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Index, Type, TypePath};

#[proc_macro_derive(Encode, attributes(bufferfish))]
#[proc_macro_error]
pub fn bufferfish_impl_encodable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut packet_id = None;

    for attr in &ast.attrs {
        if attr.path().is_ident("bufferfish") {
            if let Ok(expr) = attr.parse_args::<syn::Expr>() {
                packet_id = Some(expr);
            } else {
                abort!(attr.span(), "expected a single expression");
            }
        }
    }

    let packet_id_snippet = if let Some(packet_id) = packet_id {
        quote! { bf.write_u16(u16::from(#packet_id))?; }
    } else {
        quote! {}
    };

    let mut encoded_snippets = Vec::new();

    match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let Some(ident) = field.ident.as_ref() else {
                        abort!(field.span(), "named fields are required");
                    };

                    encode_type(quote! { self.#ident }, &field.ty, &mut encoded_snippets)
                }
            }
            Fields::Unnamed(fields) => {
                for (i, field) in fields.unnamed.iter().enumerate() {
                    let index = Index::from(i);
                    encode_type(quote! { self.#index }, &field.ty, &mut encoded_snippets)
                }
            }
            Fields::Unit => {}
        },
        _ => abort!(ast.span(), "only structs are supported"),
    };

    let gen = quote! {
        impl bufferfish::Encodable for #name {
            fn encode(&self, bf: &mut bufferfish::Bufferfish) -> Result<(), bufferfish::BufferfishError> {
                #(#encoded_snippets)*
                Ok(())
            }

            fn to_bufferfish(&self) -> Result<bufferfish::Bufferfish, bufferfish::BufferfishError> {
                let mut bf = bufferfish::Bufferfish::new();
                #packet_id_snippet
                self.encode(&mut bf)?;

                Ok(bf)
            }
        }
    };

    gen.into()
}

fn encode_type(accessor: TokenStream, ty: &Type, dest: &mut Vec<TokenStream>) {
    match ty {
        // Handle primitive types
        Type::Path(TypePath { path, .. })
            if path.is_ident("u8")
                || path.is_ident("u16")
                || path.is_ident("u32")
                || path.is_ident("i8")
                || path.is_ident("i16")
                || path.is_ident("i32")
                || path.is_ident("bool")
                || path.is_ident("String") =>
        {
            dest.push(quote! {
                bufferfish::Encodable::encode(&#accessor, bf)?;
            });
        }
        // Handle arrays where elements impl Encodable
        Type::Path(TypePath { path, .. })
            if path.segments.len() == 1 && path.segments[0].ident == "Vec" =>
        {
            dest.push(quote! {
                bf.write_array(&#accessor)?;
            });
        }
        // Handle nested structs where fields impl Encodable
        Type::Path(TypePath { .. }) => {
            dest.push(quote! {
                bufferfish::Encodable::encode(&#accessor, bf)?;
            });
        }
        _ => abort!(ty.span(), "type cannot be encoded into a bufferfish"),
    }
}
