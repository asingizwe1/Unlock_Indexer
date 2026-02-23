use ethers::contract::abigen;
use ethers::prelude::*;
use futures_util::StreamExt;

abigen!(Governor, "./abi/governor.json");

pub async fn index_governor(provider: Arc<Provider<Ws>>, address: Address) {
    let governor = Governor::new(address, provider);
    let mut stream = governor
        .event::<ProposalCreatedFilter>()
        .from_block(0u64)
        .stream()
        .await
        .unwrap();

    while let Some(evt) = stream.next().await {
        let p = evt.unwrap();
        println!("Proposal {} created", p.proposal_id);
    }
}
