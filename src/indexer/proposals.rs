use ethers::prelude::*;
use futures_util::StreamExt;
use std::sync::Arc;
use sqlx::PgPool;

abigen!(
    Governor,
    r#"[  
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
    ]"#
);

/// ENTRY POINT CALLED FROM main.rs
pub async fn sync_all(
    provider: Arc<Provider<Ws>>,
    pool: PgPool,
) {
    let governor_address: Address = "0xYOUR_MAINNET_GOVERNOR_ADDRESS"
        .parse()
        .unwrap();

    index_governor(provider, pool, governor_address).await;
}

async fn index_governor(
    provider: Arc<Provider<Ws>>,
    pool: PgPool,
    address: Address,
) {
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
            proposal.proposal_id,
            proposal.proposer
        );

        save_proposal(&pool, proposal).await;
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
