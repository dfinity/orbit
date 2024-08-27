use crate::setup::setup_new_env;
use crate::TestEnv;
use candid::{Decode, Encode, Principal};
use orbit_essentials::api::{HttpRequest, HttpResponse};
use pocket_ic::{PocketIc, WasmResult};

fn test_candid_decoding_quota(env: &PocketIc, canister_id: Principal) {
    // The anonymous end-user sends a small HTTP request. This should succeed.
    let http_request = HttpRequest {
        method: "GET".to_string(),
        url: "/metrics".to_string(),
        headers: vec![],
        body: vec![42; 1_000],
    };
    let http_request_bytes = Encode!(&http_request).unwrap();
    let response = match env
        .update_call(
            canister_id,
            Principal::anonymous(),
            "http_request",
            http_request_bytes,
        )
        .unwrap()
    {
        WasmResult::Reply(bytes) => Decode!(&bytes, HttpResponse).unwrap(),
        WasmResult::Reject(reason) => panic!("Unexpected reject: {}", reason),
    };
    assert_eq!(response.status_code, 200);

    // The anonymous end-user sends a large HTTP request. This should be rejected.
    let mut large_http_request = http_request;
    large_http_request.body = vec![42; 1_000_000];
    let large_http_request_bytes = Encode!(&large_http_request).unwrap();
    let err = env
        .update_call(
            canister_id,
            Principal::anonymous(),
            "http_request",
            large_http_request_bytes,
        )
        .unwrap_err();
    println!("desc: {}", err.description);
    assert!(err.description.contains("Deserialization Failed"));
}

#[test]
fn test_http_request_deconding_quota() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    test_candid_decoding_quota(&env, canister_ids.station);
}
