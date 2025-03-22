use std::{cell::RefCell, fs, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::from_str;

use super::arguments::IcTestArgs;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IcTestJson {
    pub canisters: Vec<CanisterSetupJson>,
    pub contracts: Vec<ContractSetupJson>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanisterSetupJson {
    pub name: String,
    pub id: String,
    pub wasm: String,
    pub candid_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractSetupJson {
    pub name: String,
    pub sol_json: String,
}

thread_local! {
    static TEST_SETUP : RefCell<IcTestJson> = RefCell::new(IcTestJson::default());
}

pub fn init_test_config(args: &IcTestArgs) -> anyhow::Result<()> {
    let path = Path::new(&args.ic_test_json);

    if !path.exists() {
        // init with default
        return Ok(());
    }

    // try opening config from the ic-test.json
    let json_string = fs::read_to_string(&args.ic_test_json)?;

    let json = from_str::<IcTestJson>(&json_string)?;

    TEST_SETUP.with(|setup| {
        let mut setup = setup.borrow_mut();
        *setup = json;
    });

    Ok(())
}

pub fn store_test_config(args: &IcTestArgs) -> anyhow::Result<()> {
    //

    let to_store = TEST_SETUP.with(|setup| {
        let setup = setup.borrow();
        serde_json::to_string_pretty(&*setup)
    })?;

    fs::write(&args.ic_test_json, to_store)?;

    Ok(())
}
