use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Error;
use candid::{types::{Type, TypeInner}, Principal};
use candid_parser::{grammar::IDLInitArgsParser, token::Tokenizer};
use candid::{IDLArgs, TypeEnv};
use candid_parser::parse_idl_args;
use wf_cdk_bindgen::code_generator;

use crate::{
    common::{expand_path, get_path_relative_to_test_dir},
    ic_test_json::{CanisterSetup, ContractSetup, IcpTestSetup},
};

use askama::Template;

#[derive(Template)]
#[template(path = "icp/mod.rs.txt")]
struct ModRsIcpTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
}

#[derive(Template)]
#[template(path = "icp_evm/mod.rs.txt")]
struct ModRsIcpEvmTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
    contracts: &'a Vec<ContractSetup>,
}


pub fn generate_candid_value(candid_path: &str, value_file: &str) -> Result<String, Error> {

    // try parse candid file
    let mut config = code_generator::Config::new();

    config.set_target(code_generator::Target::Builder);

    config.set_service_name("ServiceName".to_owned());


    let (env, actor) =
    candid_parser::typing::pretty_check_file(Path::new(candid_path))?;    

    // parse arguments
    let text_value = fs::read_to_string(Path::new(value_file))?;

    use candid::IDLArgs;
    use candid_parser::parse_idl_args;

    let args: IDLArgs = parse_idl_args(&text_value)?;
    
    println!("args1: {args:?}");
    for arg in &args.args {
        println!("value = {arg:?} \n...\n type={:?}", arg.value_ty());
    }
    println!("");
    println!("...........");
    println!("");

    if let Some(a) = &actor {

        if let TypeInner::Class(a, _) = a.as_ref() {

            // anotate types
            let args2 = args.annotate_types(true, &env, a.as_slice())?;

            for arg in args2.args {
                println!("value = {arg:?} \n...\n type={:?}", arg.value_ty());
            }

        }

    }

    //  content
    Ok("".to_owned())
}


pub fn generate_type(candid_path: &str) -> Result<(), Error> {

    // try parse candid file
    let mut config = code_generator::Config::new();

    config.set_target(code_generator::Target::Builder);

    config.set_service_name("ServiceName".to_owned());

    let canister_file = PathBuf::from("output.rs");

    let (env, actor) =
    candid_parser::typing::pretty_check_file(Path::new(candid_path))?;

    match &actor {
        None => {},
        Some(actor) => {

            let init_args = if let TypeInner::Class(args, _) = actor.as_ref() {
                Some(args)
            } else {
                None
            };

            if let Some(init) = init_args {
                for v in init {

                    match v.as_ref() {
                        TypeInner::Var(name) => {
                            // get var type

                            let t = env.0.get(name);

                            println!("{name}: {t:?},");
                            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!1");

                        },
                        _ => todo!(),
                    }
                }

            }
        
        }
    };

    let content = wf_cdk_bindgen::code_generator::compile(&config, &env, &actor);

    fs::write(&canister_file, content)?;

    Ok(())
}

pub fn generate(setup: &IcpTestSetup) -> Result<(), Error> {
    // current folder
    let bindings_path = env::current_dir()?.join(setup.test_folder.clone()).join("src/bindings");

    fs::create_dir_all(&bindings_path)?;

    generate_mod_rs(setup, &bindings_path)?;

    // generate candid files for each canister
    for (_canister_name, canister) in setup.icp_setup.canisters.iter() {
        if let Some(candid) = &canister.candid {
            // read candid
            let candid_path = expand_path(Path::new(&candid))?;

            let mut canister_file = bindings_path.clone();
            canister_file.push(format!("{}.rs", &canister.var_name));

            // try parse candid file
            let mut config = code_generator::Config::new();

            config.set_target(code_generator::Target::Builder);

            config.set_service_name(canister.service_name.clone());

            if let Some(specified_id) = canister.specified_id.clone() {
                config.set_canister_id(Principal::from_text(&specified_id).unwrap());
            }

            let mut path = PathBuf::new();
            path.push(canister.wasm.clone());

            let path = get_path_relative_to_test_dir(path.as_path(), setup)?;

            config.set_canister_wasm_path(path.to_string_lossy().to_string());

            let (env, actor) =
                candid_parser::typing::pretty_check_file(candid_path.as_path()).unwrap();

            let content = wf_cdk_bindgen::code_generator::compile(&config, &env, &actor);

            fs::write(&canister_file, content)
                .unwrap_or_else(|_| panic!("Could not write to file: {}", &canister.var_name));

            let _output = std::process::Command::new("rustfmt")
                .arg(&canister_file)
                .output()?;
        }
    }

    Ok(())
}

fn generate_mod_rs(setup: &IcpTestSetup, bindings_path: &Path) -> Result<(), Error> {
    let mut mod_file: PathBuf = bindings_path.to_path_buf();
    mod_file.push("mod.rs");

    let canisters: Vec<CanisterSetup> = setup
        .icp_setup
        .canisters
        .iter()
        .map(|x| {
            let mut x = x.1.clone();
            let path = Path::new(&x.wasm);
            let relative = get_path_relative_to_test_dir(path, setup).unwrap();
            x.wasm = relative.to_string_lossy().to_string();
            x
        })
        .collect();

    // are we using EVM template?
    if let Some(evm_setup) = &setup.evm_setup {
        let contracts: Vec<ContractSetup> = evm_setup
            .contracts
            .iter()
            .map(|x| {
                let mut x = x.1.clone();
                let path = Path::new(&x.sol_json);
                let relative = get_path_relative_to_test_dir(path, setup).unwrap();
                x.sol_json = relative.to_string_lossy().to_string();
                x
            })
            .collect();

        let mod_template = ModRsIcpEvmTemplate {
            canisters: &canisters,
            contracts: &contracts,
        };

        let mod_content = mod_template.render()?;

        fs::write(mod_file, mod_content)
            .unwrap_or_else(|_| panic!("Could not create the mod.rs file"));
    } else {
        let mod_template = ModRsIcpTemplate {
            canisters: &canisters,
        };

        let mod_content = mod_template.render()?;

        fs::write(mod_file, mod_content)
            .unwrap_or_else(|_| panic!("Could not create the mod.rs file"));
    }

    Ok(())
}
