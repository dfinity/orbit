use crate::{
    transport::{CreateWalletResponse, WalletDTO},
    types::ApiResult,
};
use ic_cdk_macros::update;

#[update(name = "create_wallet")]
async fn create_wallet() -> ApiResult<CreateWalletResponse> {
    Ok(CreateWalletResponse {
        wallet: WalletDTO {
            id: "".to_string(),
            name: None,
            owners: vec![],
            blockchain: "".to_string(),
            policies: vec![],
            standard: None,
            symbol: "".to_string(),
        },
    })
}
