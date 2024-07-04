//! Simple compile for generating TypeScript type definitions, encoders, and
//! decoders from Rust types annotated with `#[derive(Encode)]` and/or `#[derive
//! (Decode]`.

use std::{
    fs::{create_dir_all, read_dir, File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use syn::{
    Attribute, Expr, ExprLit, Fields, GenericArgument, Item, ItemEnum, ItemStruct, Lit, Meta,
    PathArguments, Type, TypePath,
};

/// Generate a TypeScript file at `dest` from the Rust source file at `src`.
/// Requires Rust types to be annotated with `#[derive(Encode)]` and/or
/// `#[derive(Decode]` macros.
pub fn generate(src: &str, dest: &str) -> io::Result<()> {
    let mut files = Vec::new();

    fn visit_dirs(dir: &std::path::Path, files: &mut Vec<String>) -> io::Result<()> {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, files)?;
            } else {
                files.push(path.to_str().unwrap().to_string());
            }
        }
        Ok(())
    }

    visit_dirs(std::path::Path::new(src), &mut files)?;

    let mut output = String::new();
    generate_output_string(files, &mut output)?;
    write_typescript_file(dest, &output)?;

    Ok(())
}

fn write_typescript_file(dest: &str, content: &str) -> io::Result<()> {
    let path = Path::new(dest);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(dest)?;

    file.write_all(content.as_bytes())
}

fn parse_rust_source_file(path: &str) -> io::Result<Vec<Item>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let syntax_tree = syn::parse_file(&content).map_err(|e| {
        eprintln!("Failed to parse Rust source file: {}", e);
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Failed to parse Rust source file",
        )
    })?;

    Ok(syntax_tree.items)
}

fn get_items_implementing_encode(items: Vec<Item>) -> (Vec<ItemStruct>, Vec<ItemEnum>) {
    let mut structs = Vec::new();
    let mut enums = Vec::new();

    let has_encode = |attrs: &[Attribute]| -> bool {
        for attr in attrs {
            if attr.path().is_ident("derive") {
                match &attr.meta {
                    Meta::List(list) => {
                        for item in list.tokens.clone() {
                            if item.to_string().contains("Encode") {
                                return true;
                            }
                        }
                    }
                    _ => return false,
                }
            }
        }

        false
    };

    for item in items {
        match item {
            Item::Struct(item_struct) => {
                if has_encode(&item_struct.attrs) {
                    structs.push(item_struct);
                }
            }
            Item::Enum(item_enum) => {
                if has_encode(&item_enum.attrs) {
                    enums.push(item_enum);
                }
            }
            _ => {}
        }
    }

    (structs, enums)
}

fn generate_output_string(input: Vec<String>, output: &mut String) -> Result<(), std::io::Error> {
    output.push_str("/* AUTOGENERATED BUFFERFISH FILE, DO NOT EDIT */\n");
    output.push_str("import { Bufferfish } from 'bufferfish'\n");

    for path in input {
        let items = parse_rust_source_file(&path)?;
        let (structs, enums) = get_items_implementing_encode(items);

        for item in &enums {
            generate_typescript_enum_defs(item.clone(), output);
            generate_typescript_enum_decoders(item.clone(), output);
        }

        for item in &structs {
            generate_typescript_struct_defs(item.clone(), output);
            generate_typescript_struct_decoders(item.clone(), output);
        }
    }

    Ok(())
}

fn generate_typescript_enum_defs(item: ItemEnum, lines: &mut String) {
    let enum_name = item.ident.to_string();
    let mut variants = Vec::new();
    let mut discriminant = 0;

    for variant in item.variants {
        let variant_name = variant.ident.to_string();
        if let Some((_, expr)) = &variant.discriminant {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) = expr
            {
                discriminant = lit_int.base10_parse().expect("Invalid discriminant value");
            }
        }

        variants.push((variant_name, discriminant));
        discriminant += 1;
    }

    lines.push_str(format!("\nexport enum {} {{\n", enum_name).as_str());
    for (variant_name, discriminant) in variants {
        lines.push_str(format!("    {} = {},\n", variant_name, discriminant).as_str());
    }
    lines.push_str("}\n");
}

fn get_typescript_type(ty: Type) -> String {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            if path.segments.len() == 1 && path.segments[0].ident == "Vec" {
                if let PathArguments::AngleBracketed(ref args) = path.segments[0].arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        let inner_ts_type = get_typescript_type(inner_ty.clone());
                        return format!("Array<{}>", inner_ts_type);
                    }
                }
            }

            match path.get_ident().map(|ident| ident.to_string()).as_deref() {
                Some("u8") | Some("u16") | Some("u32") | Some("i8") | Some("i16") | Some("i32") => {
                    "number".to_string()
                }
                Some("bool") => "boolean".to_string(),
                Some("String") => "string".to_string(),
                _ => path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::"),
            }
        }
        _ => "unknown".to_string(),
    }
}

fn generate_typescript_struct_defs(item: ItemStruct, lines: &mut String) {
    if item.fields.is_empty() {
        return;
    }

    let struct_name = item.ident.to_string();

    match &item.fields {
        Fields::Named(fields_named) => {
            lines.push_str(format!("\nexport interface {} {{\n", struct_name).as_str());
            for field in &fields_named.named {
                if let Some(field_name) = &field.ident {
                    let field_type = get_typescript_type(field.ty.clone());
                    lines.push_str(format!("    {}: {}\n", field_name, field_type).as_str());
                }
            }
            lines.push_str("}\n");
        }
        Fields::Unnamed(fields_unnamed) => {
            lines.push_str(format!("\nexport type {} = [", struct_name).as_str());
            let field_types: Vec<String> = fields_unnamed
                .unnamed
                .iter()
                .map(|f| get_typescript_type(f.ty.clone()))
                .collect();
            lines.push_str(&field_types.join(", "));
            lines.push_str("]\n");
        }
        Fields::Unit => {}
    }
}

