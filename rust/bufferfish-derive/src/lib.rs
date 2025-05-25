extern crate proc_macro;

use proc_macro_error::{abort, proc_macro_error};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use syn::{
    Data, DataEnum, DeriveInput, Expr, Fields, Index, Type, TypePath, parse_macro_input,
    spanned::Spanned,
};

fn extract_message_id(ast: &DeriveInput) -> Option<Expr> {
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

#[proc_macro_derive(Encode, attributes(bufferfish))]
#[proc_macro_error]
pub fn bufferfish_impl_encodable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let message_id = extract_message_id(&ast);
    let message_id_snippet = {
        if let Some(message_id) = message_id {
            quote! { bf.write_u16(u16::from(#message_id))?; }
        } else {
            quote! {}
        }
    };

    let mut encoded_snippets = Vec::new();

    match &ast.data {
        Data::Struct(data) => {
            encoded_snippets = generate_struct_field_encoders(data);
        }
        Data::Enum(data_enum) => {
            encoded_snippets.push(generate_enum_variant_encoders(name, data_enum));
        }
        Data::Union(_) => abort!(ast.span(), "encoding union types is not supported"),
    };

    let generated = quote! {
        impl bufferfish::Encodable for #name {
            fn encode_value(&self, bf: &mut bufferfish::Bufferfish) -> Result<(), bufferfish::BufferfishError> {
                #(#encoded_snippets)*
                Ok(())
            }

            fn to_bufferfish(&self) -> Result<bufferfish::Bufferfish, bufferfish::BufferfishError> {
                let mut bf = bufferfish::Bufferfish::new();
                self.encode(&mut bf)?;

                Ok(bf)
            }

            fn encode(&self, bf: &mut bufferfish::Bufferfish) -> Result<(), bufferfish::BufferfishError>
            {
                #message_id_snippet
                self.encode_value(bf)
            }
        }
    };

    generated.into()
}

#[proc_macro_derive(Decode, attributes(bufferfish))]
#[proc_macro_error]
pub fn bufferfish_impl_decodable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let message_id = extract_message_id(&ast);
    let has_message_id = message_id.is_some();
    let message_id_snippet = {
        if let Some(message_id) = message_id {
            quote! {
                let message_id = bf.read_u16()?;
                if message_id != u16::from(#message_id) {
                    return Err(bufferfish::BufferfishError::InvalidMessageId);
                }
            }
        } else {
            quote! {}
        }
    };

    let decoded_snippets;
    let min_size_logic;
    let max_size_logic;

    match &ast.data {
        Data::Struct(data_struct) => {
            decoded_snippets = generate_struct_field_decoders(data_struct);
            min_size_logic = generate_struct_min_size_logic(data_struct, has_message_id);
            max_size_logic = generate_struct_max_size_logic(data_struct, has_message_id);
        }
        Data::Enum(data_enum) => {
            decoded_snippets = generate_enum_variant_decoders(data_enum);
            min_size_logic = generate_enum_min_size_logic(data_enum, has_message_id);
            max_size_logic = generate_enum_max_size_logic(data_enum, has_message_id);
        }
        Data::Union(_) => abort!(ast.span(), "unions are not supported"),
    };

    let generated = match &ast.data {
        Data::Struct(data_struct) => {
            let construction = match &data_struct.fields {
                Fields::Named(_) => quote! { Self #decoded_snippets },
                Fields::Unnamed(_) => quote! { Self #decoded_snippets },
                Fields::Unit => quote! { Self {} },
            };
            quote! {
                impl bufferfish::Decodable for #name {
                    fn decode(bf: &mut bufferfish::Bufferfish) -> Result<Self, bufferfish::BufferfishError> {
                        #message_id_snippet
                        Self::decode_value(bf)
                    }

                    fn decode_value(bf: &mut bufferfish::Bufferfish) -> Result<Self, bufferfish::BufferfishError> {
                        Ok(#construction)
                    }

                    fn min_bytes_required() -> Option<usize> {
                        #min_size_logic
                    }

                    fn max_bytes_allowed() -> Option<usize> {
                        #max_size_logic
                    }
                }
            }
        }
        Data::Enum(_) => {
            quote! {
                impl bufferfish::Decodable for #name {
                    fn decode(bf: &mut bufferfish::Bufferfish) -> Result<Self, bufferfish::BufferfishError> {
                        #message_id_snippet
                        Self::decode_value(bf)
                    }

                    fn decode_value(bf: &mut bufferfish::Bufferfish) -> Result<Self, bufferfish::BufferfishError> {
                        let variant_idx = bf.read_u8()?;
                        #decoded_snippets
                    }

                    fn min_bytes_required() -> Option<usize> {
                        #min_size_logic
                    }

                    fn max_bytes_allowed() -> Option<usize> {
                        #max_size_logic
                    }
                }
            }
        }
        _ => abort!(ast.span(), "only structs and enums are supported"),
    };

    generated.into()
}

fn generate_struct_field_encoders(data: &syn::DataStruct) -> Vec<TokenStream> {
    let mut encoded_snippets = Vec::new();

    match &data.fields {
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
    }

    encoded_snippets
}

fn generate_enum_variant_encoders(name: &Ident, data_enum: &DataEnum) -> TokenStream {
    let mut arms = Vec::new();

    for (idx, variant) in data_enum.variants.iter().enumerate() {
        let v_ident = &variant.ident;
        let discrim = Literal::u8_unsuffixed(idx as u8);

        match &variant.fields {
            Fields::Unit => {
                arms.push(quote! {
                    #name::#v_ident => {
                        bf.write_u8(#discrim)?;
                    }
                });
            }
            Fields::Unnamed(fields) => {
                let idents: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| Ident::new(&format!("f{i}"), Span::call_site()))
                    .collect();

                let mut encoders = Vec::new();
                for (i, field) in fields.unnamed.iter().enumerate() {
                    let fld = &idents[i];
                    encode_type(quote! { #fld }, &field.ty, &mut encoders);
                }

                arms.push(quote! {
                    #name::#v_ident( #(#idents),* ) => {
                        bf.write_u8(#discrim)?;
                        #(#encoders)*
                    }
                });
            }
            Fields::Named(fields) => {
                let idents: Vec<Ident> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.clone().unwrap())
                    .collect();

                let mut encoders = Vec::new();
                for (i, field) in fields.named.iter().enumerate() {
                    let fld = &idents[i];
                    encode_type(quote! { #fld }, &field.ty, &mut encoders);
                }

                arms.push(quote! {
                    #name::#v_ident { #(#idents),* } => {
                        bf.write_u8(#discrim)?;
                        #(#encoders)*
                    }
                });
            }
        }
    }

    quote! {
        match self {
            #(#arms),*
        }
    }
}

fn generate_struct_field_decoders(data: &syn::DataStruct) -> TokenStream {
    match &data.fields {
        Fields::Named(fields) => {
            let field_initializers = fields.named.iter().map(|field| {
                let ident = field.ident.as_ref().expect("named fields required");
                let ty = &field.ty;
                quote! { #ident: <#ty as bufferfish::Decodable>::decode_value(bf)?, }
            });
            quote! { { #(#field_initializers)* } }
        }
        Fields::Unnamed(fields) => {
            let field_initializers = fields.unnamed.iter().map(|field| {
                let ty = &field.ty;
                quote! { <#ty as bufferfish::Decodable>::decode_value(bf)?, }
            });
            quote! { ( #(#field_initializers)* ) }
        }
        Fields::Unit => quote! { {} },
    }
}

fn generate_enum_variant_decoders(data_enum: &syn::DataEnum) -> TokenStream {
    let mut arms = Vec::new();

    for (discriminant_value, variant) in data_enum.variants.iter().enumerate() {
        let variant_ident = &variant.ident;
        let discriminant_lit = Index::from(discriminant_value);

        match &variant.fields {
            Fields::Unit => {
                arms.push(quote! { #discriminant_lit => Ok(Self::#variant_ident), });
            }
            Fields::Unnamed(fields) => {
                let mut field_decoders = Vec::new();

                for field in fields.unnamed.iter() {
                    let ty = &field.ty;
                    field_decoders
                        .push(quote! { <#ty as bufferfish::Decodable>::decode_value(bf)? });
                }
                arms.push(quote! {
                    #discriminant_lit => {
                        Ok(Self::#variant_ident( #( #field_decoders ),* ))
                    }
                });
            }
            Fields::Named(fields) => {
                let mut field_decoders = Vec::new();

                for field in fields.named.iter() {
                    let field_ident = field.ident.as_ref().unwrap();
                    let ty = &field.ty;
                    field_decoders.push(
                        quote! { #field_ident: <#ty as bufferfish::Decodable>::decode_value(bf)? },
                    );
                }
                arms.push(quote! {
                    #discriminant_lit => {
                        Ok(Self::#variant_ident { #( #field_decoders ),* })
                    }
                });
            }
        }
    }

    quote! {
        match variant_idx {
            #(#arms)*
            _ => return Err(bufferfish::BufferfishError::InvalidEnumVariant),
        }
    }
}

fn generate_struct_min_size_logic(data: &syn::DataStruct, has_message_id: bool) -> TokenStream {
    let struct_min_field_calcs = match &data.fields {
        Fields::Named(fields) => fields.named.iter().map(|field| {
            let ty = &field.ty;
            quote! { min_size += <#ty as bufferfish::Decodable>::min_bytes_required().unwrap_or(0); }
        }).collect::<Vec<_>>(),
        Fields::Unnamed(fields) => fields.unnamed.iter().map(|field| {
            let ty = &field.ty;
            quote! { min_size += <#ty as bufferfish::Decodable>::min_bytes_required().unwrap_or(0); }
        }).collect::<Vec<_>>(),
        Fields::Unit => Vec::new(),
    };

    quote! {
        let mut min_size = if #has_message_id { 2 } else { 0 };
        #(#struct_min_field_calcs)*
        Some(min_size)
    }
}

fn generate_struct_max_size_logic(data: &syn::DataStruct, has_message_id: bool) -> TokenStream {
    let struct_max_field_calcs = match &data.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| generate_max_size_field_calc(&field.ty))
            .collect::<Vec<_>>(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .map(|field| generate_max_size_field_calc(&field.ty))
            .collect::<Vec<_>>(),
        Fields::Unit => Vec::new(),
    };

    quote! {
        let mut current_max_size: Option<usize> = Some(if #has_message_id { 2 } else { 0 });
        #(#struct_max_field_calcs)*
        current_max_size
    }
}

fn generate_max_size_field_calc(ty: &Type) -> TokenStream {
    quote! {
        current_max_size = current_max_size.and_then(|acc_val| {
            <#ty as bufferfish::Decodable>::max_bytes_allowed().map(|field_m| acc_val + field_m)
        });
    }
}

fn generate_enum_min_size_logic(data: &syn::DataEnum, has_message_id: bool) -> TokenStream {
    let mut variant_min_field_sizes_calcs = Vec::new();

    for variant in data.variants.iter() {
        match &variant.fields {
            Fields::Unit => {
                variant_min_field_sizes_calcs.push(quote! { 0 });
            }
            Fields::Unnamed(fields) => {
                let mut current_variant_min_field_calcs = Vec::new();

                for field in fields.unnamed.iter() {
                    let ty = &field.ty;
                    current_variant_min_field_calcs.push(quote! { <#ty as bufferfish::Decodable>::min_bytes_required().unwrap_or(0) });
                }
                variant_min_field_sizes_calcs
                    .push(quote! { 0 #( + #current_variant_min_field_calcs)* });
            }
            Fields::Named(fields) => {
                let mut current_variant_min_field_calcs = Vec::new();

                for field in fields.named.iter() {
                    let ty = &field.ty;
                    current_variant_min_field_calcs.push(quote! { <#ty as bufferfish::Decodable>::min_bytes_required().unwrap_or(0) });
                }
                variant_min_field_sizes_calcs
                    .push(quote! { 0 #( + #current_variant_min_field_calcs)* });
            }
        }
    }

    quote! {
        let mut min_total_size = if #has_message_id { 2 } else { 0 };
        min_total_size += 1;

        let mut min_variant_fields_contribution = usize::MAX;
        if [#(#variant_min_field_sizes_calcs),*].is_empty() {
            min_variant_fields_contribution = 0;
        } else {
            for size_calc in [#(#variant_min_field_sizes_calcs),*] {
                if size_calc < min_variant_fields_contribution {
                    min_variant_fields_contribution = size_calc;
                }
            }
        }
        if min_variant_fields_contribution == usize::MAX {
             min_variant_fields_contribution = 0;
        }
        min_total_size += min_variant_fields_contribution;
        Some(min_total_size)
    }
}

fn generate_enum_max_size_logic(data: &syn::DataEnum, has_message_id: bool) -> TokenStream {
    let variant_max_field_sizes_calcs: Vec<TokenStream> = data
        .variants
        .iter()
        .map(generate_enum_variant_max_size_calc)
        .collect();

    quote! {
        let mut max_total_size_opt: Option<usize> = Some(if #has_message_id { 2 } else { 0 });

        if let Some(current_max) = max_total_size_opt {
            max_total_size_opt = Some(current_max + 1);
        } else {
            return None;
        }

        let variant_field_max_options = vec![#(#variant_max_field_sizes_calcs),*];
        let mut overall_max_variant_fields_size: Option<usize> = None;

        if variant_field_max_options.is_empty() {
            overall_max_variant_fields_size = Some(0);
        } else {
            let mut current_max_val: Option<usize> = Some(0);
            for opt_size in variant_field_max_options {
                if let Some(size) = opt_size {
                    if let Some(current_m) = current_max_val {
                        if size > current_m { current_max_val = Some(size); }
                    } else { }
                } else {
                    current_max_val = None;
                    break;
                }
            }
            overall_max_variant_fields_size = current_max_val;
        }

        if let Some(current_total_max) = max_total_size_opt {
            if let Some(fields_max) = overall_max_variant_fields_size {
                max_total_size_opt = Some(current_total_max + fields_max);
            } else {
                max_total_size_opt = None;
            }
        }

        max_total_size_opt
    }
}

fn generate_enum_variant_max_size_calc(variant: &syn::Variant) -> TokenStream {
    match &variant.fields {
        Fields::Unit => quote! { Some(0) },
        Fields::Unnamed(fields) => {
            let field_calcs: Vec<_> = fields
                .unnamed
                .iter()
                .map(|f| {
                    let ty = &f.ty;
                    quote! { <#ty as bufferfish::Decodable>::max_bytes_allowed() }
                })
                .collect();
            quote! {{
                let mut acc: Option<usize> = Some(0);
                #(
                    if let Some(sum) = acc {
                        if let Some(val) = #field_calcs {
                            acc = Some(sum + val);
                        } else {
                            acc = None;
                        }
                    }
                )*
                acc
            }}
        }
        Fields::Named(fields) => {
            let field_calcs: Vec<_> = fields
                .named
                .iter()
                .map(|f| {
                    let ty = &f.ty;
                    quote! { <#ty as bufferfish::Decodable>::max_bytes_allowed() }
                })
                .collect();
            quote! {{
                let mut acc: Option<usize> = Some(0);
                #(
                    if let Some(sum) = acc {
                        if let Some(val) = #field_calcs {
                            acc = Some(sum + val);
                        } else {
                            acc = None;
                        }
                    }
                )*
                acc
            }}
        }
    }
}

fn encode_type(accessor: TokenStream, field_type: &Type, dst: &mut Vec<TokenStream>) {
    let effective_type = if let Type::Reference(type_ref) = field_type {
        &*type_ref.elem
    } else {
        field_type
    };

    match effective_type {
        Type::Path(TypePath { path, .. }) if path.is_ident("String") => {
            dst.push(quote! {
                (#accessor).encode(bf)?;
            });
        }
        Type::Path(TypePath { path, .. })
            if path.is_ident("u8")
                || path.is_ident("u16")
                || path.is_ident("u32")
                || path.is_ident("u64")
                || path.is_ident("i8")
                || path.is_ident("i16")
                || path.is_ident("i32")
                || path.is_ident("i64")
                || path.is_ident("bool") =>
        {
            dst.push(quote! {
                (#accessor).encode(bf)?;
            });
        }
        Type::Path(TypePath { path, .. })
            if path.segments.len() == 1 && path.segments[0].ident == "Vec" =>
        {
            dst.push(quote! {
                (#accessor).encode(bf)?;
            });
        }
        Type::Array(_type_array) => {
            dst.push(quote! {
                (#accessor).encode(bf)?;
            });
        }
        Type::Path(TypePath { .. }) => {
            // Catch-all for other user-defined types (structs/enums)
            // These are assumed to implement Encodable.
            dst.push(quote! {
                (#accessor).encode(bf)?;
            });
        }
        _ => abort!(
            effective_type.span(),
            "type cannot be encoded into a bufferfish"
        ),
    }
}
