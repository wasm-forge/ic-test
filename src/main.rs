mod arguments;
mod dependencies;
mod dfx_json;
mod generator;
mod ic_test_json;

use arguments::IcTestArgs;
use clap::Parser;
use ic_test_json::{init_test_config, store_test_config};

fn process_arguments(args: IcTestArgs) -> anyhow::Result<()> {
    init_test_config(&args)?;

    generator::generate(&args)?;

    store_test_config(&args)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = IcTestArgs::try_parse()?;
    process_arguments(args)?;
    Ok(())
}