fn generate_typescript_struct_decoders(item: ItemStruct, lines: &mut String) {
    let struct_name = item.ident.to_string();

    match &item.fields {
        Fields::Named(fields_named) => {
            lines.push_str(
                format!(
                    "\nexport function decode{}(bf: Bufferfish): {} {{\n",
                    struct_name, struct_name
                )
                .as_str(),
            );
            lines.push_str("    return {\n");

            for field in &fields_named.named {
                if let Some(field_name) = &field.ident {
                    lines.push_str(
                        format!(
                            "        {}: {},\n",
                            field_name,
                            get_bufferfish_fn(field.ty.clone())
                        )
                        .as_str(),
                    );
                }
            }

            lines.push_str("    };\n");
            lines.push_str("}\n");
        }
        Fields::Unnamed(fields_unnamed) => {
            lines.push_str(
                format!(
                    "\nexport function decode{}(bf: Bufferfish): {} {{\n",
                    struct_name, struct_name
                )
                .as_str(),
            );
            lines.push_str("    return [\n");

            for field in &fields_unnamed.unnamed {
                lines.push_str(
                    format!("        {},\n", get_bufferfish_fn(field.ty.clone())).as_str(),
                );
            }

            lines.push_str("    ]\n");
            lines.push_str("}\n");
        }
        Fields::Unit => {}
    }
}

fn get_bufferfish_fn(ty: Type) -> String {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            if path.segments.len() == 1 && path.segments[0].ident == "Vec" {
                if let PathArguments::AngleBracketed(ref args) = path.segments[0].arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        let inner_fn = get_bufferfish_fn(inner_ty.clone());
                        let inner_ts_type = get_typescript_type(inner_ty.clone());
                        return format!(
                            "bf.readArray(() => {}) as Array<{}>",
                            inner_fn, inner_ts_type
                        );
                    }
                }
            }

            match path.get_ident().map(|ident| ident.to_string()).as_deref() {
                Some("u8") => "bf.readUint8() as number".to_string(),
                Some("u16") => "bf.readUint16() as number".to_string(),
                Some("u32") => "bf.readUint32() as number".to_string(),
                Some("i8") => "bf.readInt8() as number".to_string(),
                Some("i16") => "bf.readInt16() as number".to_string(),
                Some("i32") => "bf.readInt32() as number".to_string(),
                Some("bool") => "bf.readBool() as boolean".to_string(),
                Some("String") => "bf.readString() as string".to_string(),
                Some(custom) => format!("decode{}(bf)", custom),
                _ => "unknown".to_string(),
            }
        }
        _ => "unknown".to_string(),
    }
}

fn generate_typescript_enum_decoders(item: ItemEnum, output: &mut String) {
    let enum_name = item.ident.to_string();
    let repr_type = get_repr_type(&item.attrs).unwrap_or("u8".to_string());

    let read_fn = match repr_type.as_str() {
        "u8" => "readUint8",
        "u16" => "readUint16",
        "u32" => "readUint32",
        "i8" => "readInt8",
        "i16" => "readInt16",
        "i32" => "readInt32",
        _ => panic!("Unsupported repr type"),
    };

    output.push_str(
        format!(
            "\nexport function decode{}(bf: Bufferfish): {} {{\n",
            enum_name, enum_name
        )
        .as_str(),
    );
    output.push_str(format!("    return bf.{}() as {}\n", read_fn, enum_name).as_str());
    output.push_str("}\n");
}

fn get_repr_type(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("repr") {
            match &attr.meta {
                Meta::List(list) => {
                    for item in list.tokens.clone() {
                        if let Some(ident) = item.to_string().split_whitespace().next() {
                            return Some(ident.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ts_generation() {
        let input = r#"
#[derive(Encode)]
#[repr(u16)]
pub enum PacketId {
    Join = 0,
    Leave,
    Unknown = 255,
}

#[derive(Encode)]
#[bufferfish(PacketId::Join)]
pub struct JoinPacket {
    pub id: u8,
    pub username: String,
}

#[derive(Encode)]
#[bufferfish(PacketId::Leave)]
pub struct LeavePacket;

#[derive(Encode)]
#[bufferfish(PacketId::Unknown)]
pub struct UnknownPacket(pub u8, pub u16);
        "#;

        let expected_output = r#"/* AUTOGENERATED BUFFERFISH FILE, DO NOT EDIT */
import { Bufferfish } from 'bufferfish'

export enum PacketId {
    Join = 0,
    Leave = 1,
    Unknown = 255,
}

export interface JoinPacket {
    id: number
    username: string
}

export type UnknownPacket = [number, number]
"#;

        let mut output = String::new();
        output.push_str("/* AUTOGENERATED BUFFERFISH FILE, DO NOT EDIT */\n");
        output.push_str("import { Bufferfish } from 'bufferfish'\n");

        let syntax_tree = syn::parse_file(&input).unwrap();
        let (structs, enums) = get_items_implementing_encode(syntax_tree.items);

        for item in enums {
            generate_typescript_enum_defs(item, &mut output);
        }

        for item in structs {
            generate_typescript_struct_defs(item, &mut output);
        }

        assert_eq!(output, expected_output);
    }
}
