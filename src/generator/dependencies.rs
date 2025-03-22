use std::process::Command;

use super::arguments::IcTestArgs;

// adding dependencies from the command line
pub fn _add_toml_dependencies(_args: &IcTestArgs) -> anyhow::Result<()> {
    let _status = Command::new("cargo").arg("add").arg("candid").status()?;

    let _status = Command::new("cargo").arg("add").arg("ic-cdk").status()?;

    Ok(())
}
