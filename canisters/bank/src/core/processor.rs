use ic_canister_core::api::ApiError;

pub trait PostProcessor<Err = ApiError> {
    fn post_process(&mut self) -> Result<(), Err>;
}
