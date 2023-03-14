use anyhow::bail;
use is_terminal::IsTerminal;

use super::*;

use crate::{
    constants::{ABORTED_BY_USER, DOCS_URL, NON_INTERACTIVE_FAILURE},
    interact_or,
    utils::prompt_confirm_with_default};

/// Open hermesd documentation in default browser
#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: Args, _json: bool) -> Result<()> {
    interact_or!(NON_INTERACTIVE_FAILURE);

    let confirm = prompt_confirm_with_default("Open the browser?", true)?;

    if !confirm {
        bail!(ABORTED_BY_USER);
    }

    open::that(DOCS_URL)?;
    Ok(())
}