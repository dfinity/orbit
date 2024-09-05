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
    assert!(err.description.contains("Decoding cost exceeds the limit"));
}

#[test]
fn test_http_request_decoding_quota() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    test_candid_decoding_quota(&env, canister_ids.station);
    test_candid_decoding_quota(&env, canister_ids.control_panel);
}

fn fetch_asset(canister_id: Principal, port: u16, path: &str, expected: Vec<&str>) {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://{}.localhost:{}{}", canister_id, port, path);
    let res = client.get(url).send().unwrap();
    let page = String::from_utf8(res.bytes().unwrap().to_vec()).unwrap();
    for exp in expected {
        assert!(page.contains(exp));
    }
}

#[test]
fn test_asset_certification() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let port = env.make_live(None).port_or_known_default().unwrap();

    fetch_asset(
        canister_ids.station,
        port,
        "/metrics",
        vec!["# HELP station_total_users The total number of users that are registered, labeled by their status.", "# HELP station_metrics_timestamp UNIX timestamp in nanoseconds when the metrics were exported"],
    );
    fetch_asset(canister_ids.control_panel, port, "/metrics", vec!["# HELP control_panel_active_users Total number of active users in the system, labeled by the time interval."]);
}
