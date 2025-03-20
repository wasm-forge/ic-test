mod arguments;
mod generated;
mod generator;

use std::fs;

use arguments::IcTestArgs;
use clap::Parser;
use serde_json::from_str;

fn parse_dfx_json(args: &mut IcTestArgs) -> anyhow::Result<()> {
    let json_string = fs::read_to_string(&args.dfx_json)?;

    let json = from_str::<generated::dfx_json::DfxJson>(&json_string)?;

    let canisters = json.canisters;

    println!("{:?}", canisters);
    Ok(())
}

fn process_arguments(mut args: IcTestArgs) -> anyhow::Result<()> {
    parse_dfx_json(&mut args)?;

    generator::generate(&args)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = IcTestArgs::try_parse()?;
    process_arguments(args)?;
    Ok(())
}
