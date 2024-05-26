extern crate proc_macro;

use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Index, Type, TypePath};

#[proc_macro_derive(Encode)]
#[proc_macro_error]
pub fn bufferfish_impl_encodable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut serialized_snippets = Vec::new();

    match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let Some(ident) = field.ident.as_ref() else {
                        abort!(field.span(), "named fields are required");
                    };

                    serialize_type(quote! { #ident }, &field.ty, &mut serialized_snippets)
                }
            }
            Fields::Unnamed(fields) => {
                for (i, field) in fields.unnamed.iter().enumerate() {
                    let index = Index::from(i);
                    serialize_type(quote! { #index }, &field.ty, &mut serialized_snippets)
                }
            }
            Fields::Unit => {}
        },
        _ => abort!(ast.span(), "only structs are supported"),
    };

    let gen = quote! {
        impl bufferfish::encodable::Encodable for #name {
            fn encode(&self, bf: &mut bufferfish::Bufferfish) -> std::io::Result<()> {
                #(#serialized_snippets)*

                Ok(())
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(Serialize, attributes(bufferfish))]
#[proc_macro_error]
pub fn bufferfish_serializer(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let packet_id_serialization = if let Some(packet_id) = packet_id {
        quote! { bf.write_u8(#packet_id.into())?; }
    } else {
        quote! {}
    };

    let mut serialized_snippets = Vec::new();

    match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let Some(ident) = field.ident.as_ref() else {
                        abort!(field.span(), "named fields are required");
                    };

                    serialize_type(quote! { #ident }, &field.ty, &mut serialized_snippets)
                }
            }
            Fields::Unnamed(fields) => {
                for (i, field) in fields.unnamed.iter().enumerate() {
                    let index = Index::from(i);
                    serialize_type(quote! { #index }, &field.ty, &mut serialized_snippets)
                }
            }
            Fields::Unit => {}
        },
        _ => abort!(ast.span(), "only structs are supported"),
    };

    let gen = quote! {
        impl bufferfish::ToBufferfish for #name {
            fn to_bufferfish(&self) -> Result<bufferfish::Bufferfish, bufferfish::BufferfishError> {
                let mut bf = bufferfish::Bufferfish::new();
                #packet_id_serialization
                #(#serialized_snippets)*

                Ok(bf)
            }
        }
    };

    gen.into()
}

fn serialize_type(accessor: TokenStream, ty: &Type, dest: &mut Vec<TokenStream>) {
    match ty {
        Type::Path(TypePath { path, .. }) if path.is_ident("u8") => {
            dest.push(quote! {
                bf.write_u8(self.#accessor)?;
            });
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("u16") => {
            dest.push(quote! {
                bf.write_u16(self.#accessor)?;
            });
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("u32") => {
            dest.push(quote! {
                bf.write_u32(self.#accessor)?;
            });
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("i8") => {
            dest.push(quote! {
                bf.write_i8(self.#accessor)?;
            });
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("i16") => {
            dest.push(quote! {
                bf.write_i16(self.#accessor)?;
            });
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("i32") => {
            dest.push(quote! {
                bf.write_i32(self.#accessor)?;
            });
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("bool") => {
            dest.push(quote! {
                bf.write_bool(self.#accessor)?;
            });
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("String") => {
            dest.push(quote! {
                bf.write_string(&self.#accessor)?;
            });
        }
        Type::Path(TypePath { path, .. })
            if path.segments.len() == 1 && path.segments[0].ident == "Vec" =>
        {
            if let syn::PathArguments::AngleBracketed(args) = &path.segments[0].arguments {
                if let Some(syn::GenericArgument::Type(_)) = args.args.first() {
                    dest.push(quote! {
                        bf.write_u16(self.#accessor.len() as u16)?;
                        for elem in &self.#accessor {
                            elem.encode(&mut bf)?;
                        }
                    });
                } else {
                    abort!(ty.span(), "Vec<T> type not supported");
                }
            } else {
                abort!(ty.span(), "Vec<T> type not supported");
            }
        }
        _ => abort!(ty.span(), "type can not be serialized into a bufferfish"),
    }
}
