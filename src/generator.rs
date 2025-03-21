use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Error;
use ic_cdk_bindgen::code_generator::{self, CustomProvider};

use crate::{
    arguments::IcTestArgs,
    dfx_json::{CanisterSetup, parse_dfx_json},
};

use askama::Template;

pub async fn get_textual_content(path_or_url: &str) -> Result<String, Error> {
    let local_path = Path::new(path_or_url);

    if local_path.exists() && local_path.is_file() {
        // Read from local file
        let contents = fs::read_to_string(local_path)?;
        Ok(contents)
    } else if path_or_url.starts_with("http") {
        // assume it's a URL and fetch via HTTP(S)
        let response = reqwest::get(path_or_url).await?.text().await?;
        Ok(response)
    } else {
        panic!("Failed to read from {path_or_url}");
    }
}

#[derive(Template)]
#[template(path = "mod.rs.txt")]
struct ModRsTemplate<'a> {
    canisters: &'a Vec<CanisterSetup>,
}

struct MyProvider;

pub async fn generate(args: &IcTestArgs) -> Result<(), Error> {
    let canisters = parse_dfx_json(args)?;

    // current folder
    let mut bindings_path = env::current_dir()?;
    bindings_path.push("tests/bindings");

    fs::create_dir_all(&bindings_path)?;

    // prepare mod file
    {
        let mut mod_file: PathBuf = bindings_path.clone();
        mod_file.push("mod.rs");

        let mod_template = ModRsTemplate {
            canisters: &canisters,
        };

        let mod_content = mod_template.render()?;

        fs::write(mod_file, mod_content)
            .unwrap_or_else(|_| panic!("Could not create the mod.rs file"));
    }

    // generate candid files for each canister
    for canister in canisters.iter() {
        if let Some(gen_candid_file) = &canister.gen_candid_file {
            // read candid
            let candid_content = fs::read_to_string(gen_candid_file)?;

            let mut canister_file = bindings_path.clone();
            canister_file.push(format!("{}.rs", &canister.canister_name));

            // try parse candid file
            let conf = code_generator::Config::new();

            // set target to some provider
            //conf.set_target(code_generator::Target::CustomProvider(MyProvider {}));

            let (env, actor) = candid_parser::typing::check_str(&candid_content, true).unwrap();

            let content = ic_cdk_bindgen::code_generator::compile(&conf, &env, &actor);

            fs::write(&canister_file, content)
                .unwrap_or_else(|_| panic!("Could not write to file: {}", &canister.canister_name));
        }
    }

    Ok(())
}
