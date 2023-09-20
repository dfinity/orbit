//! Account services.
use crate::{
    core::ApiResult,
    mappers::AccountMapper,
    services::AccountService,
    transport::{
        AccountInfoResponse, AssociateIdentityWithAccountInput,
        AssociateIdentityWithAccountResponse, DeleteAccountResponse, ManageAccountInput,
        ManageAccountResponse, RegisterAccountInput, RegisterAccountResponse,
    },
};
use candid::candid_method;
use ic_cdk_macros::{query, update};

#[candid_method(query)]
#[query(name = "account_info")]
async fn account_info() -> ApiResult<AccountInfoResponse> {
    println!("account info called");
    unimplemented!()
}

#[candid_method(update)]
#[update(name = "register_account")]
async fn register_account(input: RegisterAccountInput) -> ApiResult<RegisterAccountResponse> {
    let account = AccountService::default().register_account(&input).await?;
    let account_mapper = AccountMapper::default();

    Ok(RegisterAccountResponse {
        account: account_mapper.map_account_to_account_dto(account),
    })
}

#[candid_method(update)]
#[update(name = "manage_account")]
async fn manage_account(input: ManageAccountInput) -> ApiResult<ManageAccountResponse> {
    println!("input name = {:?}", input.name);
    println!("input identities = {:?}", input.identities);
    println!("input use_shared_bank = {:?}", input.use_shared_bank);
    println!("input bank = {:?}", input.bank);
    unimplemented!()
}

#[candid_method(update)]
#[update(name = "associate_identity_with_account")]
async fn associate_identity_with_account(
    input: AssociateIdentityWithAccountInput,
) -> ApiResult<AssociateIdentityWithAccountResponse> {
    println!(
        "associate_identity_with_account called, {:?}",
        input.account_id
    );
    unimplemented!()
}

#[candid_method(update)]
#[update(name = "delete_account")]
async fn delete_account() -> ApiResult<DeleteAccountResponse> {
    println!("delete_Account was called");
    unimplemented!()
}
