use crate::DfxOrbit;
use anyhow::{bail, Context};
use dfx_core::config::model::dfinity::CanisterTypeProperties;
use std::path::{Path, PathBuf};

impl DfxOrbit {
    pub(super) fn as_path_bufs(
        &self,
        canister: &str,
        paths: &[String],
    ) -> anyhow::Result<Vec<PathBuf>> {
        if paths.is_empty() {
            let canister_config = self.get_canister_config(canister)?;
            let CanisterTypeProperties::Assets { source, .. } = &canister_config.type_specific
            else {
                bail!("Canister {canister} is not an asset canister");
            };
            Ok(source.clone())
        } else {
            Ok(paths.iter().map(|source| PathBuf::from(&source)).collect())
        }
    }

    pub(super) fn as_paths(paths: &[PathBuf]) -> Vec<&Path> {
        paths.iter().map(|pathbuf| pathbuf.as_path()).collect()
    }
}

/// Initalize the logger
///
/// Default log level is WARN, can be turned up to TRCE by adding -v flags
/// and down to CRIT by adding -q flags
pub(super) fn init_logger(verbose: u8, quiet: u8) -> anyhow::Result<slog::Logger> {
    use slog::Drain;

    let verbose = verbose.clamp(0, 3);
    let quiet = quiet.clamp(0, 2);
    let level = 3 + verbose - quiet;
    let level = slog::Level::from_usize(level as usize).with_context(|| "Invalid log level")?;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        .build()
        .filter_level(level)
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    Ok(slog::Logger::root(drain, slog::o!()))
}
