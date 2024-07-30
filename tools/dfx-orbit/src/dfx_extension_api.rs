//! Placeholders for the proposed dfx extension API methods.
use anyhow::Context;
use dfx_core::interface::dfx::DfxInterface;
use slog::{o, Drain, Logger};
use std::path::PathBuf;

/// The name of the Orbit dfx extension.
const ORBIT_EXTENSION_NAME: &str = "orbit";

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
        Self::init_extensions_dir(user_config_dir)
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

    /// Gets the dfx_core interface
    pub async fn dfx_interface(&mut self) -> anyhow::Result<DfxInterface> {
        let network_name = self
            .station_or_default(None)
            .with_context(|| "Failed to get station")?
            .network;
        let interface_builder = DfxInterface::builder().with_network_named(&network_name);
        let interface = interface_builder.build().await?;
        if !interface.network_descriptor().is_ic {
            interface.agent().fetch_root_key().await?;
        }
        Ok(interface)
    }
}
