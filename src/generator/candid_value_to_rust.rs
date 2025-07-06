use candid::pretty::utils::*;
use candid::types::value::IDLField;
use candid::types::{Field, Function, Label, SharedLabel, Type, TypeEnv, TypeInner};
use candid::{IDLArgs, IDLValue};
use candid_parser::bindings::analysis::{chase_actor, infer_rec};
use convert_case::{Case, Casing};
use pretty::RcDoc;
use std::collections::BTreeSet;

type RecPoints<'a> = BTreeSet<&'a str>;
// The definition of tuple is language specific.
pub(crate) fn is_tuple(fs: &[Field]) -> bool {
    if fs.is_empty() {
        return false;
    }
    !fs.iter()
        .enumerate()
        .any(|(i, field)| field.id.get_id() != (i as u32))
}
static KEYWORDS: [&str; 51] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

fn ident_<'a>(id: &'a str, case: Option<Case<'a>>) -> (RcDoc<'a>, bool) {
    if id.is_empty()
        || id.starts_with(|c: char| !c.is_ascii_alphabetic() && c != '_')
        || id.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_')
    {
        return (RcDoc::text(format!("_{}_", candid::idl_hash(id))), true);
    }
    let (is_rename, id) = if let Some(case) = case {
        let new_id = id.to_case(case);
        (new_id != id, new_id)
    } else {
        (false, id.to_owned())
    };
    if ["crate", "self", "super", "Self", "Result", "Principal"].contains(&id.as_str()) {
        (RcDoc::text(format!("{id}_")), true)
    } else if KEYWORDS.contains(&id.as_str()) {
        (RcDoc::text(format!("r#{id}")), is_rename)
    } else {
        (RcDoc::text(id), is_rename)
    }
}

fn ident<'a>(id: &'a str, case: Option<Case<'a>>) -> RcDoc<'a> {
    ident_(id, case).0
}

fn pp_label<'a>(id: &'a SharedLabel, is_variant: bool, vis: &'a str) -> RcDoc<'a> {
    let vis = if vis.is_empty() {
        RcDoc::nil()
    } else {
        kwd(vis)
    };
    match &**id {
        Label::Named(id) => {
            let case = if is_variant { Some(Case::Pascal) } else { None };
            let (doc, is_rename) = ident_(id, case);
            if is_rename {
                str("#[serde(rename=\"")
                    .append(id.escape_debug().to_string())
                    .append("\")]")
                    .append(RcDoc::line())
                    .append(vis)
                    .append(doc)
            } else {
                vis.append(doc)
            }
        }
        Label::Id(n) | Label::Unnamed(n) => vis.append("_").append(RcDoc::as_string(n)).append("_"),
    }
}

fn pp_init_record_fields<'a>(
    prefix: &'a str,
    env: &'a TypeEnv,
    fields: &'a [Field],
    field_values: &'a [IDLField],
    recs: &'a RecPoints,
) -> RcDoc<'a> {
    if is_tuple(fields) {
        let tuple = concat(
            fields.iter().enumerate().map(|(idx, f)| {
                let v = field_values.get(idx).map(|x| &x.val);

                pp_init_arg(prefix, env, &f.ty, v, recs)
            }),
            ",",
        );

        enclose("(", tuple, ")")
    } else {
        let fields = concat(
            fields.iter().map(|f: &Field| {
                let v = field_values
                    .iter()
                    .find(|x: &&IDLField| *f.id == x.id)
                    .map(|x| &x.val);

                pp_init_record_field(prefix, env, f, v, recs)
            }),
            ",",
        );

        enclose_space(" {", fields, "}")
    }
}

fn pp_init_record_field<'a>(
    prefix: &'a str,
    env: &'a TypeEnv,
    field: &'a Field,
    value: Option<&'a IDLValue>,
    recs: &'a RecPoints,
) -> RcDoc<'a> {
    pp_label(&field.id, false, "")
        .append(kwd(":"))
        .append(pp_init_arg(prefix, env, &field.ty, value, recs))
}

