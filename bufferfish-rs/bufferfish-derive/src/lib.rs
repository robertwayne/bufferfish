extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Type, TypePath};

#[proc_macro_derive(Serialize)]
#[proc_macro_error]
pub fn bufferfish_serializer(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut serialize_snippets = Vec::new();

    match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let ident = field.ident.as_ref().unwrap();
                    let ty = &field.ty;

                    match ty {
                        Type::Path(TypePath { path, .. }) if path.is_ident("u8") => {
                            serialize_snippets.push(quote! {
                                bf.write_u8(self.#ident)?;
                            });
                        }
                        Type::Path(TypePath { path, .. }) if path.is_ident("u16") => {
                            serialize_snippets.push(quote! {
                                bf.write_u16(self.#ident)?;
                            });
                        }
                        Type::Path(TypePath { path, .. }) if path.is_ident("u32") => {
                            serialize_snippets.push(quote! {
                                bf.write_u32(self.#ident)?;
                            });
                        }
                        Type::Path(TypePath { path, .. }) if path.is_ident("i8") => {
                            serialize_snippets.push(quote! {
                                bf.write_i8(self.#ident)?;
                            });
                        }
                        Type::Path(TypePath { path, .. }) if path.is_ident("i16") => {
                            serialize_snippets.push(quote! {
                                bf.write_i16(self.#ident)?;
                            });
                        }
                        Type::Path(TypePath { path, .. }) if path.is_ident("i32") => {
                            serialize_snippets.push(quote! {
                                bf.write_i32(self.#ident)?;
                            });
                        }
                        Type::Path(TypePath { path, .. }) if path.is_ident("bool") => {
                            serialize_snippets.push(quote! {
                                bf.write_bool(self.#ident)?;
                            });
                        }
                        Type::Path(TypePath { path, .. }) if path.is_ident("String") => {
                            serialize_snippets.push(quote! {
                                bf.write_string(&self.#ident)?;
                            });
                        }
                        _ => abort!(ty.span(), "type can not be serialized into a bufferfish"),
                    }
                }
            }
            _ => abort!(data.fields.span(), "only named fields are supported"),
        },
        _ => abort!(ast.span(), "only structs are supported"),
    };

    let gen = quote! {
        impl bufferfish::BufferfishWrite for #name {
            fn write(&self) -> Result<bufferfish::Bufferfish, bufferfish::BufferfishError> {
                let mut bf = bufferfish::Bufferfish::new();
                #(#serialize_snippets)*

                Ok(bf)
            }
        }
    };

    gen.into()
}
