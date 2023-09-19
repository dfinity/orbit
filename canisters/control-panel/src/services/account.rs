//! Account services.
use crate::{
    core::ServiceResult,
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
async fn account_info() -> ServiceResult<AccountInfoResponse> {
    println!("account info called");
    unimplemented!()
}

#[candid_method(update)]
#[update(name = "register_account")]
async fn register_account(input: RegisterAccountInput) -> ServiceResult<RegisterAccountResponse> {
    println!("input name = {:?}", input.name);
    println!("input main_bank = {:?}", input.main_bank);
    unimplemented!()
}

#[candid_method(update)]
#[update(name = "manage_account")]
async fn manage_account(input: ManageAccountInput) -> ServiceResult<ManageAccountResponse> {
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
) -> ServiceResult<AssociateIdentityWithAccountResponse> {
    println!(
        "associate_identity_with_account called, {:?}",
        input.account_id
    );
    unimplemented!()
}

#[candid_method(update)]
#[update(name = "delete_account")]
async fn delete_account() -> ServiceResult<DeleteAccountResponse> {
    println!("delete_Account was called");
    unimplemented!()
}
