use std::{
    io::Write,
    process::{Command, Stdio},
};

use convert_case::Casing;
use serde_json::Value;

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
            let default = def["default"]
                .as_str()
                .unwrap()
                .to_case(convert_case::Case::Pascal);

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

fn json_to_rust(value: &Value) -> String {
    let name = value["type_name"]
        .as_str()
        .unwrap_or("")
        .to_case(convert_case::Case::Pascal);

    let def = &value["def"];
    let rust_code = helper(&name, def);

    //format_rust_code(&rust_code).expect("Incorrect rust code generated")

    rust_code
}

pub fn json_values_to_rust(values: Vec<Value>) -> Vec<String> {
    let mut result = Vec::new();

    for json in &values {
        //let pretty = serde_json::to_string_pretty(&json).unwrap();
        let rust = json_to_rust(json);
        result.push(rust);
    }

    result
}
