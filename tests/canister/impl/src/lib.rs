use candid::Principal;
use futures::future::join_all;
use ic_cdk::api::call::call_raw;
use ic_cdk::update;

#[update]
async fn call(canister_id: Principal, method_name: String, arg: Vec<u8>, mut total_calls: u64) {
    while total_calls != 0 {
        let current_batch = std::cmp::min(total_calls, 500);
        total_calls -= current_batch;
        let mut futs = vec![];
        for _ in 0..current_batch {
            futs.push(call_raw(canister_id, &method_name, arg.clone(), 0));
        }
        let res = join_all(futs).await;
        for r in res {
            r.unwrap();
        }
    }
}
