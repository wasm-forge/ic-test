mod arguments;
mod dfx_json;
mod generator;

use arguments::IcTestArgs;
use clap::Parser;

async fn process_arguments(args: IcTestArgs) -> anyhow::Result<()> {
    generator::generate(&args).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = IcTestArgs::try_parse()?;
    process_arguments(args).await?;
    Ok(())
}
