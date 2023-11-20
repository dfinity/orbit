//! Repositories for entities and related data, including indexes to facilitate data retrieval.

mod user;
use ic_canister_core::repository::IndexRepository;
pub use user::*;

mod account;
pub use account::*;

mod transfer;
pub use transfer::*;

mod notification;
pub use notification::*;

mod proposal;
pub use proposal::*;

pub mod indexes;

pub fn handle_optional_index<R, Index, Key>(
    repository: &R,
    previous: Option<Index>,
    current: Option<Index>,
) where
    R: IndexRepository<Index, Key>,
    Index: Eq,
{
    match (previous, current) {
        (Some(prev), Some(curr)) => {
            if prev != curr {
                repository.remove(&prev);
                repository.insert(curr);
            }
        }
        (Some(prev), None) => {
            repository.remove(&prev);
        }
        (None, Some(curr)) => {
            repository.insert(curr);
        }
        _ => {}
    }
}
