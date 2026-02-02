use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use std::sync::Arc;
mod config;
mod db;
mod indexer;

#[tokio::main]
async fn main() {
    let cfg = config::Config::from_env();

    let pool = db::connect(&cfg.database_url).await;

    let provider = Provider::<Ws>::connect(&cfg.rpc_url)
        .await
        .expect("Failed to connect to RPC");
    let provider = Arc::new(provider);

    println!("Indexer running...");

    indexer::proposals::sync_all(provider.clone(), pool.clone()).await;
}

// Your indexing layer:

// Reads blockchain events

// Calls view functions

// Normalizes data

// Stores it in DB

// Exposes clean APIs

// ///;;;;
// use chrono::Utc;
// use ethers::prelude::*;
// use ethers::providers::{Provider, StreamExt, Ws};
// use std::sync::Arc;

// mod config;
// mod db;
// mod indexer;

// #[tokio::main]
// async fn main() {
//     let cfg = config::Config::from_env();

//     let pool = db::connect(&cfg.database_url).await;

//     // Connect to WebSocket RPC
//     let ws = Ws::connect(&cfg.rpc_url)
//         .await
//         .expect("Failed to connect to WebSocket RPC");
//     let provider = Arc::new(Provider::new(ws));

//     println!("Indexer running...");

//     // Spawn a task to log new blocks
//     let provider_blocks = provider.clone();
//     tokio::spawn(async move {
//         let mut stream = provider_blocks.subscribe_blocks().await.unwrap();
//         while let Some(block) = stream.next().await {
//             println!("📦 New block: {:?}", block.number);
//         }
//     });

//     // Heartbeat ticker every 5 seconds
//     loop {
//         println!("💡 Indexer alive at {:?}", Utc::now());
//         tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//     }
// }

/*
Contracts ❌ don’t give you:

history

analytics

aggregates

charts

DB is mandatory.
*/

/*
WHERE DATA COMES FROM

PROPOSALS
Events to index:
ProposalCreated
ProposalExecuted
ProposalCanceled
VoteCast

Contract calls:
state(proposalId)
proposalSnapshot(proposalId)
proposalDeadline(proposalId)
quorum(blockNumber)

DB table:
proposals (
  proposal_id,
  proposer,
  state,
  start_block,
  end_block,
  for_votes,
  against_votes,
  abstain_votes,
  executed,
  description
)

🟣 ACTIVE / COMPLETED PROPOSALS

Computed by:

Governor::state(proposal_id)


States:

Pending

Active

Defeated (quorum not reached)

Succeeded

Queued

Executed

👉 NOT stored on-chain as lists, you compute them.

🟣 TREASURY HOLDINGS ($478k)

From:

Timelock contract balance

ERC20 balances (SUP token)

Indexer job:

provider.get_balance(timelock, None)
token.balance_of(timelock)


Stored in DB:

treasury_snapshots (
  timestamp,
  eth_balance,
  sup_balance
)

🟣 DELEGATION DATA (graph + counts)

Events:

DelegateChanged
DelegateVotesChanged


Indexer tracks:

who delegates to who

voting power changes

DB:

delegations (
  delegator,
  delegatee,
  voting_power,
  timestamp
)


From this you compute:

Total delegations

Self-delegation %

Other delegation %

Delegation graph (nodes + edges)

🟣 ELIGIBLE VOTERS

From:

getVotes(address)


Eligible = votes > 0

You count distinct holders with voting power.

6️⃣ Your delegation graph (image you sent)

This is NOT on-chain.

Your indexer builds it:

{
  "nodes": [
    { "id": "you", "power": 50000 },
    { "id": "cryptosmonitor.eth", "power": 120000 }
  ],
  "edges": [
    { "from": "you", "to": "cryptosmonitor.eth" }
  ]
}


Frontend renders via:

D3.js

React Force Graph

Vis.js

7️⃣ Where the DB fits (VERY IMPORTANT)

Contracts ❌ don’t give you:

history

analytics

aggregates

charts

DB is mandatory.
*/
