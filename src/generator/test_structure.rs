use std::fs;

use anyhow::Error;
use askama::Template;

use crate::{arguments::IcTestArgs, common::get_project_dir, ic_test_json::IcTestSetup};

#[derive(Template)]
#[template(path = "lib.rs.txt")]
struct LibRsTemplate {}

#[derive(Template)]
#[template(path = "tests.rs.txt")]
struct TestsRsTemplate {}

#[derive(Template)]
#[template(path = "Cargo.toml.txt")]
struct CargoTomlTemplate {}

pub fn generate(args: &IcTestArgs, _setup: &IcTestSetup) -> Result<(), Error> {
    let mut project_dir = get_project_dir()?;

    project_dir.push(args.test_folder.clone());

    fs::create_dir_all(&project_dir)?;

    // only create necessary files if they don't exist
    let mut lib_rs = project_dir.clone();
    lib_rs.push("src/lib.rs");

    let mut tests_rs = project_dir.clone();
    tests_rs.push("src/tests.rs");

    let mut cargo_toml = project_dir.clone();
    cargo_toml.push("Cargo.toml");

    if !cargo_toml.exists() {
        let template = CargoTomlTemplate {};

        let content = template.render()?;
        fs::write(cargo_toml, content)
            .unwrap_or_else(|_| panic!("Could not create the Cargo.toml file"));
    }

    if !tests_rs.exists() {
        let template = TestsRsTemplate {};

        let content = template.render()?;
        fs::write(tests_rs, content)
            .unwrap_or_else(|_| panic!("Could not create the tests.rs file"));
    }

    if !lib_rs.exists() {
        let template = LibRsTemplate {};

        let content = template.render()?;
        fs::write(lib_rs, content).unwrap_or_else(|_| panic!("Could not create the lib.rs file"));
    }

    Ok(())
}
