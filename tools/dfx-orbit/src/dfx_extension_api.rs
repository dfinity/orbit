//! Placeholders for the proposed dfx extension API methods.
use anyhow::Context;
use candid::Principal;
use dfx_core::interface::dfx::DfxInterface;
use slog::{o, Drain, Logger};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};
use tempfile::tempdir;

/// The name of the Orbit dfx extension.
const ORBIT_EXTENSION_NAME: &str = "orbit";

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
        .with_context(|| "Failed to call dfx. Is the dfx cli installed?")?;

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
pub struct OrbitExtensionAgent {
    /// The directory where all extension configuration files are stored, including those of other extensions.
    extensions_dir: cap_std::fs::Dir,
    /// A logger; some public `sdk` repository methods require a specific type of logger so this is a compatible logger.
    logger: Logger,
}

impl OrbitExtensionAgent {
    /// Creates a new DfxExtensionAgent for the extension with the given name.
    pub fn new() -> anyhow::Result<Self> {
        let dir =
            Self::extensions_dir().with_context(|| "Could not get the dfx extensions directory")?;
        Ok(Self::new_from_dir(dir))
    }

    pub fn new_tmp() -> anyhow::Result<Self> {
        let dir =
            Self::tmp_extensions_dir().with_context(|| "Failed to initialize tmp directry")?;
        Ok(Self::new_from_dir(dir))
    }

    fn new_from_dir(extensions_dir: cap_std::fs::Dir) -> Self {
        let logger = {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = slog_async::Async::new(drain).build().fuse();

            slog::Logger::root(drain, o!())
        };
        Self {
            extensions_dir,
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
        dbg!(&user_config_dir);
        Self::init_extensions_dir(user_config_dir)
    }

    fn tmp_extensions_dir() -> anyhow::Result<cap_std::fs::Dir> {
        // TODO: Come up with a way that cleans up the tempdir after it goes out of scope
        let dir = tempdir()?;
        dbg!(&dir);
        Self::init_extensions_dir(dir.into_path())
    }

    fn init_extensions_dir(path: PathBuf) -> anyhow::Result<cap_std::fs::Dir> {
        let extensions_dir = path.join("extensions");
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
        dbg!(&cap_dir);
        Ok(cap_dir)
    }

    /// Gets the basename of the extension config file.
    fn config_file_name(&self) -> String {
        format!("{}.json", ORBIT_EXTENSION_NAME)
    }

    /// Gets the extension config file for this extension.  If the file does not exist, it will be created.
    ///
    /// E.g. `~/.config/dfx/extensions/<extension_name>.json`
    ///
    /// Note: The file SHOULD be JSON but this is not enforced.
    pub(crate) fn extension_config_file(&self) -> anyhow::Result<cap_std::fs::File> {
        let extension_config_dir = &self.extensions_dir;
        let filename = self.config_file_name();
        let mut open_options = cap_std::fs::OpenOptions::new();
        let open_options = open_options.read(true).write(true).create(true);
        extension_config_dir
            .open_with(filename, open_options)
            .with_context(|| {
                format!(
                    "Could not create extension config file for extension: {}",
                    ORBIT_EXTENSION_NAME
                )
            })
    }

    /// Gets the extension config directory for this extension.
    pub(crate) fn extension_config_dir(&self) -> anyhow::Result<cap_std::fs::Dir> {
        let extensions_dir = &self.extensions_dir;
        extensions_dir
            .create_dir_all(ORBIT_EXTENSION_NAME)
            .with_context(|| {
                format!(
                    "Could not create extension directory for extension: {}",
                    ORBIT_EXTENSION_NAME
                )
            })?;
        extensions_dir
            .open_dir(ORBIT_EXTENSION_NAME)
            .with_context(|| {
                format!(
                    "Could not open extension directory for extension: {}",
                    ORBIT_EXTENSION_NAME
                )
            })
    }

    /// The name of the default dfx user identity.  This is the identity given by `dfx identity whoami` (if any).
    pub fn identity() -> anyhow::Result<String> {
        call_dfx_cli(vec!["identity", "whoami"])
    }

    /// Gets the dfx_core interface
    pub async fn dfx_interface(&mut self) -> anyhow::Result<DfxInterface> {
        let network_name = self
            .station_or_default(None)
            .with_context(|| "Failed to get station")?
            .network;
        let interface_builder = DfxInterface::builder()
            .with_force_fetch_root_key_insecure_non_mainnet_only()
            .with_network_named(&network_name);
        let interface = interface_builder.build().await?;
        if !interface.network_descriptor().is_ic {
            interface.agent().fetch_root_key().await?;
        }
        Ok(interface)
    }

    /// Gets a canister ID
    // TODO: This is a bad API as the two names can be swapped and it will still compile.
    // TODO: Do this without shelling out, using dfx-core only
    pub(crate) fn canister_id(
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
