use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::instrument;

mod agent;

mod commands;
use commands::*;

mod constants;
use constants::{ABOUT, TEMPLATE};

mod utils;
use utils::LONG_VERSION;

#[macro_use]
mod macros;

#[derive(Parser)]
#[clap(
help_template = TEMPLATE,
version,
author,
long_version = LONG_VERSION.as_str(),
about = ABOUT,
long_about = None
)]
#[clap(propagate_version = true)]
pub struct Args {
    /// Sub commands for the application.
    #[clap(subcommand)]
    command: Commands,

    /// Output in JSON format
    #[clap(global = true, long)]
    json: bool,
}

// Generates the commands based on the modules in the commands directory
// Specify the modules you want to include in the commands_enum! macro
commands_enum!(run, completion, docs);

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();

    let cli = Args::parse();
    Commands::exec(cli).await?;

    Ok(())
}

#[cfg(feature = "capture-spantrace")]
fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
