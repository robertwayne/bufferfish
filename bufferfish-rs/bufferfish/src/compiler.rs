use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

fn build_typescript_definitions(lines: &[String]) -> Result<String, std::io::Error> {
    let mut current_value = 0;
    let mut variant_lines = Vec::new();
    let mut inside_enum = false;

    for line in lines {
        let line = line.trim().replace(',', "").to_string();

        if line.starts_with('}') {
            break;
        }

        if line.starts_with("pub enum PacketId {") {
            inside_enum = true;
            continue;
        }

        if !inside_enum || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        let explicit_value = parts.get(1);

        let value = match explicit_value {
            Some(value) => value.to_string(),
            None => current_value.to_string(),
        };

        variant_lines.push(format!("    {} = {}", parts[0], value));
        current_value += 1;
    }

    let lines = variant_lines.join(",\n") + ","; // Trailing comma

    let mut contents = String::new();
    contents.push_str("/* AUTOGENERATED BUFFERFISH FILE, DO NOT EDIT */\n");
    contents.push_str("/* Make sure your bundler is configured to inline TypeScript enums in order to avoid bloated codegen from the default TypeScript enum behaviour. */\n");
    contents.push_str("import { Bufferfish } from 'bufferfish';\n\n");
    contents.push_str("export enum PacketId {\n");
    contents.push_str(&lines);
    contents.push_str("\n}\n");

    Ok(contents)
}

fn build_typescript_decoder(lines: &[String]) -> Result<String, std::io::Error> {
    let mut fn_lines = Vec::new();
    let mut interfaces = Vec::new();
    let mut inside_struct = false;
    let mut fields: Vec<(String, String)> = Vec::new();
    let mut struct_name = String::new();

    for line in lines {
        let line = line.trim().replace(',', "").to_string();

        if line.starts_with('}') {
            if inside_struct {
                if !fields.is_empty() {
                    fn_lines.push("\n    return {\n".to_string());
                    for field in &fields {
                        fn_lines.push(format!("        {}: __{},\n", field.0, field.0));
                    }
                    fn_lines.push("    };\n".to_string());
                } else {
                    fn_lines.push("    return {};\n".to_string());
                }
                fn_lines.push("};\n".to_string());

                // Add the interface definition
                let mut interface_lines = Vec::new();
                interface_lines.push(format!("export interface {} {{", struct_name));
                for (field, ts_type) in &fields {
                    interface_lines.push(format!("    {}: {};", field, ts_type));
                }
                interface_lines.push("}\n".to_string());
                interfaces.push(interface_lines.join("\n"));

                inside_struct = false;
                fields.clear();
            }
            continue;
        }

        if line.contains("Encode") {
            inside_struct = true;
            continue;
        }

        if !inside_struct || line.starts_with('#') {
            continue;
        }

        if line.ends_with(';') {
            inside_struct = false;
            continue;
        }

        if line.starts_with("pub struct") {
            struct_name = line.split_whitespace().nth(2).unwrap().to_string();
            fn_lines.push(format!(
                "export const parse{} = (bf: Bufferfish): {} => {{\n",
                struct_name, struct_name
            ));
            continue;
        }

        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        if parts.len() < 2 {
            continue;
        }
        let field_name = parts[0].replace("pub", "").trim().to_string();
        let field_type = parts[1].to_string();

        match field_type.as_str() {
            "u8" => {
                fields.push((field_name.clone(), "number".to_string()));
                fn_lines.push(format!("    const __{} = bf.readUint8();\n", field_name));
            }
            "u16" => {
                fields.push((field_name.clone(), "number".to_string()));
                fn_lines.push(format!("    const __{} = bf.readUint16();\n", field_name));
            }
            "u32" => {
                fields.push((field_name.clone(), "number".to_string()));
                fn_lines.push(format!("    const __{} = bf.readUint32();\n", field_name));
            }
            "i8" => {
                fields.push((field_name.clone(), "number".to_string()));
                fn_lines.push(format!("    const __{} = bf.readInt8();\n", field_name));
            }
            "i16" => {
                fields.push((field_name.clone(), "number".to_string()));
                fn_lines.push(format!("    const __{} = bf.readInt16();\n", field_name));
            }
            "i32" => {
                fields.push((field_name.clone(), "number".to_string()));
                fn_lines.push(format!("    const __{} = bf.readInt32();\n", field_name));
            }
            "bool" => {
                fields.push((field_name.clone(), "boolean".to_string()));
                fn_lines.push(format!("    const __{} = bf.readBool();\n", field_name));
            }
            "String" => {
                fields.push((field_name.clone(), "string".to_string()));
                fn_lines.push(format!("    const __{} = bf.readString();\n", field_name));
            }
            _ if field_type.starts_with("Vec<") => {
                let inner_type = &field_type[4..field_type.len() - 1];
                fields.push((
                    field_name.clone(),
                    format!("{}[]", map_to_ts_type(inner_type)),
                ));
                fn_lines.push(format!(
                    "    const __{} = bf.readArray(() => {});\n",
                    field_name,
                    generate_ts_decoder_call(inner_type)
                ));
            }
            _ => {
                fields.push((field_name.clone(), field_type.clone()));
                fn_lines.push(format!(
                    "    const __{} = {};\n",
                    field_name,
                    generate_ts_decoder_call(&field_type)
                ));
            }
        }
    }

    let mut contents = String::new();

    for interface in interfaces {
        contents.push_str(&interface);
        contents.push('\n');
    }

    for line in fn_lines {
        contents.push_str(&line);
    }

    Ok(contents)
}

