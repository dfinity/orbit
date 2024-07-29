//! A dfx and IC agent for communicating with an Orbit station.

pub use crate::station_agent::{config::StationConfig, error::StationAgentResult};
use candid::CandidType;
use ic_agent::{agent::UpdateBuilder, Agent};
use orbit_station_api::{
    ApiErrorDTO, CreateRequestInput, CreateRequestResponse, GetNextApprovableRequestInput,
    GetNextApprovableRequestResponse, GetRequestInput, GetRequestResponse, ListRequestsInput,
    ListRequestsResponse, MeResponse, SubmitRequestApprovalInput, SubmitRequestApprovalResponse,
};

mod config;
mod error;

/// A dfx agent for communicating with a specific station.
pub struct StationAgent {
    /// The station to communicate with.
    pub config: StationConfig,
    pub agent: Agent,
}

impl StationAgent {
    pub fn new(agent: Agent, config: StationConfig) -> Self {
        Self { config, agent }
    }

    pub async fn request(
        &mut self,
        input: CreateRequestInput,
    ) -> StationAgentResult<CreateRequestResponse> {
        self.update_orbit_typed("create_request", input).await
    }

    pub async fn submit(
        &mut self,
        args: SubmitRequestApprovalInput,
    ) -> StationAgentResult<SubmitRequestApprovalResponse> {
        self.update_orbit_typed("submit_request_approval", args)
            .await
    }

    pub async fn me(&mut self) -> StationAgentResult<MeResponse> {
        self.update_orbit_typed("me", ()).await
    }

    pub async fn review_id(
        &mut self,
        args: GetRequestInput,
    ) -> StationAgentResult<GetRequestResponse> {
        self.update_orbit_typed("get_request", args).await
    }

    pub async fn review_list(
        &mut self,
        args: ListRequestsInput,
    ) -> StationAgentResult<ListRequestsResponse> {
        self.update_orbit_typed("list_requests", args).await
    }

    pub async fn review_next(
        &mut self,
        args: GetNextApprovableRequestInput,
    ) -> StationAgentResult<GetNextApprovableRequestResponse> {
        self.update_orbit_typed("get_next_approvable_request", args)
            .await
    }

    // async fn update_canister_id(
    //     &mut self,
    //     canister_id: &Principal,
    //     method_name: &str,
    // ) -> anyhow::Result<UpdateBuilder> {
    //     Ok(self.agent.update(canister_id, method_name))
    // }

    // pub fn update_canister(
    //     &mut self,
    //     canister: &str,
    //     method_name: &str,
    // ) -> anyhow::Result<UpdateBuilder> {
    //     let canister_id = self.canister_id(canister)?;
    //     Ok(self.agent().update(&canister_id, method_name))
    // }

    async fn update_orbit(&mut self, method_name: &str) -> UpdateBuilder {
        self.agent.update(&self.config.station_id, method_name)
    }

    /// Makes an update call to the station.
    ///
    /// This version integrates candid encoding / decoding
    async fn update_orbit_typed<Req, Res>(
        &mut self,
        method_name: &str,
        request: Req,
    ) -> StationAgentResult<Res>
    where
        Req: CandidType,
        Res: CandidType + for<'a> candid::Deserialize<'a>,
    {
        let encoded_request = candid::encode_one(request)?;

        let response_bytes = self
            .update_orbit(method_name)
            .await
            .with_arg(encoded_request)
            .call_and_wait()
            .await?;
        let ans: Result<Res, ApiErrorDTO> = candid::decode_one(&response_bytes)?;

        Ok(ans?)
    }

    /// The URL for a request in the Orbit UI.
    pub fn request_url(&self, request_id: &str) -> String {
        format!(
            "{}/en/settings/requests?reqid={}",
            self.config.url, request_id
        )
    }
}
