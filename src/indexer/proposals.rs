use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;

use crate::db::save_proposal;

abigen!(
    Governor,
    r#"
    event ProposalCreated(
        uint256 proposalId,
        address proposer,
        uint256 startBlock,
        uint256 endBlock,
        string description
    )

    event VoteCast(
        address voter,
        uint256 proposalId,
        uint8 support,
        uint256 weight
    )
    "#
);

/// ENTRY POINT CALLED FROM main.rs
pub async fn sync_all(provider: Arc<Provider<Ws>>, pool: PgPool) {
    let governor_address: Address = "0x65bA0624403Fc5Ca2b20479e9F626eD4D78E0aD9"
        .parse()
        .unwrap();

    index_governor(provider, pool, governor_address).await;
}

async fn index_governor(provider: Arc<Provider<Ws>>, pool: PgPool, address: Address) {
    let governor = Governor::new(address, provider);

    let mut stream = governor
        .event::<ProposalCreatedFilter>()
        .from_block(0u64)
        .stream()
        .await
        .expect("Failed to create event stream");

    while let Some(evt) = stream.next().await {
        let proposal = evt.expect("Event error");

        println!(
            "Proposal {} by {:?}",
            proposal.proposal_id, proposal.proposer
        );

        save_proposal(
            &pool,
            proposal.proposal_id.as_u64() as i64,
            format!("{:?}", proposal.proposer),
            proposal.start_block.as_u64() as i64,
            proposal.end_block.as_u64() as i64,
            proposal.description,
        )
        .await;
    }
}

//Fetch all proposals that already exist on mainnet and store them.
/*
src/indexer/proposals.rs

Purpose:
Fetch proposals that already exist and store them.

You will:

loop proposal IDs

call state()

save to DB

This alone gives:

proposal count

active vs completed
*/
