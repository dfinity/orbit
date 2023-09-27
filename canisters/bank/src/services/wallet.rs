use crate::{
    core::{CallContext, WithCallContext},
    errors::WalletError,
    mappers::{HelperMapper, WalletMapper},
    models::Wallet,
    repositories::WalletRepository,
    transport::{CreateWalletInput, GetWalletInput, WalletDTO},
};
use ic_canister_core::{api::ServiceResult, repository::Repository, utils::generate_uuid_v4};

#[derive(Default, Debug)]
pub struct WalletService {
    // todo: removed if not used by the service
    _call_context: CallContext,
    wallet_repository: WalletRepository,
    wallet_mapper: WalletMapper,
    helper_mapper: HelperMapper,
}

impl WithCallContext for WalletService {
    fn with_call_context(self, call_context: CallContext) -> Self {
        Self {
            _call_context: call_context,
            ..self
        }
    }
}

impl WalletService {
    pub fn create() -> Self {
        Default::default()
    }

    pub async fn create_wallet(&self, input: CreateWalletInput) -> ServiceResult<WalletDTO> {
        let uuid = generate_uuid_v4().await;
        let new_wallet = self
            .wallet_mapper
            .new_wallet_from_create_input(input, uuid, vec![])?;
        let key = Wallet::key(*uuid.as_bytes());
        self.wallet_repository.insert(key, new_wallet.clone());

        Ok(self.wallet_mapper.wallet_to_dto(new_wallet))
    }

    pub async fn get_wallet(&self, input: GetWalletInput) -> ServiceResult<WalletDTO> {
        let wallet_id = self.helper_mapper.uuid_from_str(input.wallet_id)?;
        let wallet_key = Wallet::key(*wallet_id.as_bytes());
        let wallet =
            self.wallet_repository
                .get(&wallet_key)
                .ok_or(WalletError::WalletNotFound {
                    id: wallet_id.hyphenated().to_string(),
                })?;

        Ok(self.wallet_mapper.wallet_to_dto(wallet))
    }
}
