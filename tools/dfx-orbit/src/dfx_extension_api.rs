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
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!(
            "dfx failed with status {}: {stderr}",
            output.status
        ))
    }
}

pub struct DfxExtensionAgent {
    name: String,
}

impl DfxExtensionAgent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Gets the extensions directory, typically at `~/.config/dfx/extensions`
    fn extensions_dir() -> anyhow::Result<cap_std::fs::Dir> {
        let user_config_dir = dfx_core::config::directories::get_user_dfx_config_dir()
            .with_context(|| "Could not find user dfx config dir")?;
        let extensions_dir = user_config_dir.join("extensions");
        std::fs::create_dir_all(&extensions_dir).with_context(|| {
            format!(
                "Could not create directory at: {}",
                extensions_dir.display()
            )
        })?;
        let std_dir = std::fs::File::open(&extensions_dir).with_context(|| {
            format!(
                "Could not open directory at: {}",
                extensions_dir.display()
            )
        })?;
        let cap_dir = cap_std::fs::Dir::from_std_file(std_dir);
        Ok(cap_dir)
    }

    pub fn extension_config_file(&self) -> anyhow::Result<cap_std::fs::File> {
        let extension_config_dir = self.extension_config_dir()?;
        let mut open_options = cap_std::fs::OpenOptions::new();
        let open_options = open_options
            .write(true)
            .create(true);
        extension_config_dir.open_with(format!("{}.json", &self.name), &open_options).with_context(|| {
            format!(
                "Could not create extension config file for extension: {}", &self.name
            )
        })
    }

    /// Gets the extension config directory for this extension.
    pub fn extension_config_dir(&self) -> anyhow::Result<cap_std::fs::Dir> {
        let extensions_dir = Self::extensions_dir()?;
        extensions_dir.create_dir_all(&self.name).with_context(|| {
            format!(
                "Could not create extension directory for extension: {}", &self.name
            )
        })?;
        extensions_dir.open_dir(&self.name).with_context(|| {
            format!(
                "Could not open extension directory for extension: {}", &self.name
            )
        })
    }
}

pub mod identity {
    use super::call_dfx_cli;

    /// The name of the default identity.  This is the identity given by `dfx identity whoami` (if any).
    pub fn default() -> anyhow::Result<String> {
        call_dfx_cli(vec!["identity", "whoami"])
    }
}