fn pp_value_type_error_comment<'a>(arg_type: &'a Type, arg_value: &'a IDLValue) -> RcDoc<'a> {
    enclose(
        "todo!() /* ",
        RcDoc::text(format!(
            "incompatible type value '{arg_value}' for type '{arg_type}'"
        )),
        " */",
    )
}

fn pp_todo(text: &str) -> RcDoc {
    RcDoc::text(format!("todo!() /* {text} */"))
}

fn pp_number(number: i128, suffix: &str) -> RcDoc {
    RcDoc::text(format!("{number}{suffix}"))
}

fn pp_init_arg<'a>(
    prefix: &'a str,
    env: &'a TypeEnv,
    arg_type: &'a Type,
    arg_value: Option<&'a IDLValue>,
    recs: &'a RecPoints,
) -> RcDoc<'a> {
    let arg_value = if let Some(arg_value) = arg_value {
        arg_value
    } else {
        return pp_default_arg(prefix, env, arg_type, recs);
    };

    match arg_type.0.as_ref() {
        TypeInner::Record(fields) => {
            // we expect arg_value to be Record as well
            match arg_value {
                IDLValue::Record(idlfields) => {
                    //
                    pp_init_record_fields(prefix, env, fields, idlfields, recs)
                }
                _ => pp_value_type_error_comment(arg_type, arg_value),
            }
        }
        TypeInner::Var(type_name) => {
            let name = ident(type_name, Some(Case::Pascal));

            let ty = env.0.get(type_name);

            if let Some(ty) = &ty {
                let init = RcDoc::text(format!("{prefix}::"))
                    .append(name)
                    .append("::")
                    .append(pp_init_arg(prefix, env, ty, Some(arg_value), recs));

                if recs.contains(type_name.as_str()) {
                    RcDoc::text("Box::new").append(enclose("(", init, ")"))
                } else {
                    init
                }
            } else {
                enclose("todo!() /* var type '", name, "' not found */")
            }
        }
        TypeInner::Variant(fields) => match arg_value {
            IDLValue::Variant(variant_value) => {
                let mut iter = fields.iter();

                let id = variant_value.0.id.clone();

                let field = iter.find(|x| *x.id == id);

                if let Some(field) = field {
                    let res = RcDoc::text(id.to_string().to_case(Case::Pascal));

                    match field.ty.0.as_ref() {
                        TypeInner::Null => res,
                        _ => res.append(enclose(
                            "(",
                            pp_init_arg(prefix, env, &field.ty, Some(&variant_value.0.val), recs),
                            ")",
                        )),
                    }
                } else {
                    RcDoc::text("todo!()")
                }

                //pp_init_variant_field(env, field.ty, variant_value, recs)
            }
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Vec(ty) => match arg_value {
            IDLValue::Vec(idlvalues) => {
                let items = concat(
                    idlvalues
                        .iter()
                        .map(|v| pp_init_arg(prefix, env, ty, Some(v), recs)),
                    ",",
                );

                enclose("vec![", items, "]")
            }
            // TODO: should we rather treat it as a single element?
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Opt(ty) => match arg_value {
            IDLValue::Null | IDLValue::None => RcDoc::text("None"),
            IDLValue::Opt(opt_val) => enclose(
                "Some(",
                pp_init_arg(prefix, env, ty, Some(opt_val), recs),
                ")",
            ),
            _ => enclose(
                "Some(",
                pp_init_arg(prefix, env, ty, Some(arg_value), recs),
                ")",
            ),
        },
        TypeInner::Text => match arg_value {
            IDLValue::Text(v) => RcDoc::text(format!("\"{v}\".to_string()")),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Bool => match arg_value {
            IDLValue::Bool(b) => RcDoc::text(format!("{b}")),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Principal => RcDoc::text(format!("Principal::from_text({arg_value})")),
        TypeInner::Nat8 => match arg_value {
            IDLValue::Nat8(v) => pp_number(*v as i128, "u8"),
            IDLValue::Nat(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Number(number) => RcDoc::text(number.to_string()),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Nat16 => match arg_value {
            IDLValue::Nat16(v) => pp_number(*v as i128, "u16"),
            IDLValue::Nat(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Number(number) => RcDoc::text(number.to_string()),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Nat32 => match arg_value {
            IDLValue::Nat32(v) => pp_number(*v as i128, "u32"),
            IDLValue::Nat(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Number(number) => RcDoc::text(number.to_string()),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Nat64 => match arg_value {
            IDLValue::Nat64(v) => pp_number(*v as i128, "u64"),
            IDLValue::Nat(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Number(number) => RcDoc::text(number.to_string()),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Int8 => match arg_value {
            IDLValue::Int8(v) => pp_number(*v as i128, "i8"),
            IDLValue::Nat(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Int(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Number(number) => RcDoc::text(number.to_string()),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Int16 => match arg_value {
            IDLValue::Int16(v) => pp_number(*v as i128, "i16"),
            IDLValue::Nat(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Int(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Number(number) => RcDoc::text(number.to_string()),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Int32 => match arg_value {
            IDLValue::Int32(v) => pp_number(*v as i128, "i32"),
            IDLValue::Nat(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Int(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Number(number) => RcDoc::text(number.to_string()),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Int64 => match arg_value {
            IDLValue::Int64(v) => pp_number(*v as i128, "i64"),
            IDLValue::Nat(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Int(v) => {
                let v = v.0.clone().try_into().unwrap();
                pp_number(v, "")
            }
            IDLValue::Number(number) => RcDoc::text(number.to_string()),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Nat => match arg_value {
            IDLValue::Nat(v) => RcDoc::text(format!("candid::Nat::from_str(\"{v}\")")),
            IDLValue::Number(number) => RcDoc::text(format!("candid::Nat::from_str(\"{number})\"")),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Int => match arg_value {
            IDLValue::Int(v) => RcDoc::text(format!("candid::Int::from_str({v})")),
            IDLValue::Number(number) => RcDoc::text(format!("candid::Int::from_str({number})")),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },

        TypeInner::Float32 => match arg_value {
            IDLValue::Float32(v) => RcDoc::text(format!("{v}")),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },
        TypeInner::Float64 => match arg_value {
            IDLValue::Float64(v) => RcDoc::text(format!("{v}")),
            _ => pp_value_type_error_comment(arg_type, arg_value),
        },

        TypeInner::Null => pp_todo("Null"),
        TypeInner::Reserved => pp_todo("Reserved"),
        TypeInner::Empty => pp_todo("Empty"),
        TypeInner::Knot(_) => pp_todo("Knot"),
        TypeInner::Unknown => pp_todo("Unknown"),
        TypeInner::Func(_) => pp_todo("Func"),
        TypeInner::Service(_) => pp_todo("Service"),
        TypeInner::Future => pp_todo("Future"),
        TypeInner::Class(_, _) => pp_todo("Class"),
    }
}

fn pp_init_args<'a>(
    prefix: &'a str,
    env: &'a TypeEnv,
    types: &'a [&Type],
    values: Option<&'a [IDLValue]>,
    recs: &'a RecPoints,
) -> RcDoc<'a> {
    concat(
        types.iter().enumerate().map(|(idx, ty)| {
            let v = values.and_then(|vals| vals.get(idx));
            pp_init_arg(prefix, env, ty, v, recs)
        }),
        ",",
    )
}

pub enum TypePath {
    Id(String),
    Opt,
    Vec,
    RecordField(String),
    VariantField(String),
    Func(String),
    Init,
}

fn path_to_var(path: &[TypePath]) -> String {
    let name: Vec<&str> = path
        .iter()
        .map(|node| match node {
            TypePath::Id(id) => id.as_str(),
            TypePath::RecordField(f) | TypePath::VariantField(f) => f.as_str(),
            TypePath::Opt => "inner",
            TypePath::Vec => "item",
            TypePath::Func(id) => id.as_str(),
            TypePath::Init => "init",
        })
        .collect();
    name.join("_").to_case(Case::Pascal)
}
// Convert structural typing to nominal typing to fit Rust's type system
fn nominalize(env: &mut TypeEnv, path: &mut Vec<TypePath>, t: &Type) -> Type {
    match t.as_ref() {
        TypeInner::Opt(ty) => {
            path.push(TypePath::Opt);
            let ty = nominalize(env, path, ty);
            path.pop();
            TypeInner::Opt(ty)
        }
        TypeInner::Vec(ty) => {
            path.push(TypePath::Vec);
            let ty = nominalize(env, path, ty);
            path.pop();
            TypeInner::Vec(ty)
        }
        TypeInner::Record(fs) => {
            if matches!(
                path.last(),
                None | Some(TypePath::VariantField(_)) | Some(TypePath::Id(_))
            ) || is_tuple(fs)
            {
                let fs: Vec<_> = fs
                    .iter()
                    .map(|Field { id, ty }| {
                        path.push(TypePath::RecordField(id.to_string()));
                        let ty = nominalize(env, path, ty);
                        path.pop();
                        Field { id: id.clone(), ty }
                    })
                    .collect();
                TypeInner::Record(fs)
            } else {
                let new_var = path_to_var(path);
                let ty = nominalize(
                    env,
                    &mut vec![TypePath::Id(new_var.clone())],
                    &TypeInner::Record(fs.to_vec()).into(),
                );
                env.0.insert(new_var.clone(), ty);
                TypeInner::Var(new_var)
            }
        }
        TypeInner::Variant(fs) => match path.last() {
            None | Some(TypePath::Id(_)) => {
                let fs: Vec<_> = fs
                    .iter()
                    .map(|Field { id, ty }| {
                        path.push(TypePath::VariantField(id.to_string()));
                        let ty = nominalize(env, path, ty);
                        path.pop();
                        Field { id: id.clone(), ty }
                    })
                    .collect();
                TypeInner::Variant(fs)
            }
            Some(_) => {
                let new_var = path_to_var(path);
                let ty = nominalize(
                    env,
                    &mut vec![TypePath::Id(new_var.clone())],
                    &TypeInner::Variant(fs.to_vec()).into(),
                );
                env.0.insert(new_var.clone(), ty);
                TypeInner::Var(new_var)
            }
        },
        TypeInner::Func(func) => match path.last() {
            None | Some(TypePath::Id(_)) => {
                let func = func.clone();
                TypeInner::Func(Function {
                    modes: func.modes,
                    args: func
                        .args
                        .into_iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let i = if i == 0 {
                                "".to_string()
                            } else {
                                i.to_string()
                            };
                            path.push(TypePath::Func(format!("arg{i}")));
                            let ty = nominalize(env, path, &ty);
                            path.pop();
                            ty
                        })
                        .collect(),
                    rets: func
                        .rets
                        .into_iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let i = if i == 0 {
                                "".to_string()
                            } else {
                                i.to_string()
                            };
                            path.push(TypePath::Func(format!("ret{i}")));
                            let ty = nominalize(env, path, &ty);
                            path.pop();
                            ty
                        })
                        .collect(),
                })
            }
            Some(_) => {
                let new_var = path_to_var(path);
                let ty = nominalize(
                    env,
                    &mut vec![TypePath::Id(new_var.clone())],
                    &TypeInner::Func(func.clone()).into(),
                );
                env.0.insert(new_var.clone(), ty);
                TypeInner::Var(new_var)
            }
        },
        TypeInner::Service(serv) => match path.last() {
            None | Some(TypePath::Id(_)) => TypeInner::Service(
                serv.iter()
                    .map(|(meth, ty)| {
                        path.push(TypePath::Id(meth.to_string()));
                        let ty = nominalize(env, path, ty);
                        path.pop();
                        (meth.clone(), ty)
                    })
                    .collect(),
            ),
            Some(_) => {
                let new_var = path_to_var(path);
                let ty = nominalize(
                    env,
                    &mut vec![TypePath::Id(new_var.clone())],
                    &TypeInner::Service(serv.clone()).into(),
                );
                env.0.insert(new_var.clone(), ty);
                TypeInner::Var(new_var)
            }
        },
        TypeInner::Class(args, ty) => TypeInner::Class(
            args.iter()
                .map(|ty| {
                    path.push(TypePath::Init);
                    let ty = nominalize(env, path, ty);
                    path.pop();
                    ty
                })
                .collect(),
            nominalize(env, path, ty),
        ),
        _ => return t.clone(),
    }
    .into()
}

fn nominalize_all(env: &TypeEnv, actor: &Option<Type>) -> (TypeEnv, Option<Type>) {
    let mut res = TypeEnv(Default::default());
    for (id, ty) in env.0.iter() {
        let ty = nominalize(&mut res, &mut vec![TypePath::Id(id.clone())], ty);
        res.0.insert(id.to_string(), ty);
    }
    let actor = actor
        .as_ref()
        .map(|ty| nominalize(&mut res, &mut vec![], ty));
    (res, actor)
}

pub fn generate_init_values(
    prefix: &str,
    env: &TypeEnv,
    actor: &Option<Type>,
    values: Option<&IDLArgs>,
) -> String {
    let (env, actor) = nominalize_all(env, actor);
    let def_list: Vec<_> = if let Some(actor) = &actor {
        chase_actor(&env, actor).unwrap()
    } else {
        env.0.iter().map(|pair| pair.0.as_ref()).collect()
    };

    let recs = infer_rec(&env, &def_list).unwrap();

    let mut args = Vec::new();
    if let Some(at) = &actor {
        if let TypeInner::Class(init_args, _) = at.as_ref() {
            for arg_type in init_args {
                args.push(arg_type);
            }
        }
    }

    let defs = pp_init_args(
        prefix,
        &env,
        &args,
        values.map(|v| v.args.as_slice()),
        &recs,
    );

    let doc = RcDoc::line().append(defs);
    doc.pretty(LINE_WIDTH).to_string()
}

fn pp_default_arg<'a>(
    prefix: &'a str,
    env: &'a TypeEnv,
    arg_type: &'a Type,
    recs: &'a RecPoints,
) -> RcDoc<'a> {
    match arg_type.0.as_ref() {
        TypeInner::Record(fields) => pp_default_record_fields(prefix, env, fields, recs),
        TypeInner::Var(type_name) => {
            let name = ident(type_name, Some(Case::Pascal));

            let ty = env.0.get(type_name);

            if let Some(ty) = &ty {
                let init = RcDoc::text(format!("{prefix}::"))
                    .append(name)
                    .append("::")
                    .append(pp_default_arg(prefix, env, ty, recs));

                if recs.contains(type_name.as_str()) {
                    RcDoc::text("Box::new").append(enclose("(", init, ")"))
                } else {
                    init
                }
            } else {
                enclose("todo!() /* var type '", name, "' not found */")
            }
        }
        TypeInner::Variant(_fields) => RcDoc::text("todo!()"),
        TypeInner::Vec(_ty) => RcDoc::text("vec![ todo!() ]"),
        TypeInner::Principal => RcDoc::text("Principal::from_text(todo!())"),
        TypeInner::Opt(_ty) => RcDoc::text("None"),
        _ => RcDoc::text("todo!()"),
    }
}

fn pp_default_record_fields<'a>(
    prefix: &'a str,
    env: &'a TypeEnv,
    fields: &'a [Field],
    recs: &'a BTreeSet<&str>,
) -> RcDoc<'a> {
    if is_tuple(fields) {
        let tuple = concat(
            fields
                .iter()
                //.map(|f| pp_default_arg(prefix, env, &f.ty, recs)),
                .map(|_f| RcDoc::text("todo!()")),
            ",",
        );

        enclose("(", tuple, ")")
    } else {
        let fields = concat(
            fields
                .iter()
                .map(|f: &Field| pp_default_record_field(prefix, env, f, recs)),
            ",",
        );

        enclose_space(" {", fields, "}")
    }
}

fn pp_default_record_field<'a>(
    _prefix: &'a str,
    _env: &'a TypeEnv,
    field: &'a Field,
    _recs: &'a RecPoints,
) -> RcDoc<'a> {
    pp_label(&field.id, false, "")
        .append(kwd(": "))
        //.append(pp_default_arg(prefix, env, &field.ty, recs))
        .append("todo!()")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::candid_value_to_rust;

    fn get_generated_from_path(type_path: &str, value_path: &str) -> String {
        let type_path = Path::new(type_path);
        let value_path = Path::new(value_path);

        let (env, actor) = candid_parser::typing::pretty_check_file(type_path).unwrap();

        let candid_value = std::fs::read_to_string(value_path).unwrap();

        let arg_value = candid_parser::parse_idl_args(&candid_value).unwrap();

        candid_value_to_rust::generate_init_values("prefix", &env, &actor, Some(&arg_value))
    }

    fn get_generated(type_path: &str, candid_value: &str) -> String {
        let type_path = Path::new(type_path);

        let (env, actor) = candid_parser::typing::pretty_check_file(type_path).unwrap();

        if !candid_value.is_empty() {
            let arg_value = candid_parser::parse_idl_args(candid_value).unwrap();

            candid_value_to_rust::generate_init_values("prefix", &env, &actor, Some(&arg_value))
        } else {
            candid_value_to_rust::generate_init_values("prefix", &env, &actor, None)
        }
    }

    #[test]
    fn basic_numeric_value_test() {
        let rust = get_generated_from_path("tests/numbers_type.did", "tests/numbers_value.did");
        println!("{rust}");

        assert!(!rust.contains("todo"));
        assert!(!rust.contains("::::"));
    }

    #[test]
    fn test_todos() {
        let rust = get_generated("tests/candid_types.did", "");
        assert!(rust.contains("todo!"));
        assert!(!rust.contains("::::"));

        println!("{rust}");

        let rust = get_generated(
            "tests/candid_types.did",
            "(variant { variant1 }, variant { tvar2 = variant { other_variant3 } }, record { rec_some = variant { variant2 }; rec_vec_text = vec {\"a\";\"b\"}; rec_opt = variant { variant4 }; rec_vec_opt = vec { variant { variant1 }; variant { variant1 } } })",
        );

        println!("{rust}");

        assert!(!rust.contains("todo!"));
        assert!(!rust.contains("::::"));
    }

    #[test]
    fn test_opt_types() {
        let rust = get_generated("tests/opt_types.did", "(record { rec_vec_opt = vec {} })");

        println!("{rust}");

        assert!(!rust.contains("todo"));

        let rust = get_generated(
            "tests/opt_types.did",
            "(record { rec_some = variant { variant2 }; rec_other = variant { other_variant2 }; rec_opt_vec = opt vec {}; rec_opt = variant { variant4 }; rec_vec_opt = vec { opt variant { variant1 }; opt variant { variant3 } } })",
        );

        println!("{rust}");

        assert!(!rust.contains("todo"));
    }

    #[test]
    fn test_opt_variant() {
        let rust = get_generated(
            "tests/opt_variant.did",
            "(record { rec_opt = variant { variant3 } })",
        );

        println!("{rust}");

        assert!(!rust.contains("todo"));
    }
}
