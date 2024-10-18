use candid::{CandidType, Deserialize, Principal};
use futures::future::join_all;
use ic_cdk::api::call::call_raw;
use ic_cdk::api::performance_counter;
use ic_cdk::{id, query, update};

thread_local! {
    static NUMBER: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
}

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

#[derive(CandidType, Deserialize)]
pub enum ValidationResponse {
    Ok(String),
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub struct StoreNumberInput {
    number: u64,
}

#[update]
async fn validate_number(input: StoreNumberInput) -> ValidationResponse {
    if input.number > 100 {
        ValidationResponse::Err("Number is too large".to_string())
    } else {
        ValidationResponse::Ok("Number is valid".to_string())
    }
}

#[update]
async fn store_number(input: StoreNumberInput) {
    NUMBER.with(|n| {
        *n.borrow_mut() = input.number;
    });
}

#[query]
async fn get_number() -> u64 {
    NUMBER.with(|n| *n.borrow())
}

#[update]
async fn noop() {}

#[update]
async fn expensive() {
    loop {
        if performance_counter(0) >= 19_000_000_000 {
            ic_cdk::call::<_, ()>(id(), "noop", ((),)).await.unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_candid_interface() {
        use candid_parser::utils::{service_equal, CandidSource};

        candid::export_service!();
        let new_interface = __export_service();

        service_equal(
            CandidSource::Text(&new_interface),
            CandidSource::Text(include_str!("../../api/spec.did")),
        )
        .unwrap();
    }
}
