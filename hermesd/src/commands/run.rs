use super::*;
use crate::constants::BIN_NAME;

/// Runs the application and begins reporting metrics.
#[derive(Parser)]
pub struct Args {
    /// Secure token provided by the controller for endpoint check in.
    #[arg(short, long, env = format ! ("{}_TOKEN", BIN_NAME))]
    token: String,
}

pub async fn command(args: Args, _json: bool) -> Result<()> {
    let _token = args.token;
    Ok(())
}
