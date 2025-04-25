use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Error;
use candid::types::TypeInner;
use pocket_ic::common;
use serde_json::Value;
use wf_cdk_bindgen::code_generator;

use crate::type2json::type_to_json;

fn format_rust_code(code: &str) -> std::io::Result<String> {
    let code = format!("fn m() {{\n{} \n}}", code);

    let mut child = Command::new("rustfmt")
        .arg("--emit=stdout")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(code.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let formatted = String::from_utf8_lossy(&output.stdout).to_string();

    // now remove the first and the last line
    let mut lines: Vec<&str> = formatted.lines().collect();
    if lines.len() <= 2 {
        return Ok("".to_owned());
    }

    lines.remove(0);
    lines.pop();

    Ok(lines.join("\n"))
}

fn helper(class_name: &str, def: &Value) -> String {
    match def.get("type").and_then(|v| v.as_str()) {
        Some("record") => {
            let fields = def["fields"]
                .as_array()
                .unwrap()
                .iter()
                .map(|field| {
                    let field_name = field["field_name"].as_str().unwrap();
                    let field_type_name = field.get("type_name").and_then(|v| v.as_str());
                    let field_def = &field["def"];
                    let field_val = helper(field_type_name.unwrap_or(field_name), field_def);
                    format!("{field_name}: {field_val},")
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("{class_name} {{\n{fields}\n}}")
        }
        Some("variant") => {
            let default = def["default"].as_str().unwrap();
            format!("{class_name}::{default}")
        }
        Some("var") => {
            let default = def["default"].as_str().unwrap();
            format!("{class_name}::{default}")
        }
        Some("text") => {
            let default = def.get("default").and_then(|v| v.as_str()).unwrap_or("");
            format!("\"{}\".to_string()", default)
        }
        Some("int") => def.get("default").unwrap_or(&Value::from(0)).to_string(),
        Some("vec") => {
            let empty_value = Value::Array(vec![]);
            let default = def.get("default").unwrap_or(&empty_value);

            if default.as_array().map(|a| a.is_empty()).unwrap_or(true) {
                "vec![]".to_string()
            } else {
                let inner_def = &def["def"];
                let elements = default
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| helper(class_name, inner_def))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("vec![{elements}]")
            }
        }
        _ => "/* unsupported */".to_string(),
    }
}

pub fn json_to_rust(schema: &Value) -> String {
    let name = schema["type_name"].as_str().unwrap_or("");
    let def = &schema["def"];
    let rust = helper(name, def);

    let format = format_rust_code(&rust).expect("Incorrect rust code generated");

    format
}
