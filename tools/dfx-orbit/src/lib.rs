//! Library for interacting with Orbit on the Internet Computer.
//#![warn(missing_docs)]
//#![warn(clippy::missing_docs_in_private_items)]
//#![deny(clippy::panic)]
//#![deny(clippy::unwrap_used)]

pub mod args;
pub mod cli;
pub mod dfx_extension_api;
pub mod local_config;
pub mod station_agent;

use anyhow::anyhow;
use candid::Principal;
pub use cli::asset::AssetAgent;
use dfx_core::{
    config::model::{
        canister_id_store::CanisterIdStore,
        dfinity::{Config, ConfigCanistersCanister},
    },
    DfxInterface,
};
use dfx_extension_api::OrbitExtensionAgent;
use ic_utils::{canister::CanisterBuilder, Canister};
use orbit_station_api::CreateRequestResponse;
use slog::Logger;
pub use station_agent::StationAgent;
use station_agent::StationConfig;
use std::sync::Arc;

pub struct DfxOrbit {
    // The station agent that handles communication with the station
    pub station: StationAgent,
    /// The dfx agent.
    pub dfx: OrbitExtensionAgent,
    // The dfx interface
    pub interface: DfxInterface,
    /// A logger; some public `sdk` repository methods require a specific type of logger so this is a compatible logger.
    logger: Logger,
}

impl DfxOrbit {
    /// Creates a new agent for communicating with the default station.
    pub async fn new(
        mut agent: OrbitExtensionAgent,
        config: StationConfig,
        logger: Logger,
    ) -> anyhow::Result<Self> {
        let interface = agent.dfx_interface(&config.network).await?;

        Ok(Self {
            station: StationAgent::new(interface.agent().clone(), config),
            dfx: agent,
            interface,
            logger,
        })
    }

    /// Gets the ID of a given canister name.  If the name is already an ID, it is returned as is.
    pub fn canister_id(&self, canister_name: &str) -> anyhow::Result<Principal> {
        let canister_id_store = CanisterIdStore::new(
            &self.logger,
            self.interface.network_descriptor(),
            self.interface.config(),
        )?;

        let canister_id = Principal::from_text(canister_name)
            .or_else(|_| canister_id_store.get(canister_name))?;

        Ok(canister_id)
    }

    /// Gets the name of the canister given it's id
    pub fn canister_name(&self, canister_id: &Principal) -> anyhow::Result<String> {
        let canister_id_store = CanisterIdStore::new(
            &self.logger,
            self.interface.network_descriptor(),
            self.interface.config(),
        )?;

        let canister_id = canister_id.to_string();
        let canister_name = canister_id_store
            .get_name(&canister_id)
            .cloned()
            .ok_or(anyhow!("Failed to find canister name"))?;

        Ok(canister_name)
    }

    pub fn own_principal(&self) -> anyhow::Result<Principal> {
        self.interface
            .identity()
            .sender()
            .map_err(anyhow::Error::msg)
    }

    pub fn canister_agent(&self, canister_id: Principal) -> anyhow::Result<Canister> {
        Ok(CanisterBuilder::new()
            .with_agent(self.interface.agent())
            .with_canister_id(canister_id)
            .build()?)
    }

    pub fn print_create_request_info(&self, response: &CreateRequestResponse) {
        let request_id = &response.request.id;
        let request_url = self.station.request_url(request_id);
        println!("Created request: {request_id}");
        println!("Request URL: {request_url}");
        println!("To view the request, run: dfx-orbit review id {request_id}");
    }

    pub fn get_config(&self) -> anyhow::Result<Arc<Config>> {
        Ok(self.interface.config().ok_or_else(|| {
            anyhow!("Could not read \"dfx.json\". Are you in the correct directory?")
        })?)
    }

    pub fn get_canister_config(&self, canister: &str) -> anyhow::Result<ConfigCanistersCanister> {
        let config = self.get_config()?;
        let canister_config = config
            .get_config()
            .canisters
            .as_ref()
            .ok_or_else(|| anyhow!("No canisters defined in this \"dfx.json\""))?
            .get(canister)
            .ok_or_else(|| anyhow!("Could not find {canister} in \"dfx.json\""))?;

        Ok(canister_config.clone())
    }
}
