use std::{collections::HashMap, str::FromStr};

use crate::{
    core::{CallContext, WithCallContext},
    mappers::WalletMapper,
    models::Wallet,
    repositories::WalletRepository,
    transport::{CreateWalletInput, GetWalletInput, WalletDTO},
    types::{ApiError, ApiResult},
};
use ic_canister_core::repository::Repository;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct WalletService {
    // todo: removed if not used by the service
    call_context: CallContext,
    wallet_repository: WalletRepository,
    wallet_mapper: WalletMapper,
}

impl WithCallContext for WalletService {
    fn with_call_context(self, call_context: CallContext) -> Self {
        Self {
            call_context: call_context,
            ..self
        }
    }
}

impl WalletService {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn create_wallet(&self, input: CreateWalletInput) -> ApiResult<WalletDTO> {
        unimplemented!()
    }

    pub async fn get_wallet(&self, input: GetWalletInput) -> ApiResult<WalletDTO> {
        let wallet_id = Uuid::from_str(input.wallet_id.as_str())
            .map_err(|_| ApiError::new("MALFORMED_WALLET_ID".to_string(), None, None))?;
        let wallet_key = Wallet::key(*wallet_id.as_bytes());
        let wallet = self
            .wallet_repository
            .get(&wallet_key)
            .ok_or(ApiError::new("WALLET_NOT_FOUND".to_string(), None, None))?;

        Ok(self.wallet_mapper.to_dto(wallet))
    }
}
