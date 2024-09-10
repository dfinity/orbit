use crate::{
    setup::{setup_new_env, WALLET_ADMIN_USER},
    test_data::asset::{add_asset, add_asset_with_input, edit_asset_name, remove_asset},
    TestEnv,
};

#[test]
fn asset_lifecycle_test() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create asset
    let asset = add_asset(&env, canister_ids.station, WALLET_ADMIN_USER);

    // edit asset

    edit_asset_name(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        asset.id.clone(),
        "test".to_string(),
    );

    // remove asset

    remove_asset(&env, canister_ids.station, WALLET_ADMIN_USER, asset.id);
}

#[test]
#[should_panic]
fn asset_uniqeness_test() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // assets with the same symbol and blockchain are not allowed

    add_asset_with_input(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        station_api::AddAssetOperationInput {
            name: "asset".to_string(),
            blockchain: "icp".to_string(),
            standards: vec!["native".to_string()],
            metadata: Vec::new(),
            symbol: "SYM".to_string(),
            decimals: 8,
        },
    );

    add_asset_with_input(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        station_api::AddAssetOperationInput {
            name: "asset".to_string(),
            blockchain: "icp".to_string(),
            standards: vec!["native".to_string()],
            metadata: Vec::new(),
            symbol: "SYM".to_string(),
            decimals: 8,
        },
    );
}

#[test]
fn asset_permission_test() {
    // unauthorized users can interact with assets
}

#[test]
fn asset_policy_test() {
    // unauthorized users cant approve asset requests
}
