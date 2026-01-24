use ethers::prelude::*;

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
use futures_util::StreamExt;

pub async fn index_governor(provider: Provider<Ws>, address: Address) {
    let governor = Governor::new(address, provider);

    let mut stream = governor
        .event::<ProposalCreatedFilter>()
        .from_block(0)
        .stream()
        .await
        .unwrap();

    while let Some(evt) = stream.next().await {
        let p = evt.unwrap();
        println!("Proposal {} created", p.proposal_id);
    }
}
