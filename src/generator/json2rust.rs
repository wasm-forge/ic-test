use std::{
    io::Write,
    process::{Command, Stdio},
};

use convert_case::Casing;
use serde_json::Value;

fn _format_rust_code(code: &str) -> std::io::Result<String> {
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

fn helper(prefix: &str, parent_class_name: &str, value: &Value) -> String {
    match value.get("type").and_then(|v| v.as_str()) {
        Some("record") => {
            let fields = value["fields"]
                .as_array()
                .unwrap()
                .iter()
                .map(|field| {
                    let field_name = field["field_name"].as_str().unwrap();
                    let field_type_name = field
                        .get("type_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or(field_name);
                    let field_def = &field["def"];
                    let field_val = helper(prefix, field_type_name, field_def);
                    format!("{field_name}: {field_val},")
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("{prefix}::{parent_class_name} {{\n{fields}\n}}")
        }
        Some("variant") => {
            if let Some(default) = value.get("default") {
                let default = helper(prefix, "", default);

                format!("{prefix}::{parent_class_name}({default})")
            } else {
                format!("{prefix}::{parent_class_name}")
            }
        }
        Some("opt") => {
            let def = value.get("def");

            if let Some(def) = def {
                let val = helper(prefix, "", def);
                format!("Some({})", val)
            } else {
                "None".to_string()
            }
        }
        Some("var") => {
            let type_name = value["type_name"].as_str().unwrap_or("").to_string();
            let def = value.get("def");

            if let Some(def) = def {
                helper(prefix, &type_name, def)
            } else {
                type_name
            }
        }
        Some("text") => {
            let default = value.get("default").and_then(|v| v.as_str()).unwrap_or("");
            format!("\"{}\".to_string()", default)
        }
        Some("int") => value.get("default").unwrap_or(&Value::from(0)).to_string(),
        Some("vec") => {
            let empty_value = Value::Array(vec![]);
            let default = value.get("default").unwrap_or(&empty_value);

            if default.as_array().map(|a| a.is_empty()).unwrap_or(true) {
                "vec![]".to_string()
            } else {
                let inner_def = &value["def"];
                let elements = default
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|_v| helper(prefix, parent_class_name, inner_def))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("vec![{elements}]")
            }
        }
        Some(t) => {
            format!("/* some {t} */")
        }
        None => "None".to_string(),
    }
}

fn json_to_rust(prefix: &str, value: &Value) -> String {
    helper(prefix, "", value)
}

pub fn json_values_to_rust(prefix: &str, values: Vec<Value>) -> Vec<String> {
    let mut result = Vec::new();

    for json in &values {
        let rust = json_to_rust(prefix, json);
        result.push(rust);
    }

    result
}
