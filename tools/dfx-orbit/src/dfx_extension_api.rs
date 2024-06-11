//! Placeholders for the proposed dfx extension API methods.
use std::process::{Command, Stdio};

use anyhow::Context;

/// Calls the dfx cli.
///
/// Methods are implemented as calls to the dfx cli until a library is available.
fn call_dfx_cli(args: Vec<&str>) -> anyhow::Result<String> {
    let output = Command::new("dfx")
        .args(args)
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        // Execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .with_context(|| "Failed to call dfx.  Is the dfx cli installed?")?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)
            .context("Failed to parse dfx output as UTF-8")?
            .trim()
            .to_string())
    } else {
        Err(anyhow::anyhow!("dfx failed with status: {}", output.status))
    }
}

pub mod identity {
    use super::call_dfx_cli;

    /// The name of the default identity.  This is the identity given by `dfx identity whoami` (if any).
    pub fn default() -> anyhow::Result<String> {
        call_dfx_cli(vec!["identity", "whoami"])
    }
}
