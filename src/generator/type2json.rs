//! src/ty_json.rs
use std::path::Path;

use anyhow::Error;
use candid::types::{Field, Type, TypeEnv, TypeInner};
use serde_json::{json, Value};
use wf_cdk_bindgen::code_generator;

pub fn type_to_json(ty: &Type, env: &TypeEnv) -> Value {
    let b = TypeVis { env };

    b.visit(ty)
}

struct TypeVis<'a> {
    env: &'a TypeEnv,
}

impl<'a> TypeVis<'a> {
    fn visit(&self, ty: &Type) -> Value {
        match ty.0.as_ref() {
            TypeInner::Bool => json!({ "type": "bool", "default": "false" }),
            TypeInner::Nat
            | TypeInner::Nat8
            | TypeInner::Nat16
            | TypeInner::Nat32
            | TypeInner::Nat64
            | TypeInner::Int
            | TypeInner::Int8
            | TypeInner::Int16
            | TypeInner::Int32
            | TypeInner::Int64 => json!({ "type": "int", "default": 0 }),
            TypeInner::Float32 | TypeInner::Float64 => json!({ "type": "float", "default": 0.0 }),
            TypeInner::Text => json!({ "type": "text", "default": "" }),
            TypeInner::Principal => json!({ "type": "Principal" }),

            TypeInner::Var(name) => {
                let t = self
                    .env
                    .0
                    .get(name)
                    .expect(&format!("Type '{name}' is undefined!"));

                json!({ "type": "var", "type_name": name,
                    "def": self.visit(t)
                })
            }

            TypeInner::Opt(t) => json!({ "type": "opt", "def": self.visit(t), "default" : "None" }),

            TypeInner::Vec(t) => json!({ "type": "vec", "def": self.visit(t), "default" : [] }),

            TypeInner::Record(fs) => {
                json!({
                    "type": "record",
                    "fields": fs.iter().map(|f| field_to_json(self, f)).collect::<Vec<_>>()
                })
            }
            TypeInner::Variant(fs) => {
                let values = fs.iter().map(|f| f.id.to_string()).collect::<Vec<_>>();

                json!({
                    "type": "variant",
                    "values": values,
                    "default": values[0]
                })
            }

            TypeInner::Reserved => unimplemented!(),
            TypeInner::Empty => unimplemented!(),
            TypeInner::Future => unimplemented!(),
            TypeInner::Null => unimplemented!(),
            TypeInner::Knot(_id) => unimplemented!(),
            TypeInner::Unknown => unimplemented!(),
            TypeInner::Func(_fun) => unimplemented!(),
            TypeInner::Service(_meths) => unimplemented!(),
            TypeInner::Class(_tys, _ret) => unimplemented!(),
        }
    }
}

fn field_to_json(v: &TypeVis, f: &Field) -> Value {
    let name = f.ty.to_string();

    let t = v.env.0.get(&name);

    if let Some(t) = t {
        json!({
            "field_name":   f.id.to_string(),
            "type_name":         f.ty.to_string(),
            "def":          v.visit(t),
        })
    } else {
        json!({
            "field_name":   f.id.to_string(),
            "def":          v.visit(&f.ty),
        })
    }
}

pub fn generate_init_args_json(
    candid_path: &Path,
    candid_value_path: &Path,
) -> Result<Vec<Value>, Error> {
    // try parse candid file
    let mut config = code_generator::Config::new();

    config.set_target(code_generator::Target::Builder);

    config.set_service_name("ServiceName".to_owned());

    let (env, actor) = candid_parser::typing::pretty_check_file(candid_path)?;

    // concrete value example

    // parse arguments
    // let text_value = fs::read_to_string(Path::new(value_file))?;

    // use candid::IDLArgs;
    // use candid_parser::parse_idl_args;

    //let args: IDLArgs = parse_idl_args(&text_value)?;

    let mut result = Vec::new();

    if let Some(a) = &actor {
        if let TypeInner::Class(a, _) = a.as_ref() {
            for t in a {
                let json = type_to_json(t, &env);
                result.push(json);
            }
        }
    }

    Ok(result)
}
