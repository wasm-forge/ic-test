use std::{env, fs, path::PathBuf};

use anyhow::Error;
use ic_cdk_bindgen::code_generator;

use crate::{
    arguments::IcTestArgs,
    dfx_json::{parse_dfx_json, CanisterSetup},
};

use askama::Template;

#[derive(Template)]
#[template(path = "mod.rs.txt")]
struct ModRsTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
}

pub fn generate(args: &IcTestArgs) -> Result<(), Error> {
    let canisters = parse_dfx_json(args)?;

    // current folder
    let mut bindings_path = env::current_dir()?;
    bindings_path.push("tests/bindings");

    fs::create_dir_all(&bindings_path)?;

    generate_mod_rs(&canisters, &bindings_path)?;

    // generate candid files for each canister
    for canister in canisters.iter() {
        if let Some(gen_candid_file) = &canister.gen_candid_file {
            // read candid
            let candid_content = fs::read_to_string(gen_candid_file)?;

            let mut canister_file = bindings_path.clone();
            canister_file.push(format!("{}.rs", &canister.canister_name));

            // try parse candid file
            let mut config = code_generator::Config::new();

            config.set_target(code_generator::Target::Provider);
            config.set_service_name(format!("{}Canister", canister.canister_name));

            let (env, actor) = candid_parser::typing::check_str(&candid_content, true).unwrap();

            let content = ic_cdk_bindgen::code_generator::compile(&config, &env, &actor);

            fs::write(&canister_file, content)
                .unwrap_or_else(|_| panic!("Could not write to file: {}", &canister.canister_name));
            let output = std::process::Command::new("rustfmt")
                .arg(&canister_file)
                .output();
            println!("{:?}", output);
        }
    }

    Ok(())
}

fn generate_mod_rs(canisters: &Vec<CanisterSetup>, bindings_path: &PathBuf) -> Result<(), Error> {
    Ok({
        let mut mod_file: PathBuf = bindings_path.clone();
        mod_file.push("mod.rs");

        let mod_template = ModRsTemplate {
            canisters: canisters,
        };

        let mod_content = mod_template.render()?;

        fs::write(mod_file, mod_content)
            .unwrap_or_else(|_| panic!("Could not create the mod.rs file"));
    })
}
