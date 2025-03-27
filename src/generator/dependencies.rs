use std::process::Command;

use super::arguments::IcpTestArgs;

// adding dependencies from the command line
pub fn _add_toml_dependencies(_args: &IcpTestArgs) -> anyhow::Result<()> {
    let _status = Command::new("cargo").arg("add").arg("candid").status()?;

    let _status = Command::new("cargo").arg("add").arg("ic-cdk").status()?;

    Ok(())
}
