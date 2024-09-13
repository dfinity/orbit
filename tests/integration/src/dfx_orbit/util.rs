use candid::Principal;

use super::PORT;

/// Fetches an asset from the local host and port
///
/// This is a bit tricky, as the boundary node uses the `Referer` header to determine the
/// resource being fetched.
pub(super) async fn fetch_asset(canister_id: Principal, path: &str) -> Vec<u8> {
    let port = PORT.with(|port| *port.borrow());
    let local_url = format!("http://localhost:{}{}", port, path);
    let referer = format!("http://localhost:{}?canisterId={}", port, canister_id);

    reqwest::Client::new()
        .get(local_url)
        .header("Referer", referer)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .into()
}
