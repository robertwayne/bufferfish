extern crate proc_macro;

use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Expr, Fields, Index, Type, TypePath,
};

#[proc_macro_derive(Encode, attributes(bufferfish))]
#[proc_macro_error]
pub fn bufferfish_impl_encodable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let packet_id = get_packet_id(&ast);
    let packet_id_snippet = {
        if let Some(packet_id) = packet_id {
            quote! { bf.write_u16(u16::from(#packet_id))?; }
        } else {
            quote! {}
        }
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
        Data::Enum(_) => {
            // Enums are just encoded as a u8.
            // TODO: Support any size.
            encoded_snippets.push(quote! {
                bf.write_u8(*self as u8)?;
            });
        }
        Data::Union(_) => abort!(ast.span(), "decoding union types is not supported"),
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

#[proc_macro_derive(Decode, attributes(bufferfish))]
#[proc_macro_error]
pub fn bufferfish_impl_decodable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let packet_id = get_packet_id(&ast);
    let packet_id_snippet = {
        if let Some(packet_id) = packet_id {
            quote! {
                let packet_id = bf.read_u16()?;
                if packet_id != u16::from(#packet_id) {
                    return Err(bufferfish::BufferfishError::InvalidPacketId);
                }
            }
        } else {
            quote! {}
        }
    };

    let decoded_snippets = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| {
                    let ident = field.ident.as_ref().expect("named fields required");
                    let ty = &field.ty;
                    quote! {
                        #ident: <#ty as bufferfish::Decodable>::decode(bf)?,
                    }
                })
                .collect::<Vec<_>>(),
            Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .map(|field| {
                    let ty = &field.ty;
                    quote! {
                        <#ty as bufferfish::Decodable>::decode(bf)?,
                    }
                })
                .collect::<Vec<_>>(),
            Fields::Unit => Vec::new(),
        },
        Data::Enum(data_enum) => data_enum
            .variants
            .iter()
            .enumerate()
            .map(|(i, variant)| {
                let ident = &variant.ident;
                let idx = Index::from(i);
                quote! {
                    #idx => Self::#ident,
                }
            })
            .collect::<Vec<_>>(),
        Data::Union(_) => abort!(ast.span(), "unions are not supported"),
    };

    let gen = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(_) => {
                quote! {
                    impl bufferfish::Decodable for #name {
                        fn decode(bf: &mut bufferfish::Bufferfish) -> Result<Self, bufferfish::BufferfishError> {
                            #packet_id_snippet
                            Ok(Self {
                                #(#decoded_snippets)*
                            })
                        }
                    }
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    impl bufferfish::Decodable for #name {
                        fn decode(bf: &mut bufferfish::Bufferfish) -> Result<Self, bufferfish::BufferfishError> {
                            #packet_id_snippet
                            Ok(Self(
                                #(#decoded_snippets)*
                            ))
                        }
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    impl bufferfish::Decodable for #name {
                        fn decode(bf: &mut bufferfish::Bufferfish) -> Result<Self, bufferfish::BufferfishError> {
                            #packet_id_snippet
                            Ok(Self)
                        }
                    }
                }
            }
        },
        Data::Enum(_) => {
            quote! {
                impl bufferfish::Decodable for #name {
                    fn decode(bf: &mut bufferfish::Bufferfish) -> Result<Self, bufferfish::BufferfishError> {
                        #packet_id_snippet
                        let variant_idx = bf.read_u8()?;
                        Ok(match variant_idx {
                            #(#decoded_snippets)*
                            _ => return Err(bufferfish::BufferfishError::InvalidEnumVariant),
                        })
                    }
                }
            }
        }
        _ => abort!(ast.span(), "only structs and enums are supported"),
    };

    gen.into()
}

fn get_packet_id(ast: &DeriveInput) -> Option<Expr> {
    for attr in &ast.attrs {
        if attr.path().is_ident("bufferfish") {
            if let Ok(expr) = attr.parse_args::<syn::Expr>() {
                return Some(expr);
            } else {
                abort!(attr.span(), "expected a single expression");
            }
        }
    }

    None
}

fn encode_type(accessor: TokenStream, ty: &Type, dst: &mut Vec<TokenStream>) {
    match ty {
        // Handle primitive types
        Type::Path(TypePath { path, .. })
            if path.is_ident("u8")
                || path.is_ident("u16")
                || path.is_ident("u32")
                || path.is_ident("u64")
                || path.is_ident("i8")
                || path.is_ident("i16")
                || path.is_ident("i32")
                || path.is_ident("i64")
                || path.is_ident("bool")
                || path.is_ident("String") =>
        {
            dst.push(quote! {
                bufferfish::Encodable::encode(&#accessor, bf)?;
            });
        }
        // Handle arrays where elements impl Encodable
        Type::Path(TypePath { path, .. })
            if path.segments.len() == 1 && path.segments[0].ident == "Vec" =>
        {
            dst.push(quote! {
                bf.write_array(&#accessor)?;
            });
        }
        // Handle nested structs where fields impl Encodable
        Type::Path(TypePath { .. }) => {
            dst.push(quote! {
                bufferfish::Encodable::encode(&#accessor, bf)?;
            });
        }
        _ => abort!(ty.span(), "type cannot be encoded into a bufferfish"),
    }
}
