use canbench_rs::{bench, BenchResult};
use ic_canister_core::repository::Repository;
use uuid::Uuid;
use wallet_impl::{
    models::proposal_test_utils::mock_proposal,
    repositories::{ProposalWhereClause, PROPOSAL_REPOSITORY},
};

#[bench]
fn batch_insert_100_proposals() {
    add_proposals_to_repository(100);
}

#[bench(raw)]
fn list_all_proposals() -> BenchResult {
    add_proposals_to_repository(1_000);

    canbench_rs::bench_fn(|| {
        let _ = PROPOSAL_REPOSITORY.list();
    })
}

#[bench(raw)]
fn filter_all_proposals_by_default_filters() -> BenchResult {
    add_proposals_to_repository(1_000);

    canbench_rs::bench_fn(|| {
        let _ = PROPOSAL_REPOSITORY.find_where(
            ProposalWhereClause {
                created_dt_from: None,
                created_dt_to: None,
                expiration_dt_from: None,
                expiration_dt_to: None,
                operation_types: Vec::new(),
                proposers: Vec::new(),
                voters: Vec::new(),
                statuses: Vec::new(),
            },
            None,
        );
    })
}

fn add_proposals_to_repository(count: usize) {
    for _ in 0..count {
        let mut proposal = mock_proposal();
        proposal.id = *Uuid::new_v4().as_bytes();

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());
    }
}
