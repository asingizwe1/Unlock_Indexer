use ethers::prelude::*;
use std::sync::Arc;

mod config;
mod db;
mod indexer;

#[tokio::main]
async fn main() {
    let cfg = config::Config::from_env();

    let pool = db::connect(&cfg.database_url).await;

    let provider = Provider::<Ws>::connect(&cfg.rpc_url).await.unwrap();
    let provider = Arc::new(provider);

    println!("Indexer running...");
}
