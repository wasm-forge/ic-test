use std::process::Command;

use crate::arguments::IcTestArgs;

// adding dependencies from the command line
pub fn add_toml_dependencies(_args: &IcTestArgs) -> anyhow::Result<()> {
    let _status = Command::new("cargo").arg("add").arg("candid").status()?;

    let _status = Command::new("cargo").arg("add").arg("ic-cdk").status()?;

    Ok(())
}
