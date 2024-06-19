//! Implements the dfx extension CLI commands
use crate::args::dfx_extension_api;
use std::io::Read;

/// Implements CLI commands for getting data from the dfx extension API.
pub fn exec(dfx_extension_args: dfx_extension_api::Args) -> anyhow::Result<()> {
    match dfx_extension_args {
        dfx_extension_api::Args::Config(config_args) => match config_args {
            dfx_extension_api::config::Args::Dir => {
                let extension_agent = crate::dfx_extension_api::DfxExtensionAgent::new("orbit");
                let ans = extension_agent.extension_config_dir();
                println!("{ans:?}");
            }
            dfx_extension_api::config::Args::File => {
                let extension_agent = crate::dfx_extension_api::DfxExtensionAgent::new("orbit");
                let mut file = extension_agent
                    .extension_config_file()
                    .expect("Could not open file");
                let mut ans = String::new();
                file.read_to_string(&mut ans).expect("Could not read file");
                // let config: crate::local_config::CommonConfig = serde_json::from_reader(&mut file).unwrap();
                println!("{ans:?}");
            }
        },
    }
    Ok(())
}