fn map_to_ts_type(rust_type: &str) -> &str {
    match rust_type {
        "u8" | "u16" | "u32" | "i8" | "i16" | "i32" => "number",
        "bool" => "boolean",
        "String" => "string",
        _ => rust_type,
    }
}

fn generate_ts_decoder_call(rust_type: &str) -> String {
    match rust_type {
        "u8" => "bf.readUint8()".to_string(),
        "u16" => "bf.readUint16()".to_string(),
        "u32" => "bf.readUint32()".to_string(),
        "i8" => "bf.readInt8()".to_string(),
        "i16" => "bf.readInt16()".to_string(),
        "i32" => "bf.readInt32()".to_string(),
        "bool" => "bf.readBool()".to_string(),
        "String" => "bf.readString()".to_string(),
        _ => format!("parse{}(bf)", rust_type),
    }
}

fn write_typescript_file(dest: &str, contents: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(dest)?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}

fn read_input_file(src: &str) -> Result<Vec<String>, std::io::Error> {
    let input_file = File::open(src).map_err(|e| {
        eprintln!("Failed to open input file: {}", e);
        e
    })?;
    let reader = BufReader::new(input_file);

    let mut contents = Vec::new();
    for line in reader.lines() {
        contents.push(line?);
    }

    Ok(contents)
}

pub fn generate(src: &str, dest: &str) -> Result<(), std::io::Error> {
    let lines = read_input_file(src)?;
    let definitions = build_typescript_definitions(&lines)?;
    let decoder = build_typescript_decoder(&lines)?;

    let content = format!("{}\n{}", definitions, decoder);

    write_typescript_file(dest, &content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ts_generation() {
        let input = r#"
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
        "#;

        let expected_output = r#"/* AUTOGENERATED BUFFERFISH FILE, DO NOT EDIT */
/* Make sure your bundler is configured to inline TypeScript enums in order to avoid bloated codegen from the default TypeScript enum behaviour. */
import { Bufferfish } from 'bufferfish';

export enum PacketId {
    Join = 0,
    Leave = 1,
    Unknown = 255,
}

export interface JoinPacket {
    id: number;
    username: string;
}

export const parseJoinPacket = (bf: Bufferfish): JoinPacket => {
    const __id = bf.readUint8();
    const __username = bf.readString();

    return {
        id: __id,
        username: __username,
    };
};
"#;

        let lines = input
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let definitions = build_typescript_definitions(&lines).unwrap();
        let decoder = build_typescript_decoder(&lines).unwrap();

        let result = format!("{}\n{}", definitions, decoder);

        assert_eq!(result, expected_output);
    }
}