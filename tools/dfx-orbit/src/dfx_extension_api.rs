//! Placeholders for the proposed dfx extension API methods.
use std::{
    process::{Command, Stdio},
    str::FromStr,
};

use anyhow::Context;
use candid::Principal;
use dfx_core::interface::dfx::DfxInterface;
use ic_agent::Agent;
use slog::{o, Drain, Logger};

use crate::local_config;

/// Calls the dfx cli.
///
/// Some methods are implemented as calls to the dfx cli until a library is available.
pub fn call_dfx_cli(args: Vec<&str>) -> anyhow::Result<String> {
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

/// The API through which extensions SHOULD interact with ICP networks and dfx configuration.
pub struct DfxExtensionAgent {
    /// The name of the dfx extension.
    name: String,
    /// The directory where all extension configuration files are stored, including those of other extensions.
    extensions_dir: cap_std::fs::Dir,
    /// An interface including an ic-agent.
    dfx_interface: Option<DfxInterface>,
    /// A logger; some public `sdk` repository methods require a specific type of logger so this is a compatible logger.
    logger: Logger,
}

impl DfxExtensionAgent {
    /// Creates a new DfxExtensionAgent for the extension with the given name.
    pub fn new(name: &str) -> Self {
        let logger = {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = slog_async::Async::new(drain).build().fuse();

            slog::Logger::root(drain, o!())
        };
        Self {
            name: name.to_string(),
            extensions_dir: Self::extensions_dir()
                .expect("Could not get the dfx extensions directory"),
            dfx_interface: None,
            logger,
        }
    }

    /// A logger; some public `sdk` repository methods require a specific type of logger so this is a compatible logger.
    pub fn logger(&self) -> &Logger {
        &self.logger
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
            format!("Could not open directory at: {}", extensions_dir.display())
        })?;
        let cap_dir = cap_std::fs::Dir::from_std_file(std_dir);
        Ok(cap_dir)
    }

    /// Gets the basename of the extension config file.
    fn config_file_name(&self) -> String {
        format!("{}.json", &self.name)
    }

    /// Gets the extension config file for this extension.  If the file does not exist, it will be created.
    ///
    /// E.g. `~/.config/dfx/extensions/<extension_name>.json`
    ///
    /// Note: The file SHOULD be JSON but this is not enforced.
    pub fn extension_config_file(&self) -> anyhow::Result<cap_std::fs::File> {
        let extension_config_dir = &self.extensions_dir;
        let filename = self.config_file_name();
        let mut open_options = cap_std::fs::OpenOptions::new();
        let open_options = open_options.read(true).write(true).create(true);
        extension_config_dir
            .open_with(filename, open_options)
            .with_context(|| {
                format!(
                    "Could not create extension config file for extension: {}",
                    &self.name
                )
            })
    }

    /// Gets the extension config directory for this extension.
    pub fn extension_config_dir(&self) -> anyhow::Result<cap_std::fs::Dir> {
        let extensions_dir = &self.extensions_dir;
        extensions_dir.create_dir_all(&self.name).with_context(|| {
            format!(
                "Could not create extension directory for extension: {}",
                &self.name
            )
        })?;
        extensions_dir.open_dir(&self.name).with_context(|| {
            format!(
                "Could not open extension directory for extension: {}",
                &self.name
            )
        })
    }

    /// The name of the default dfx user identity.  This is the identity given by `dfx identity whoami` (if any).
    pub fn identity() -> anyhow::Result<String> {
        call_dfx_cli(vec!["identity", "whoami"])
    }

    /// Gets the dfx_core interface
    pub async fn dfx_interface(&mut self) -> anyhow::Result<&DfxInterface> {
        if self.dfx_interface.is_none() {
            let network_name = local_config::station_or_default(None)
                .with_context(|| "Failed to get station")?
                .network;
            println!("Network name: {}", network_name);
            let interface_builder = DfxInterface::builder().with_network_named(&network_name);
            let interface = interface_builder.build().await?;
            if !interface.network_descriptor().is_ic {
                interface.agent().fetch_root_key().await?;
            }
            self.dfx_interface = Some(interface);
        }
        Ok(self
            .dfx_interface
            .as_ref()
            .expect("Failed to get dfx interface"))
    }

    /// Gets the dfx agent.
    pub async fn agent(&mut self) -> anyhow::Result<&Agent> {
        Ok(self.dfx_interface().await?.agent())
    }

    /// Gets a canister ID
    // TODO: This is a bad API as the two names can be swapped and it will still compile.
    pub fn canister_id(
        &self,
        canister_name: &str,
        network_name: &str,
    ) -> anyhow::Result<Principal> {
        let id = call_dfx_cli(vec![
            "canister",
            "id",
            "--network",
            network_name,
            canister_name,
        ])
        .with_context(|| format!("Failed to look up canister '{canister_name}'"))?;
        Principal::from_str(&id).with_context(|| format!("Could not parse canister ID: {}", id))
    }
}
