//! Account services.
use crate::{
    core::{ApiResult, CallContext},
    mappers::AccountMapper,
    services::AccountService,
    transport::{
        AccountDetailsResponse, AssociateIdentityWithAccountInput,
        AssociateIdentityWithAccountResponse, DeleteAccountResponse, ManageAccountInput,
        ManageAccountResponse, RegisterAccountInput, RegisterAccountResponse,
    },
};
use ic_cdk_macros::{query, update};

#[query(name = "account_details")]
async fn account_details() -> ApiResult<AccountDetailsResponse> {
    let account_details = AccountService::default()
        .get_account_details(&CallContext::get().caller())
        .await?;

    Ok(AccountDetailsResponse { account_details })
}

#[update(name = "register_account")]
async fn register_account(input: RegisterAccountInput) -> ApiResult<RegisterAccountResponse> {
    let ctx = CallContext::get();
    let account = AccountService::default()
        .register_account(&ctx.caller(), &input)
        .await?;
    let account_mapper = AccountMapper::default();

    Ok(RegisterAccountResponse {
        account: account_mapper.map_account_to_account_dto(account),
    })
}

#[update(name = "manage_account")]
async fn manage_account(input: ManageAccountInput) -> ApiResult<ManageAccountResponse> {
    let account_details = AccountService::default()
        .manage_account(&CallContext::get().caller(), &input)
        .await?;

    Ok(ManageAccountResponse { account_details })
}

#[update(name = "associate_identity_with_account")]
async fn associate_identity_with_account(
    input: AssociateIdentityWithAccountInput,
) -> ApiResult<AssociateIdentityWithAccountResponse> {
    let account = AccountService::default()
        .associate_identity_with_account(&CallContext::get().caller(), &input)
        .await?;

    let account_mapper = AccountMapper::default();

    Ok(AssociateIdentityWithAccountResponse {
        account: account_mapper.map_account_to_account_dto(account),
    })
}

#[update(name = "delete_account")]
async fn delete_account() -> ApiResult<DeleteAccountResponse> {
    let deleted_account = AccountService::default()
        .remove_account(&CallContext::get().caller())
        .await?;

    let account_mapper = AccountMapper::default();

    Ok(DeleteAccountResponse {
        account: account_mapper.map_account_to_account_dto(deleted_account),
    })
}
